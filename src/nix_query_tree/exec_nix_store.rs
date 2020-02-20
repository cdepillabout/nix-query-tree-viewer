use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use super::parsing;
use super::{NixQueryEntry, NixQueryPathMap, NixQueryTree};
use crate::tree;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NixStoreErr {
    CommandErr(String),
    Utf8Err(String),
    NixStoreErr(String),
    ParseErr(String),
}

impl std::fmt::Display for NixStoreErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            NixStoreErr::CommandErr(string) => string,
            NixStoreErr::Utf8Err(string) => string,
            NixStoreErr::NixStoreErr(string) => string,
            NixStoreErr::ParseErr(string) => string,
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NixStoreRes {
    pub raw: String,
    pub tree: NixQueryTree,
    pub map: NixQueryPathMap,
}

impl NixStoreRes {
    pub fn new(raw: &str, tree: NixQueryTree) -> Self {
        let map: NixQueryPathMap = tree.path_map();
        NixStoreRes {
            raw: String::from(raw),
            tree,
            map,
        }
    }

    pub fn lookup_first_query_entry(
        &self,
        nix_query_entry: &NixQueryEntry,
    ) -> Option<&tree::Path> {
        self.map.lookup_first(&nix_query_entry.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecNixStoreRes {
    pub nix_store_path: PathBuf,
    pub res: Result<NixStoreRes, NixStoreErr>,
}

impl ExecNixStoreRes {
    pub fn new(
        nix_store_path: &Path,
        res: Result<NixStoreRes, NixStoreErr>,
    ) -> Self {
        ExecNixStoreRes {
            nix_store_path: nix_store_path.to_path_buf(),
            res,
        }
    }
}

fn nix_store_res(nix_store_path: &Path) -> Result<NixStoreRes, NixStoreErr> {
    let nix_store_output: Output = Command::new("nix-store")
        .args(&["--query", "--tree", &nix_store_path.to_string_lossy()])
        .output()
        .map_err(|io_err| NixStoreErr::CommandErr(io_err.to_string()))?;

    if nix_store_output.status.success() {
        let stdout = from_utf8(nix_store_output.stdout)?;
        parsing::nix_query_tree_parser(&stdout)
            .map(|nix_query_tree| NixStoreRes::new(&stdout, nix_query_tree))
            .map_err(|nom_err| NixStoreErr::ParseErr(nom_err.to_string()))
    } else {
        let stderr = from_utf8(nix_store_output.stderr)?;
        Err(NixStoreErr::NixStoreErr(stderr))
    }
}

/// Run `nix-store --query --tree` for the given nix store path.
pub fn run(nix_store_path: &Path) -> ExecNixStoreRes {
    ExecNixStoreRes {
        nix_store_path: nix_store_path.to_path_buf(),
        res: nix_store_res(nix_store_path),
    }
}

/// Convert a `Vec<u8>` to a proper utf8 `String`, converting the error to `NixStoreErr::Utf8Err`.
fn from_utf8(i: Vec<u8>) -> Result<String, NixStoreErr> {
    String::from_utf8(i)
        .map_err(|utf8_err| NixStoreErr::Utf8Err(utf8_err.to_string()))
}
