use clap::{Args, ValueHint};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct EdlArgs {
    #[clap(
    help = "Set the input RPU file to use",
    value_hint = ValueHint::FilePath
    )]
    pub input: Option<PathBuf>,

    #[clap(
    help = "Set the output EDL file location. See --help for more info",
    long_help = "Set the output EDL file location.\n \
                 If there are too many cuts to be saved in a single file,\n \
                 multiple files will be saved with a suffix added to the file name.",
    value_hint = ValueHint::FilePath
    )]
    pub output: Option<PathBuf>,

    #[clap(
    help = "Set the clip name in EDL",
    value_hint = ValueHint::FilePath
    )]
    pub clip_name: String,

    #[clap(
        short = 'f',
        long,
        help = "Force output even if per-frame RPU is detected"
    )]
    pub force: bool,

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
        short = 's',
        long,
        default_value = "01:00:00:00",
        help = "Set the starting timecode in timeline. Format: HH:MM:SS:FF or integer FRAMES offset"
    )]
    pub start_timecode: String,

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
}
