use super::prelude::*;
use glib::clone;

use super::super::ui;

fn handle_search(state: &ui::State) {
    let search_entry = state.get_search_entry();
    let search_text = search_entry.get_buffer().get_text();

    ui::search_for(state, std::path::Path::new(&search_text));
}

fn handle_select_sort_order(state: &ui::State) {
    let combo_box = state.get_sort_combo_box();
    let active_id: u32 = combo_box.get_active().expect("There should always be something active in the sort order combo box.");
    let sort_order = ui::SortOrder::try_from(active_id).expect("active id is not a valid value for SortOrder");
    ui::set_sort_order(state, sort_order);
}

fn handle_select_view_style(state: &ui::State) {
    let combo_box = state.get_view_combo_box();
    let active_id: u32 = combo_box.get_active().expect("There should always be something active in the view style combo box.");
    let view_style = ui::ViewStyle::try_from(active_id).expect("active id is not a valid value for ViewStyle");
    ui::set_view_style(state, view_style);
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

    state.get_sort_combo_box().connect_changed(
        clone!(@strong state => move |_| {
            handle_select_sort_order(&state);
        }),
    );

    state.get_view_combo_box().connect_changed(
        clone!(@strong state => move |_| {
            handle_select_view_style(&state);
        }),
    );
}

pub fn disable(state: &ui::State) {
    state.get_search_entry().set_sensitive(false);
    state.get_search_button().set_sensitive(false);
    state.get_sort_combo_box().set_sensitive(false);
}

pub fn enable(state: &ui::State) {
    state.get_search_entry().set_sensitive(true);
    state.get_search_button().set_sensitive(true);
    state.get_sort_combo_box().set_sensitive(true);
}


pub fn setup(state: &ui::State) {
    connect_signals(state);
}
