use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "GUI viewer for `nix store --query --tree` output."
)]
pub struct Opts {
    /// PATH in /nix/store to view references of
    #[structopt(name = "PATH", parse(from_os_str))]
    pub nix_store_path: PathBuf,
}

impl Opts {
    pub fn parse_from_args() -> Self {
        Opts::from_args()
    }
}
