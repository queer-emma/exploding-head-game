//! # todo
//!
//! create components for camera and sprite.
//!
//! for 3d stuff, take a look at:  https://www.nalgebra.org/docs/user_guide/cg_recipes

pub mod components;
pub mod resources;
pub mod systems;

use legion::{
    system,
    Resources,
    Schedule,
    World,
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
    game::{
        components::{
            Camera,
            Dimension,
            GlobalTransform,
            LocalTransform,
            Parent,
            Sprite,
        },
        resources::{
            CurrentCamera,
            Time,
        },
    },
    graphics::Graphics,
};

pub struct Game {
    /// the winit event loop that delivers window events.
    ///
    /// note: the generic type is a user-define event, which we don't need
    /// afaik.
    pub event_loop: EventLoop<()>,

    /// the winit window. this is not graphics specific, because it also gives
    /// us input events. although we could probably move it too, since the
    /// events are received through the event loop.
    ///
    /// # note
    ///
    /// this needs to be kept somewhere, otherwise it's dropped and destroyed,
    /// and nothing will be rendered anymore.
    pub window: Window,

    // note: moved into resources.
    // todo: put graphics stuff in here too.
    //pub graphics: Graphics,
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

        // this needs to be done in the main thread[1].
        //
        // [1] https://docs.rs/winit/latest/winit/event_loop/struct.EventLoop.html#method.new
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
        // a single sprite
        let model_entity = world.push((
            Dimension::OuterWorld,
            LocalTransform::default(),
            Sprite::new(0),
        ));
        let camera_entity = world.push((
            Dimension::OuterWorld,
            LocalTransform::default(),
            Camera::new(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0),
        ));

        // fill resources
        let mut resources = Resources::default();
        // timing (fps)
        resources.insert(Time::default());
        // the game configuration
        resources.insert(config);
        // graphics
        resources.insert(graphics);
        // the one and only camera fow now.
        // todo: a camera is kind of associated with a surface and view.
        resources.insert(CurrentCamera {
            entity: camera_entity,
        });

        let schedule = Schedule::builder()
            .add_system(update_local_transforms_system())
            .flush()
            //.add_system(render_sprites_system())
            .add_system(render_graphics_system())
            .build();

        Ok(Self {
            event_loop,
            world,
            resources,
            schedule,
            window,
        })
    }

    pub fn run(mut self) -> ! {
        // note: the run method never terminates. `ControlFlow::Exit` will exit the
        // process.
        self.event_loop.run(move |event, _, control_flow| {
            // todo: somehow we need to know when to set `control_flow = ControlFlow::Exit`.
            *control_flow = ControlFlow::Wait;
            log::trace!("event: {:?}", event);

            // todo: dispatch events to systems.
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
                            /*self.config.graphics.width = size.width;
                            self.config.graphics.height = size.height;
                            self.graphics
                                .surface
                                .configure(&self.graphics.device, &self.graphics.config);*/
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

                    // update timing information
                    /*{
                        let mut time = self.resources.get_mut::<Time>().unwrap();
                        time.update();
                    }*/

                    // do we want to run everything only when a redraw is requested?
                    self.schedule.execute(&mut self.world, &mut self.resources);
                }
                _ => {}
            }
        });
    }
}

/// updates global transforms.
#[system(for_each)]
fn update_local_transforms(
    global_transform: &mut GlobalTransform,
    local_transform: &LocalTransform,
    parent: &Option<Parent>,
) {
    log::debug!("system running: update local transforms");
}

/// system for rendering sprites from the global sprite sheet.
///
/// todo: move this into graphics. but we can refactor stuff out of the graphics
/// object first.
#[system(for_each)]
fn render_sprites(
    sprite: &mut Sprite,
    dimension: &Dimension,
    global_transform: &GlobalTransform,
    #[resource] current_camera: &CurrentCamera,
) {
    log::debug!("system running: render sprites");

    // todo
}

#[system]
fn render_graphics(#[resource] graphics: &mut Graphics) {
    graphics.render().expect("failed to render frame");
}