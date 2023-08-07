use anyhow::Result;
use clap::Parser;

use crate::commands::Command;
use crate::functions::{Converter, EdlConverter};
use crate::levels::*;
use crate::metadata::*;
use crate::Command::{Convert, Edl};

mod commands;
mod functions;
mod metadata;

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = "CLI tool for creating Dolby Vision XML metadata from an encoded deliverable with binary metadata",
    author = "Rainbaby",
    version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or(env!("CARGO_PKG_VERSION"))
)]
struct Opt {
    #[clap(subcommand)]
    cmd: Command,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    match opt.cmd {
        Convert(args) => Converter::convert(args),
        Edl(args) => EdlConverter::convert(args),
    }
}
