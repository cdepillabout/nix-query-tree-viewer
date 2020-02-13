use super::prelude::*;
use glib::clone;

use super::super::ui;

fn connect_signals(state: &ui::State) {
    let about_menu_item: gtk::MenuItem = state.get_about_menu_item();
    let about_dialog: gtk::AboutDialog = state.get_about_dialog();

    about_menu_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    let quit_menu_item: gtk::MenuItem = state.get_quit_menu_item();

    quit_menu_item.connect_activate(
        clone!(@weak state.app as app => move |_| {
            app.quit();
        }),
    );
}

pub fn setup(state: &ui::State) {
    connect_signals(state);
}
