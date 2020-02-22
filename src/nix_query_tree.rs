pub mod exec_nix_store;
pub mod parsing;

use super::tree::{Path, Tree, TreePathMap};
use std::path::PathBuf;
use std::str::FromStr;

/// This corresponds to a nix store path.
///
/// ```
/// use nix_query_tree_viewer::nix_query_tree::NixQueryDrv;
///
/// let nix_query_drv =
///     NixQueryDrv::from("/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10");
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NixQueryDrv(PathBuf);

impl<T: ?Sized + AsRef<std::ffi::OsStr>> From<&T> for NixQueryDrv {
    fn from(s: &T) -> NixQueryDrv {
        NixQueryDrv(PathBuf::from(s.as_ref().to_os_string()))
    }
}

impl std::ops::Deref for NixQueryDrv {
    type Target = std::path::Path;

    fn deref(&self) -> &std::path::Path {
        &self.0
    }
}

impl NixQueryDrv {
    pub fn cmp_hash(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }

    pub fn cmp_drv_name(&self, other: &Self) -> std::cmp::Ordering {
        self.drv_name().cmp(&other.drv_name())
    }

    /// Pull out the hash and derivation name from a `NixQueryDrv`
    ///
    /// ```
    /// use nix_query_tree_viewer::nix_query_tree::NixQueryDrv;
    ///
    /// let nix_query_drv =
    ///     NixQueryDrv::from("/nix/store/az4kl5slhbkmmy4vj98z3hzxxkan7zza-gnugrep-3.3");
    /// assert_eq!(
    ///     nix_query_drv.hash_and_drv_name(),
    ///     String::from("az4kl5slhbkmmy4vj98z3hzxxkan7zza-gnugrep-3.3")
    /// );
    /// ```
    pub fn hash_and_drv_name(&self) -> String {
        let drv_str = self.0.to_string_lossy();
        String::from((drv_str).trim_start_matches("/nix/store/"))
    }

    /// Pull out a truncated hash and derivation name from a `NixQueryDrv`
    ///
    /// ```
    /// use nix_query_tree_viewer::nix_query_tree::NixQueryDrv;
    ///
    /// let nix_query_drv =
    ///     NixQueryDrv::from("/nix/store/az4kl5slhbkmmy4vj98z3hzxxkan7zza-gnugrep-3.3");
    /// assert_eq!(
    ///     nix_query_drv.short_hash_and_drv_name(),
    ///     String::from("az4kl5s..gnugrep-3.3")
    /// );
    /// ```
    pub fn short_hash_and_drv_name(&self) -> String {
        let drv_str = self.0.to_string_lossy();
        let drv_str_no_store = String::from(drv_str.trim_start_matches("/nix/store/"));
        let option_drv_name = drv_str_no_store.find('-').and_then(|i| drv_str_no_store.get(i+1..));
        let option_short_hash = drv_str_no_store.get(0..7);
        match (option_drv_name, option_short_hash) {
            (Some (drv_name), Some (short_hash)) => {
                format!("{}..{}", short_hash, drv_name)
            }
            _ => {
                panic!("Ill-formed nix path")
            }
        }
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
    /// This panics if the derivation name doesn't have a `-` in it.  All nix derivations have
    /// a `-` in them after the hash.
    pub fn drv_name(&self) -> String {
        let drv_str = self.0.to_string_lossy();
        let option_dash_index = drv_str.find('-');
        match option_dash_index {
            None => drv_str.into_owned(),
            Some(dash_index) => {
                let option_just_drv_name = drv_str.get(dash_index + 1..);
                match option_just_drv_name {
                    None => {
                        panic!("Nix paths will always have a dash in them.")
                    }
                    Some(drv_name) => drv_name.to_string(),
                }
            }
        }
    }
}

impl FromStr for NixQueryDrv {
    // This should really be never.
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

/// Whether or not there is a separate entry in this tree that recurses into the dependencies for
/// this nix store entry.
///
/// See `NixQueryEntry`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Recurse {
    Yes,
    No,
}

/// `NixQueryDrv` coupled with a marker for a recursive entry.
///
/// ```
/// use nix_query_tree_viewer::nix_query_tree::{NixQueryEntry, Recurse};
/// use std::str::FromStr;
///
/// let nix_query_entry =
///     NixQueryEntry::from_str("/nix/store/az4kl5slhbkmmy4vj98z3hzxxkan7zza-gnugrep-3.3 [...]");
/// let actual_nix_query_entry =
///     NixQueryEntry::new("/nix/store/az4kl5slhbkmmy4vj98z3hzxxkan7zza-gnugrep-3.3", Recurse::Yes);
/// assert_eq!(nix_query_entry, Ok(actual_nix_query_entry));
/// ```
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NixQueryEntry(pub NixQueryDrv, pub Recurse);

impl FromStr for NixQueryEntry {
    type Err = nom::Err<(String, nom::error::ErrorKind)>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parsing::nix_query_entry_parser(s).map_err(|err| err.to_owned())
    }
}

impl std::ops::Deref for NixQueryEntry {
    type Target = std::path::Path;

    fn deref(&self) -> &std::path::Path {
        &self.0
    }
}

impl NixQueryEntry {
    pub fn new<T>(nix_query_drv: &T, recurse: Recurse) -> NixQueryEntry
    where
        T: ?Sized + AsRef<std::ffi::OsStr>,
    {
        NixQueryEntry(NixQueryDrv::from(nix_query_drv), recurse)
    }

    pub fn cmp_hash(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp_hash(&other.0)
    }

    pub fn cmp_drv_name(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp_drv_name(&other.0)
    }

    pub fn hash_and_drv_name(&self) -> String {
        self.0.hash_and_drv_name()
    }

    pub fn short_hash_and_drv_name(&self) -> String {
        self.0.short_hash_and_drv_name()
    }

    pub fn drv_name(&self) -> String {
        self.0.drv_name()
    }
}

/// A `Tree` representing the result from `nix store --query --tree`.
///
/// ```
/// use indoc::indoc;
/// use nix_query_tree_viewer::nix_query_tree::NixQueryTree;
/// use std::str::FromStr;
///
/// let raw_tree = indoc!(
///         "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10
///         +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27
///         |   +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27 [...]
///         +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]
///         "
///     );
/// let nix_query_tree = NixQueryTree::from_str(raw_tree);
///
/// assert!(nix_query_tree.is_ok());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NixQueryTree(pub Tree<NixQueryEntry>);

impl NixQueryTree {
    pub fn path_map(&self) -> NixQueryPathMap {
        let tree: &Tree<NixQueryEntry> = &self.0;
        let tree_path_map =
            tree.path_map_map(&|nix_query_entry| nix_query_entry.0.clone());
        NixQueryPathMap(tree_path_map)
    }

    pub fn lookup(&self, path: Path) -> Option<&NixQueryEntry> {
        self.0.lookup(path)
    }
}

impl FromStr for NixQueryTree {
    type Err = nom::Err<(String, nom::error::ErrorKind)>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parsing::nix_query_tree_parser(s).map_err(|err| err.to_owned())
    }
}

/// A mapping of `NixQueryDrv` to `TreePath`.  This gives an easy way to
/// figure out where a `NixQueryDrv` is an a `NixQueryTree`.
///
/// ```
/// use indoc::indoc;
/// use nix_query_tree_viewer::nix_query_tree::{NixQueryDrv, NixQueryTree};
/// use nix_query_tree_viewer::tree::Path;
/// use std::str::FromStr;
///
/// let raw_tree = indoc!(
///         "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10
///         +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27
///         |   +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27 [...]
///         +---/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10 [...]
///         +---/nix/store/9ny6szla9dg61jv8q22qbnqsz37465n0-multiple-outputs.sh
///             +---/nix/store/pnd2kl27sag76h23wa5kl95a76n3k9i3-glibc-2.27 [...]
///                 +---/nix/store/5wvmvcc3b7sisirx1vsqbqdis0sd1x5d-cc-wrapper.sh
///                 +---/nix/store/5jzbjvnrz85n454inlyxcpgap9i6k6la-pcre-8.43
///         "
///     );
/// let nix_query_tree = NixQueryTree::from_str(raw_tree).unwrap();
/// let map = nix_query_tree.path_map();
/// let pcre_drv = NixQueryDrv::from("/nix/store/5jzbjvnrz85n454inlyxcpgap9i6k6la-pcre-8.43");
/// let expected_path = Some(Path::from(vec![2, 0, 1]));
///
/// assert_eq!(map.lookup_first(&pcre_drv), expected_path.as_ref());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NixQueryPathMap(pub TreePathMap<NixQueryDrv>);

impl NixQueryPathMap {
    pub fn lookup_first(&self, k: &NixQueryDrv) -> Option<&Path> {
        self.0.lookup_first(k)
    }
}
