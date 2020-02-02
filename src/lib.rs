
pub mod nix_query_tree;
pub mod opts;
pub mod tree;
pub mod ui;

extern crate gio;
extern crate gtk;

pub fn default_main() {
    // nix-store --query --tree /nix/store/jymg0kanmlgbcv35wxd8d660rw0fawhv-hello-2.10.drv
    // nix-store --query --tree /nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10

    let opts = opts::Opts::parse_from_args();

    let nix_store_res = nix_query_tree::exec_nix_store::exec_nix_store(opts.nix_store_path);

    ui::run(nix_store_res);
}
