pub mod convert;

// use crate::commands::analyze::AnalyzeArgs;
use crate::commands::convert::ConvertArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(
        about = "Convert a binary RPU to XML Metadata (DolbyLabsMDF)",
        arg_required_else_help(true)
    )]
    Convert(ConvertArgs),
}
