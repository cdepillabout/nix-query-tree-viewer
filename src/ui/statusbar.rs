
use super::prelude::*;
use super::super::ui;

pub fn clear(state: &ui::State) {
    show_msg(state, "");
}

pub fn show_msg(state: &ui::State, msg: &str) {
    let statusbar: gtk::Statusbar = state.get_statusbar();
    statusbar.remove_all(0);
    statusbar.push(0, msg);
}

