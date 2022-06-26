#![feature(result_option_inspect)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod config;
pub mod error;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::PhysicalSize,
    event::{
        Event,
        WindowEvent,
    },
    event_loop::ControlFlow,
    window::Window,
};
use wgpu::{
    include_wgsl,
    util::DeviceExt,
    Device,
    Queue,
    Surface,
    SurfaceConfiguration,
};

use crate::{
    config::Config,
    error::Error,
};


/// game state.
#[derive(Debug)]
pub struct State {
    /// the surface that we render to.
    surface: Surface,

    /// the graphics device used for rendering.
    device: Device,

    /// the device's command queue.
    queue: Queue,

    /// config of the surface that we render to.
    config: SurfaceConfiguration,

    /// physical size of surface in pixels.
    size: PhysicalSize<u32>,

    /// render pipeline containing the vertex and fragment shaders.
    render_pipeline: wgpu::RenderPipeline,

    /// vertex buffer
    vertex_buffer: wgpu::Buffer,

    /// index buffer
    index_buffer: wgpu::Buffer,

    /// number of indices in the index buffer.
    num_indices: u32,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Result<Self, Error> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(Error::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    }
                    else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        // make sure the window size is not 0
        assert!(size.width > 0 && size.height > 0);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // load shaders
        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));

        // create pipeline layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // create rendering pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = INDICES.len() as u32;

        // load image
        // todo: use a sprite atlas

        let diffuse_bytes = include_bytes!("../../../assets/exploding_head_pixelart.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        
        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
            }
        );
        

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        })
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    /// todo: this only needs to render rectangular sprites later.
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // This is what [[location(0)]] in the fragment shader targets
                wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 2.

        // drop this, so the encoder is not borrowed anymore
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn handle_event<T: 'static>(
        &mut self,
        event: Event<T>,
        control_flow: &mut ControlFlow,
    ) -> Result<(), Error> {
        *control_flow = ControlFlow::Wait;
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
                        self.config.width = size.width;
                        self.config.height = size.height;
                        self.surface.configure(&self.device, &self.config);
                    }
                    WindowEvent::CloseRequested => {
                        log::info!("close requested");
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => self.render().expect("failed to render frame"),
            _ => {}
        }

        Ok(())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];


#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    color_eyre::install().expect("failed to intall error handling");

    // initialize panic handler and logger for wasm
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
    }

    // initialize panic handler and logger for standalone
    #[cfg(not(target_arch = "wasm32"))]
    {
        dotenv::dotenv()
            .inspect_err(|e| log::warn!("failed to load .env file: {}", e))
            .ok();

        pretty_env_logger::init();
    }

    // load game config
    let config = Config::load().await.expect("config error");
    let window_size = config.graphics.physical_size();
    log::debug!("window_size = {:?}", window_size);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_min_inner_size(window_size)
        .build(&event_loop)
        .unwrap();
    log::info!("window id: {:?}", window.id());

    #[cfg(target_arch = "wasm32")]
    {
        // winit prevents sizing with css, so we have to set
        // the size manually when on web.
        use winit::{
            platform::web::WindowExtWebSys,
        };

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

    let mut state = State::new(&window)
        .await
        .expect("failed to initialize state");

    event_loop.run(move |event, _, control_flow| {
        state
            .handle_event(event, control_flow)
            .expect("failed to handle event");
    });
}
