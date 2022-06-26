use serde::{
    Deserialize,
    Serialize,
};
use winit::dpi::PhysicalSize;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicsConfig {
    #[serde(default = "GraphicsConfig::default_width")]
    pub width: u32,

    #[serde(default = "GraphicsConfig::default_height")]
    pub height: u32,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            width: Self::default_width(),
            height: Self::default_height(),
        }
    }
}

impl GraphicsConfig {
    fn default_width() -> u32 {
        1920
    }

    fn default_height() -> u32 {
        1080
    }

    pub fn physical_size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.width, self.height)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub graphics: GraphicsConfig,
}

impl Config {
    ///  load config depending on whether we're in wasm or standalone.
    ///
    ///  # wasm
    ///
    /// for wasm this will try to get json from local storage key `config`, and
    /// then deserialize it.
    ///
    /// # standalone
    ///
    /// if the `-c` or `--config` flag specifies a path, this will load the
    /// config from the given path as json.
    ///
    /// # default
    ///
    /// if there is no config specified, this will return the default config.
    pub async fn load_opt() -> Result<Option<Self>, Error> {
        let mut config_opt: Option<Self> = None;

        // deserialize config from local storage value for key `config`.
        #[cfg(target_arch = "wasm32")]
        {
            let local_storage = web_sys::window().unwrap().local_storage()?.unwrap();
            if let Some(json) = local_storage.get_item("config")? {
                config_opt = Some(Self::load_from_str(&json)?);
            }
        }

        // get config path from command line and then deserialize from file.
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::path::PathBuf;

            use structopt::StructOpt;

            #[derive(Debug, StructOpt)]
            struct Args {
                /// path to config file.
                #[structopt(short, long)]
                config: Option<PathBuf>,
            }

            let args = Args::from_args();
            log::debug!("args = {:?}", args);

            if let Some(path) = &args.config {
                log::debug!("loading config from file: {}", path.display());
                let json = tokio::fs::read_to_string(path).await?;
                config_opt = Some(Self::load_from_str(&json)?);
            }
        }

        Ok(config_opt)
    }

    pub async fn load() -> Result<Self, Error> {
        let config_opt = Self::load_opt().await?;

        let config = config_opt.unwrap_or_else(|| {
            log::debug!("using default config");

            Default::default()
        });

        Ok(config)
    }

    fn load_from_str(s: &str) -> Result<Self, Error> {
        let config = serde_json::from_str(s)?;

        Ok(config)
    }
}
