use super::prelude::*;
use glib::clone;

use super::super::ui;

fn handle_search(state: &ui::State) {

    let search_entry = state.get_search_entry();
    let search_text = search_entry.get_buffer().get_text();

    ui::search_for(state, std::path::Path::new(&search_text));
}

pub fn connect_signals(state: &ui::State) {
    state.get_search_entry().connect_activate(
        clone!(@strong state => move |_| {
            handle_search(&state);
        }),
    );

    state.get_search_button().connect_button_press_event(
        clone!(@strong state => move |_, _| {
            handle_search(&state);
            Inhibit(false)
        }),
    );
}

pub fn disable(state: &ui::State) {
    state.get_search_entry().set_sensitive(false);
    state.get_search_button().set_sensitive(false);
}

pub fn enable(state: &ui::State) {
    state.get_search_entry().set_sensitive(true);
    state.get_search_button().set_sensitive(true);
}


pub fn setup(state: &ui::State) {
    connect_signals(state);
}
