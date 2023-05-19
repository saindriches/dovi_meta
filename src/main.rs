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

// Some Clap features are broken, keep it for now.
#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), author = "Rainbaby", about = "CLI tool for creating Dolby Vision XML metadata from an encoded deliverable with binary metadata.", version = option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT").unwrap_or(env!("VERGEN_BUILD_SEMVER")))]
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
