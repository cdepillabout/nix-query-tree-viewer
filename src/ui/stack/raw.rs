
use std::sync::Arc;

use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes};

use super::super::prelude::*;
use super::super::super::ui;

pub fn setup(state: &ui::State) {
}

pub fn disable(state: &ui::State) {
}

pub fn enable(state: &ui::State) {
}

pub fn redisplay_data(state: &ui::State, exec_nix_store_res: Arc<ExecNixStoreRes>) {
    enable(state);

    let text_buffer: gtk::TextBuffer = state.get_raw_text_buffer();

    // TODO: This is super ugly.  Why do I have to clone the string in the Ok arm of the match when
    // I just call text_buffer.set_text() on the reference?
    let text: String = match &exec_nix_store_res.res {
        Err(nix_store_err) => nix_store_err.to_string(),
        Ok(nix_store_res_rc) => String::clone(&nix_store_res_rc.raw),
    };

    text_buffer.set_text(&text);
}
