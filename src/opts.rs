use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "GUI viewer for `nix store --query --tree` output.")]
pub struct Opts {
    /// PATH in /nix/store to view references of
    #[structopt(name = "PATH", parse(from_os_str))]
    pub output: PathBuf,

    // /// Where to write the output: to `stdout` or `file`
    // #[structopt(short)]
    // pub out_type: String,

    // /// File name: only required when `out` is set to `file`
    // #[structopt(name = "FILE", required_if("out_type", "file"))]
    // pub file_name: String,
}

impl Opts {
    pub fn parse_from_args() -> Self {
        Opts::from_args()
    }
}
