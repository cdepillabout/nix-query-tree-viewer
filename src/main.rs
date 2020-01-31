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
    // let button: gtk::Button = builder.get_object("button1").unwrap();
    // let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();
    let about_menu_item: gtk::MenuItem = builder.get_object("aboutMenuItem").unwrap();
    let quit_menu_item: gtk::MenuItem = builder.get_object("quitMenuItem").unwrap();

    let about_dialog: gtk::AboutDialog = builder.get_object("aboutDialog").unwrap();

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

    about_menu_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    quit_menu_item.connect_activate(clone!(@weak app => move |_| {
        app.quit();
    }));

    window.show_all();
}

// ApplicationExt.connect_activate:
// fn connect_activate
//      <F: Fn(&Self) + 'static>
//      (&self, f: F) ->
//      SignalHandlerId
//
// MenuItemExt.connect_activate:
// fn connect_activate
//      <F: Fn(&Self) + 'static>
//      (&self, f: F) ->
//      SignalHandlerId

fn main() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(run);
    uiapp.run(&env::args().collect::<Vec<_>>());
}
