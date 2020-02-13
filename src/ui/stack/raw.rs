
use std::sync::Arc;

use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes, NixStoreRes};

use super::super::prelude::*;
use super::super::super::ui;

pub fn setup(state: &ui::State) {
}

pub fn disable(state: &ui::State) {
}

pub fn enable(state: &ui::State) {
}

pub fn redisplay_data(state: &ui::State) {
    enable(state);

    if let Some(nix_store_res) = &*state.nix_store_res.lock().unwrap() {
        let text_buffer: gtk::TextBuffer = state.get_raw_text_buffer();
        text_buffer.set_text(&nix_store_res.raw);
    }

    // if let Some(raw) = &state.nix_store_res {
    //     let text_buffer: gtk::TextBuffer = state.get_raw_text_buffer();
    //     let text: &str = raw;
    //     text_buffer.set_text(&text);
    // }
}
