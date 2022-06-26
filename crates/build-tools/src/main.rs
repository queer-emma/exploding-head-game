mod args;
mod sprite_sheet;

use color_eyre::eyre::Error;
use structopt::StructOpt;

use crate::args::Args;

#[tokio::main]
async fn main() -> Result<(), Error> {
    color_eyre::install()?;

    let args = Args::from_args();
    args.run().await?;

    Ok(())
}
