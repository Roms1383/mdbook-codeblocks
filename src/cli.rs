use crate::Result;
use clap::{crate_description, Parser, Subcommand};
use log::error;

const DESCRIPTION: &str = concat!(crate_description!(), "\n", env!("CARGO_PKG_HOMEPAGE"));

#[derive(Parser)]
#[command(author, version, about=DESCRIPTION, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check whether a renderer is supported by this preprocessor
    Supports { renderer: String },
}

pub fn init() -> Result<Opts> {
    Opts::try_parse().map_err(Into::into).map_err(|err| {
        error!("{}", err);
        err
    })
}
