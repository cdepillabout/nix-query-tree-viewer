
use std::path::PathBuf;
use std::process::{Output, Command};

use super::NixQueryTree;

#[derive(Clone, Debug, PartialEq)]
pub enum ExecNixStoreErr {
    CommandErr(String),
    Utf8Err(String),
    NixStoreErr(String),
    ParseErr(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecNixStoreRes {
    pub raw: String,
    pub tree: NixQueryTree,
}

impl ExecNixStoreRes {
    pub fn new(raw: String, tree: NixQueryTree) -> Self {
        ExecNixStoreRes{ raw, tree }
    }
}

pub fn exec_nix_store(nix_store_path: PathBuf) -> Result<ExecNixStoreRes, ExecNixStoreErr> {
    let nix_store_output: Output = Command::new("nix-store")
        .args(&[
            "--query",
            "--tree",
            &nix_store_path.to_string_lossy(),
        ])
        .output()
        .map_err(|io_err| ExecNixStoreErr::CommandErr(io_err.to_string()))?;

    if nix_store_output.status.success() {
        let stdout = from_utf8(nix_store_output.stdout)?;
        super::parsing::nix_query_tree_parser(&stdout.clone())
            .map(|nix_query_tree| ExecNixStoreRes::new(stdout, nix_query_tree))
            .map_err(|nom_err| ExecNixStoreErr::ParseErr(nom_err.to_string()))
    } else {
        let stderr = from_utf8(nix_store_output.stderr)?;
        Err(ExecNixStoreErr::NixStoreErr(stderr))
    }
}

fn from_utf8(i: Vec<u8>) -> Result<String, ExecNixStoreErr> {
    String::from_utf8(i).map_err(|utf8_err| ExecNixStoreErr::Utf8Err(utf8_err.to_string()))
}
