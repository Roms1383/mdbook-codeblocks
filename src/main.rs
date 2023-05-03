use log::error;
use mdbook::preprocess::Preprocessor;
use std::error::Error;
use std::process;

mod cli;
mod language;
mod preprocessor;

pub type Result<Ok = (), Err = Box<dyn Error>> = std::result::Result<Ok, Err>;

fn main() -> Result {
    env_logger::init();
    let opts = cli::init()?;

    // handle supports or processing:
    let codeblocks = preprocessor::Codeblocks::new();
    if let Some(cli::Commands::Supports { renderer }) = opts.command {
        // Signal whether the renderer is supported by exiting with 1 or 0.
        if codeblocks.supports_renderer(&renderer) {
            process::exit(0);
        } else {
            process::exit(1);
        }
    } else if let Err(e) = codeblocks.handle_preprocessing() {
        print_error(&e);
        process::exit(1);
    }

    Ok(())
}

fn print_error(error: &anyhow::Error) {
    let mut chain = error.chain();
    error!("{}", chain.next().unwrap());
    for e in chain {
        error!(" - {e}");
    }
}
