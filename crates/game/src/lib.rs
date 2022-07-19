#![feature(result_option_inspect)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod config;
pub mod error;
pub mod game;
pub mod graphics;
pub mod inputs;
pub mod sounds;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::game::Game;

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

    // todo: handle error with color-eyre somehow?
    log::info!("initializing game");
    let game = Game::new()
        .await
        .inspect_err(|e| log::error!("{}", e))
        .unwrap();

    log::info!("running game");
    game.run();
}
