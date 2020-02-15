use super::super::super::ui;
use super::super::prelude::*;

pub fn setup(_state: &ui::State) {}

pub fn disable(_state: &ui::State) {}

pub fn enable(_state: &ui::State) {}

pub fn redisplay_data(state: &ui::State) {
    enable(state);

    if let Some(nix_store_res) = &*state.read_nix_store_res() {
        let text_buffer: gtk::TextBuffer = state.get_raw_text_buffer();
        text_buffer.set_text(&nix_store_res.raw);
    }
}
