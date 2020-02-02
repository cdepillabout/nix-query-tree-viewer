
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use super::nix_query_tree::exec_nix_store::{ExecNixStoreErr, ExecNixStoreRes};

fn connect_menu_buttons(app: gtk::Application, builder: gtk::Builder) {
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
}

fn render_tree_view(builder: gtk::Builder, nix_store_res: &Result<ExecNixStoreRes, ExecNixStoreErr>) {
    let tree_view: gtk::TreeView = builder.get_object("treeView").unwrap();
    let tree_store: gtk::TreeStore = gtk::TreeStore::new(&[glib::types::Type::String]);
    let _top_level_iter = tree_store.insert_with_values(None, None, &[0], &[&String::from("test")]);

    tree_view.set_model(Some(&tree_store));

    let renderer = gtk::CellRendererText::new();

    let column = gtk::TreeViewColumn::new();
    // column.set_title("the title");
    column.pack_start(&renderer, false);
    column.add_attribute(&renderer, "text", 0);

    tree_view.append_column(&column);
}

fn create_builder() -> gtk::Builder {
    let glade_src = include_str!("../glade/ui.glade");
    gtk::Builder::new_from_string(glade_src)
}

fn app_activate(nix_store_res: Result<ExecNixStoreRes, ExecNixStoreErr>, app: gtk::Application) {
    let builder = create_builder();

    let window: gtk::ApplicationWindow = builder.get_object("appWindow").unwrap();
    window.set_application(Some(&app));

    render_tree_view(builder.clone(), &nix_store_res);

    connect_menu_buttons(app, builder);

    window.show_all();
}

pub fn run(nix_store_res: Result<ExecNixStoreRes, ExecNixStoreErr>) {
    let uiapp = gtk::Application::new(
        Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(move |app| app_activate(nix_store_res.clone(), app.clone()));

    // uiapp.run(&env::args().collect::<Vec<_>>());
    uiapp.run(&[]);
}
