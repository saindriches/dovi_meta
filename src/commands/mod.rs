pub mod convert;
pub mod edl;

// use crate::commands::analyze::AnalyzeArgs;
use crate::commands::convert::ConvertArgs;
use crate::commands::edl::EdlArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(
        about = "Convert a binary RPU to XML Metadata (DolbyLabsMDF)",
        arg_required_else_help(true)
    )]
    Convert(ConvertArgs),

    #[clap(
        about = "Convert a binary RPU to EDL (Edit Decision List)",
        arg_required_else_help(true)
    )]
    Edl(EdlArgs),
}
