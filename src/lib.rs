
pub mod nix_query_tree;
pub mod tree;

mod opts;
mod ui;

pub fn default_main() {
    // nix-store --query --tree /nix/store/jymg0kanmlgbcv35wxd8d660rw0fawhv-hello-2.10.drv
    // nix-store --query --tree /nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10

    let opts = opts::Opts::parse_from_args();

    let nix_store_res = nix_query_tree::exec_nix_store::exec_nix_store(opts.nix_store_path);

    ui::run(nix_store_res);
}
