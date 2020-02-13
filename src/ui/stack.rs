mod raw;
mod tree;

use std::sync::Arc;

use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes};

use super::super::ui;

pub fn setup(state: &ui::State) {
    tree::setup(&state);
    raw::setup(&state);
}

pub fn disable(state: &ui::State) {
    tree::disable(state);
    raw::disable(state);
}

pub fn enable(state: &ui::State) {
    tree::enable(state);
    raw::enable(state);
}

pub fn redisplay_data(state: &ui::State, exec_nix_store_res_rc: Arc<ExecNixStoreRes>) {
    tree::redisplay_data(&state, Arc::clone(&exec_nix_store_res_rc));
    raw::redisplay_data(&state, exec_nix_store_res_rc);
}
