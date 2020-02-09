mod raw;
mod tree;

use std::sync::Arc;

use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes};

use super::super::ui as ui;

pub fn setup(state: ui::State, exec_nix_store_res_rc: Arc<ExecNixStoreRes>) {
    tree::setup(&state, Arc::clone(&exec_nix_store_res_rc));
    raw::setup(&state, exec_nix_store_res_rc);
}
