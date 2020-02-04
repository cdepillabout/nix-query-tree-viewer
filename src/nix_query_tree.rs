pub mod parsing;
pub mod exec_nix_store;

use super::tree::{Path, Tree, TreePathMap};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NixQueryDrv(PathBuf);

impl From<&str> for NixQueryDrv {
    fn from(item: &str) -> Self {
        NixQueryDrv(PathBuf::from(item))
    }
}

impl FromStr for NixQueryDrv {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl std::fmt::Display for NixQueryDrv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Recurse {
    Yes,
    No,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NixQueryEntry(pub NixQueryDrv, pub Recurse);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NixQueryTree(pub Tree<NixQueryEntry>);

impl NixQueryTree {
    fn path_map(&self) -> NixQueryPathMap {
        let tree: &Tree<NixQueryEntry> = &self.0;
        let tree_path_map = tree.path_map_map(&|nix_query_entry| nix_query_entry.0);
        NixQueryPathMap(tree_path_map)
    }

    pub fn lookup(&self, path: Path) -> Option<&NixQueryEntry> {
        self.0.lookup(path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NixQueryPathMap(pub TreePathMap<NixQueryDrv>);

impl NixQueryPathMap {
    pub fn lookup_first(&self, k: &NixQueryDrv) -> Option<&Path> {
        self.0.lookup_first(k)
    }
}
