//! # todo
//!
//! create components for camera and sprite.
//!
//! for 3d stuff, take a look at:  https://www.nalgebra.org/docs/user_guide/cg_recipes

use legion::{World, Resources, Schedule, Entity, system};
use nalgebra::{
    Isometry3,
    Perspective3,
    Point2,
    Point3,
    Similarity3,
    Vector2,
    Vector3,
};
use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::{
        ControlFlow,
        EventLoop,
    },
    window::{
        Window,
        WindowBuilder,
    },
};

use crate::{
    config::Config,
    error::Error,
    graphics::Graphics,
};

/// dimension of the object.
pub enum Dimension {
    /// a.k.a. wonderworld. this is the imagined world inside the player's head.
    InnerWorld,

    /// a.k.a. reality. the physical game world. represents shared reality.
    OuterWorld,
}

/// a model's transform relative to the parent
/// 
/// todo: should we just add the parent entity to this?
#[derive(Clone, Copy, Debug)]
pub struct LocalTransform(pub Similarity3<f32>);

impl Default for LocalTransform {
    fn default() -> Self {
        Self(Similarity3::new(
            nalgebra::zero(),
            nalgebra::zero(),
            nalgebra::one(),
        ))
    }
}

impl LocalTransform {
    pub fn new(x: impl Into<Similarity3<f32>>) -> Self {
        Self(x.into())
    }

    /// todo: call this from a system to update the global transform. or remove
    /// this method and move the code altogether.
    pub fn to_global<'a>(
        self,
        parent_transform: impl Into<Option<&'a GlobalTransform>>,
    ) -> GlobalTransform {
        if let Some(parent_transform) = parent_transform.into() {
            GlobalTransform(&parent_transform.0 * &self.0)
        }
        else {
            GlobalTransform(self.0)
        }
    }
}

/// a model's global transform
#[derive(Clone, Copy, Debug)]
pub struct GlobalTransform(pub Similarity3<f32>);

impl GlobalTransform {
    pub fn new(similarity: impl Into<Similarity3<f32>>) -> Self {
        Self(similarity.into())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Parent {
    // todo: entity id of parent. we'll use this when we compute the absolute transform for an
    // entity.
}

/// position in the world
#[derive(Clone, Copy, Debug)]
pub struct Position(pub Point2<f32>);

/// orientation of entity. the vector points up.
#[derive(Clone, Copy, Debug)]
pub struct Orientation(pub Vector2<f32>);

/// velocity of an object. this will be added to the position in every tick
/// (asjusted for framerate)
#[derive(Clone, Copy, Debug)]
pub struct Velocity(pub Vector2<f32>);

#[derive(Clone, Copy, Debug)]
pub struct Acceleration(pub Vector2<f32>);

/// camera component. the camera entity will also need a position and
/// orientation component (or use defaults). we can compute.
///
/// todo: we want projection and not orthographic, right? with perspective we
/// can have parallex with the background.
pub struct Camera {
    camera_projection: Perspective3<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 9.0, std::f32::consts::FRAC_PI_2, 1.0, 1000.0)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
        // our camera looks toward the point (0.0, 0.0, 0.0).
        // it is located at (0.0, 0.0, 1.0).
        let eye = Point3::from([0.0, 0.0, 1.0]); // todo: this is the global transform of the camera entity

        let target = Point3::from([0.0, 0.0, 0.0]);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        // a perspective projection.
        // todo: use the aspect ratio from the window
        let camera_projection = Perspective3::new(aspect_ratio, fovy, znear, zfar);

        Self { camera_projection }
    }
}

/// this defines which camera is used for rendering. this also currently restricts stuff to rendering to exactly one surface, which is not strictly necessary.
/// 
/// the dimension attached to the camera will determine which entities are endered.
#[derive(Copy, Clone, Debug)]
pub struct CurrentCamera {
    entity: Entity,
}

/// component that hides entities.
pub struct Hidden;

/// todo: something better than a plain index would be nice.
pub struct Sprite {
    sprite_index: usize,
}

pub struct Game {
    /// game config
    pub config: Config,

    /// the winit event loop that delivers window events.
    ///
    /// todo: there's docs about making this work across threads.
    pub event_loop: EventLoop<()>,

    /// the winit window. this is not graphics specific, because it also gives
    /// us input events. although we could probably move it too, since the
    /// events are received through the event loop.
    pub window: Window,

    // todo: put graphics stuff in here too.
    pub graphics: Graphics,

    /// the game world that contains all entities
    pub world: World,

    /// global world resources, such as the currently selected camera.
    /// 
    /// todo: do we want to put the graphics, etc. in there?
    pub resources: Resources,

    /// the system schedule. this runs systems that do work on entities.
    pub schedule: Schedule,
}

impl Game {
    pub async fn new() -> Result<Self, Error> {
        // todo: we need to remember how we loaded this (e.g. when we have config file
        // path), so that we can also store it todo: separate stuff the game
        // remembers (i.e. properties) and user config (that is not written back and
        // keeps comments, is a toml file, etc.).
        let config = Config::load().await.expect("config error");

        // create window and even loop
        // todo: put this stuff into a resource.
        let window_size = config.graphics.physical_size();
        log::debug!("window_size = {:?}", window_size);

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_min_inner_size(window_size)
            .build(&event_loop)
            .unwrap();
        log::debug!("window id: {:?}", window.id());

        #[cfg(target_arch = "wasm32")]
        {
            // winit prevents sizing with css, so we have to set
            // the size manually when on web.
            use winit::platform::web::WindowExtWebSys;

            window.set_inner_size(window_size);

            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("root")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        // create graphics
        let graphics = Graphics::new(&window).await?;

        // create a test world
        let mut world = World::default();

        let model_entity = world.push((Dimension::OuterWorld, LocalTransform::default()));
        let camera_entity = world.push((
            Dimension::OuterWorld,
            LocalTransform::default(),
            Camera::new(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0),
        ));

        // create resources, such as the current camera.
        //
        // todo: it would make sense to have the sprite sheet stuff in a resource.
        let mut resources = Resources::default();
        resources.insert(CurrentCamera { entity: camera_entity });

        let schedule = Schedule::builder()
            .add_system(update_local_transforms_system())
            .flush()
            .add_system(render_sprites_system())
            .build();

        Ok(Self {
            config,
            event_loop,
            window,
            graphics,
            world,
            resources,
            schedule,
        })
    }

    pub fn run(mut self) -> Result<(), Error> {
        self.event_loop.run(move |event, _, control_flow| {
            // todo: for now we can keep it this way. but i think we want to just send
            // messages to systems.
            *control_flow = ControlFlow::Wait;
            log::trace!("event: {:?}", event);
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::KeyboardInput { input, .. } => {
                            log::debug!("keyboard input: {:?}", input);
                        }
                        WindowEvent::ModifiersChanged(modifiers_state) => {
                            log::debug!("modifiers changed: {:?}", modifiers_state);
                        }
                        WindowEvent::Resized(size) => {
                            log::info!("resized: {:?}", size);

                            // reconfigure the surface with the new size
                            self.config.graphics.width = size.width;
                            self.config.graphics.height = size.height;
                            self.graphics
                                .surface
                                .configure(&self.graphics.device, &self.graphics.config);
                        }
                        WindowEvent::CloseRequested => {
                            log::info!("close requested");
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(_) => {
                    log::debug!("render frame");

                    // todo: remove this once we render in the system
                    self.graphics.render().expect("failed to render frame");
                
                    // do we want to run everything only when a redraw is requested?
                    //self.schedule.execute(&mut self.world, &mut self.resources);
                }
                _ => {}
            }
        });
    }
}


/// updates global transforms.
/// 
#[system(for_each)]
fn update_local_transforms(global_transform: &mut GlobalTransform, local_transform: &LocalTransform, parent: &Option<Parent>) {
    log::debug!("system running: update local transforms");
}

/// system for rendering sprites from the global sprite sheet.
/// 
/// todo: move this into graphics. but we can refactor stuff out of the graphics object first.
#[system(for_each)]
fn render_sprites(sprite: &mut Sprite, dimension: &Dimension, global_transform: &GlobalTransform, #[resource] current_camera: &CurrentCamera) {
    log::debug!("system running: render sprites");

    // todo
}
