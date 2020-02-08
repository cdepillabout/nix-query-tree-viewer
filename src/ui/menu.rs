use glib::clone;
use super::prelude::*;

pub fn connect_signals(app: gtk::Application, builder: gtk::Builder) {
    let about_menu_item: gtk::MenuItem = builder.get_object_expect("aboutMenuItem");
    let about_dialog: gtk::AboutDialog = builder.get_object_expect("aboutDialog");
    about_menu_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    let quit_menu_item: gtk::MenuItem = builder.get_object_expect("quitMenuItem");
    quit_menu_item.connect_activate(clone!(@weak app => move |_| {
        app.quit();
    }));
}

