use color_eyre::eyre::Error;
use game::run;

/// standalone main function. this just awaits the run function, which is the
/// wasm/standalone entry point.
#[tokio::main]
async fn main() -> Result<(), Error> {
    run().await;

    Ok(())
}
