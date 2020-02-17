pub mod exec_nix_store;
pub mod parsing;

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

impl NixQueryDrv {
    pub fn path(&self) -> &std::path::Path {
        &self.0
    }

    pub fn cmp_hash(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }

    /// Pull out a derivation name from a `NixQueryDrv`.
    ///
    /// ```
    /// use nix_query_tree_viewer::nix_query_tree::NixQueryDrv;
    ///
    /// let nix_query_drv =
    ///     NixQueryDrv::from("/nix/store/az4kl5slhbkmmy4vj98z3hzxxkan7zza-gnugrep-3.3");
    /// assert_eq!(nix_query_drv.drv_name(), String::from("gnugrep-3.3"));
    /// ```
    ///
    /// * Panics
    ///
    /// This panics if the derivation name doesn't have a `-` in it.  All nix derivations must have
    /// a `-` in them after the hash.
    pub fn drv_name(&self) -> String {
        let drv_str = self.0.to_string_lossy();
        let option_dash_index = drv_str.find('-');
        match option_dash_index {
            None => drv_str.into_owned(),
            Some(dash_index) => {
                let option_just_drv_name = drv_str.get(dash_index + 1 ..);
                match option_just_drv_name {
                    None => panic!("Nix paths will always have a dash in them."),
                    Some(drv_name) => drv_name.to_string(),
                }
            }
        }
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

impl NixQueryEntry {
    pub fn path(&self) -> &std::path::Path {
        self.0.path()
    }

    pub fn cmp_hash(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp_hash(&other.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NixQueryTree(pub Tree<NixQueryEntry>);

impl NixQueryTree {
    fn path_map(&self) -> NixQueryPathMap {
        let tree: &Tree<NixQueryEntry> = &self.0;
        let tree_path_map =
            tree.path_map_map(&|nix_query_entry| nix_query_entry.0);
        NixQueryPathMap(tree_path_map)
    }

    pub fn lookup(&self, path: &Path) -> Option<&NixQueryEntry> {
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
