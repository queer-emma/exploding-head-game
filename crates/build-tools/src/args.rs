use std::path::PathBuf;

use color_eyre::eyre::Error;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub enum Args {
    Atlas {
        /// output path for the sprite sheet image
        #[structopt(short, long)]
        output_texture: PathBuf,

        /// output path for the sprite sheet meta data.
        #[structopt(short, long)]
        output_sprite_sheet: PathBuf,

        /// files to put into the texture atlas
        files: Vec<PathBuf>,
    }
}

impl Args {
    pub async fn run(self) -> Result<(), Error> {
        match self {
            Args::Atlas {
                output_texture,
                output_sprite_sheet,
                files,
            } => {
                log::debug!("output_image: `{}`", output_texture.display());
                log::debug!("output_sheet: `{}`", output_sprite_sheet.display());
                log::debug!("output_image:");
                for file in &files {
                    log::debug!(" - `{}`", file.display());
                }

                crate::sprite_sheet::build(output_texture, output_sprite_sheet, &files).await?;
            }
        }

        Ok(())
    }
}
