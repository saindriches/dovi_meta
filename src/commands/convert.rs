use clap::{Args, ValueHint};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ConvertArgs {
    #[clap(
        help = "Set the input RPU file to use",
        value_hint = ValueHint::FilePath
    )]
    pub input: Option<PathBuf>,

    #[clap(
        help = "Set the output XML file location",
        value_hint = ValueHint::FilePath
    )]
    pub output: Option<PathBuf>,

    #[clap(
        short = 's',
        long,
        default_value = "3840x2160",
        use_value_delimiter = true,
        value_delimiter = 'x',
        num_args(1..=2),
        help = "Set the canvas size"
    )]
    pub size: Vec<usize>,

    #[clap(
        short = 'r',
        long,
        default_value = "24000/1001",
        use_value_delimiter = true,
        value_delimiter = '/',
        num_args(1..=2),
        help = "Set the frame rate. Format: integer NUM or NUM/DENOM"
    )]
    pub rate: Vec<usize>,

    #[clap(
        short = '6',
        long,
        help = "Use MaxCLL and MaxFALL from RPU, if possible"
    )]
    pub use_level6: bool,

    #[clap(short = 'd', long, help = "Drop per-frame metadata in shots")]
    pub drop_per_frame: bool,

    #[clap(
        short = 't',
        long,
        default_value = "0",
        help = "Set the number of frames to be skipped from start"
    )]
    pub skip: usize,

    #[clap(
        short = 'n',
        long,
        help = "Set the number of frames to be parsed explicitly"
    )]
    pub count: Option<usize>,

    #[clap(
        short = 'k',
        long,
        requires = "skip",
        help = "Keep the offset of frames when --skip is set"
    )]
    pub keep_offset: bool,
}
