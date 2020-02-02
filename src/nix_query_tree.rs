pub mod parsing;
pub mod exec_nix_store;

use super::tree::Tree;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Recurse {
    Yes,
    No,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NixQueryEntry(pub NixQueryDrv, pub Recurse);

#[derive(Clone, Debug, PartialEq)]
pub struct NixQueryTree(pub Tree<NixQueryEntry>);
