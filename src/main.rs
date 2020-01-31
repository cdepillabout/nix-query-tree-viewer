extern crate gtk;
extern crate gio;

use glib::clone;
use gio::prelude::*;
use gtk::prelude::*;

use std::env;

fn run(app: &gtk::Application) {
    let glade_src = include_str!("../glade/ui.glade");
    // Then we call the Builder call.
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("appWindow").unwrap();
    window.set_application(Some(app));

    let tree_view: gtk::TreeView = builder.get_object("treeView").unwrap();
    let tree_store: gtk::TreeStore = gtk::TreeStore::new(&[glib::types::Type::String]);
    let top_level_iter = tree_store.insert_with_values(None, None, &[0], &[&String::from("test")]);

    tree_view.set_model(Some(&tree_store));

    let renderer = gtk::CellRendererText::new();

    let column = gtk::TreeViewColumn::new();
    // column.set_title("the title");
    column.pack_start(&renderer, false);
    column.add_attribute(&renderer, "text", 0);

    tree_view.append_column(&column);

    let about_menu_item: gtk::MenuItem = builder.get_object("aboutMenuItem").unwrap();
    let about_dialog: gtk::AboutDialog = builder.get_object("aboutDialog").unwrap();
    about_menu_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    let quit_menu_item: gtk::MenuItem = builder.get_object("quitMenuItem").unwrap();
    quit_menu_item.connect_activate(clone!(@weak app => move |_| {
        app.quit();
    }));

    window.show_all();
}

fn main() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(run);
    uiapp.run(&env::args().collect::<Vec<_>>());
}
