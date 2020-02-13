
use super::super::prelude::*;
use super::super::super::ui;

pub fn setup(_state: &ui::State) {
}

pub fn disable(_state: &ui::State) {
}

pub fn enable(_state: &ui::State) {
}

pub fn redisplay_data(state: &ui::State) {
    enable(state);

    if let Some(nix_store_res) = &*state.nix_store_res.lock().unwrap() {
        let text_buffer: gtk::TextBuffer = state.get_raw_text_buffer();
        text_buffer.set_text(&nix_store_res.raw);
    }
}
