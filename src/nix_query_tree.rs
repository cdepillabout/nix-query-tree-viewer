
use std::process::Command;
use std::str::FromStr;

struct NixQueryDrv(String);

impl From<&str> for NixQueryDrv {
    fn from(item: &str) -> Self {
        NixQueryDrv(item.into())
    }
}

impl FromStr for NixQueryDrv {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

struct NixQueryTree(ego_tree::Tree<NixQueryDrv>);

pub fn exec_command() -> String {
    let nix_store_stdout_raw = Command::new("nix-store")
        .args(&[
            "--query",
            "--tree",
            "/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10",
        ])
        .output()
        .expect("failed to execute nix-store")
        .stdout;

    let nix_store_stdout = String::from_utf8(nix_store_stdout_raw)
        .expect("failed to convert nix-store output to utf8");

    println!("nix-store output: {}", nix_store_stdout);

    nix_store_stdout
}
