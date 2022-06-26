#![feature(result_option_inspect)]
#![allow(dead_code)]
#![allow(unused_variables)]


pub mod config;
pub mod error;
pub mod game;
pub mod graphics;
pub mod inputs;
pub mod sounds;

use winit::{event_loop::EventLoop, window::WindowBuilder};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{config::Config, graphics::Graphics};


/// todo: split out window stuff and input better.
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

    let mut graphics = Graphics::new(&window)
        .await
        .expect("failed to initialize state");

    event_loop.run(move |event, _, control_flow| {
        graphics
            .handle_event(event, control_flow)
            .expect("failed to handle event");
    });
}

