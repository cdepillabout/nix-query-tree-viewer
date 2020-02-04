pub mod builder;

use gdk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

use super::nix_query_tree::exec_nix_store::{ExecNixStoreRes, NixStoreErr, NixStoreRes};
use super::nix_query_tree::{NixQueryDrv, NixQueryEntry, NixQueryTree, Recurse};
use super::tree;
use super::tree::Tree;
use builder::*;

fn connect_menu_buttons(app: gtk::Application, builder: gtk::Builder) {
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

fn show_msg_in_statusbar(builder: gtk::Builder, msg: &str) {
    let statusbar: gtk::Statusbar = builder.get_object_expect("statusbar");
    statusbar.remove_all(0);
    statusbar.push(0, msg);
}

#[derive(Debug)]
#[repr(i32)]
enum Column {
    Item = 0,
    Recurse,
}

const COL_INDICIES: [u32; 2] = [Column::Item as u32, Column::Recurse as u32];

fn insert_child_into_tree_store(
    tree_store: gtk::TreeStore,
    parent: Option<gtk::TreeIter>,
    child: &Tree<NixQueryEntry>,
) {
    let Tree { item, children }: &Tree<NixQueryEntry> = child;
    let drv: &NixQueryDrv = &item.0;
    let drv_str = drv.to_string();
    let recurse_str = if item.1 == Recurse::Yes {
        "go to first instance"
    } else {
        ""
    };
    let this_iter: gtk::TreeIter =
        tree_store.insert_with_values(parent.as_ref(), None, &COL_INDICIES, &[&drv_str, &recurse_str]);
    insert_children_into_tree_store(tree_store, this_iter, children);
}

fn insert_children_into_tree_store(
    tree_store: gtk::TreeStore,
    parent: gtk::TreeIter,
    children: &[Tree<NixQueryEntry>],
) {
    for child in children {
        let _: &Tree<NixQueryEntry> = child;
        insert_child_into_tree_store(tree_store.clone(), Some(parent.clone()), child)
    }
}

fn insert_into_tree_store(tree_store: gtk::TreeStore, nix_store_res: &NixStoreRes) {
    let nix_query_tree: &NixQueryTree = &nix_store_res.tree;
    let tree: &Tree<NixQueryEntry> = &nix_query_tree.0;
    insert_child_into_tree_store(tree_store, None, tree);
}

fn render_nix_store_err(builder: gtk::Builder, nix_store_path: &Path, nix_store_err: &NixStoreErr) {
    let error_dialog: gtk::MessageDialog = builder.get_object_expect("errorDialog");
    let error_msg = &format!(
        "Error running `nix-store --query --tree {}`:\n\n{}",
        nix_store_path.to_string_lossy(),
        nix_store_err
    );
    error_dialog.set_property_secondary_text(Some(error_msg));
    error_dialog.run();
    error_dialog.destroy();
    show_msg_in_statusbar(
        builder,
        &format!(
            "Error running `nix-store --query --tree {}`",
            nix_store_path.to_string_lossy()
        ),
    );
}

fn render_nix_store_res(
    builder: gtk::Builder,
    tree_store: gtk::TreeStore,
    nix_store_res: &ExecNixStoreRes,
) {
    match &nix_store_res.res {
        Err(err) => render_nix_store_err(builder, &nix_store_res.nix_store_path, err),
        Ok(res) => insert_into_tree_store(tree_store, res),
    }
}

fn create_builder() -> gtk::Builder {
    let glade_src = include_str!("../glade/ui.glade");
    gtk::Builder::new_from_string(glade_src)
}

fn toggle_row(tree_view: gtk::TreeView, tree_path: gtk::TreePath, recurse: bool) {
    if tree_view.row_expanded(&tree_path) {
        tree_view.collapse_row(&tree_path);
    } else {
        tree_view.expand_row(&tree_path, recurse);
    }
}

fn create_item_column(tree_view: gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();

    let column = gtk::TreeViewColumn::new();
    column.set_title("item");
    column.pack_start(&renderer, false);
    column.add_attribute(&renderer, "text", Column::Item as i32);

    tree_view.append_column(&column);
}

fn create_link_column(tree_view: gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    renderer.set_property_underline(pango::Underline::Single);
    renderer.set_property_foreground(Some("blue"));

    let column = gtk::TreeViewColumn::new();
    column.set_title("repeat");
    column.pack_end(&renderer, false);
    column.add_attribute(&renderer, "text", Column::Recurse as i32);

    tree_view.append_column(&column);
}

fn create_columns(tree_view: gtk::TreeView) {
    create_item_column(tree_view.clone());
    create_link_column(tree_view);
}

fn get_tree_view_column_pos(tree_view: gtk::TreeView, tree_view_column: gtk::TreeViewColumn) -> usize {
    let all_tree_view_columns = tree_view.get_columns();
    let option_pos = all_tree_view_columns.iter().position(|col| *col == tree_view_column);
    match option_pos {
        None => panic!("No column matching id.  This should never happen."),
        Some(pos) => pos,
    }
}

fn gtk_tree_path_to_tree_path(gtk_tree_path: gtk::TreePath) -> tree::Path {
    tree::Path(gtk_tree_path.get_indices().iter().map(|i| *i as usize).collect::<VecDeque<usize>>())
}

// fn nix_query_entry_from_gtk_tree_path(

fn setup_tree_view(builder: gtk::Builder, nix_store_res: &ExecNixStoreRes) -> (gtk::TreeStore, gtk::TreeView) {
    let tree_view: gtk::TreeView = builder.get_object_expect("treeView");
    let tree_store: gtk::TreeStore =
        gtk::TreeStore::new(&[glib::types::Type::String, glib::types::Type::String]);

    tree_view.set_model(Some(&tree_store));

    create_columns(tree_view.clone());

    // TODO: It is kinda ugly that I have to clone this...
    let res_clone = nix_store_res.clone();

    tree_view.connect_row_activated(move |tree_view_ref, tree_path, tree_view_column: &gtk::TreeViewColumn| {
        match &res_clone.res {
            Err(_) => {
                let column_pos = get_tree_view_column_pos(tree_view_ref.clone(), tree_view_column.clone());
                let path = gtk_tree_path_to_tree_path(tree_path.clone());
                format!("column_pos: {:?}, path: {:?}", column_pos, path.clone());
            },
            Ok(res) => {
                let column_pos = get_tree_view_column_pos(tree_view_ref.clone(), tree_view_column.clone());
                let path = gtk_tree_path_to_tree_path(tree_path.clone());
                let nix_query_tree = &res.tree;
                let option_nix_query_entry = nix_query_tree.lookup(path.clone());

                format!("column_pos: {:?}, path: {:?}, option_nix_query_entry: {:?}", column_pos, path.clone(), option_nix_query_entry);

                // stupid rust
                // match column_pos == Column::Recurse as usize, option_nix_query_entry) let Some(nix_query_entry) =  option_nix_query_entry {
                match (column_pos == Column::Recurse as usize, option_nix_query_entry) {
                    (true, Some(nix_query_entry)) if nix_query_entry.1 == Recurse::Yes => {
                        println!("hello");
                    }
                    _ => {
                        toggle_row(tree_view_ref.clone(), tree_path.clone(), false);
                    }
                }
            }
        }
    });

    (tree_store, tree_view)
}

fn setup_css(window: gtk::Window) {
    let screen: gdk::Screen = match window.get_screen() {
        Some(screen) => screen,
        None => {
            println!("Failed to get the screen for window.");
            return;
        }
    };
    let css_provider = gtk::CssProvider::new();
    let css_src = include_str!("../style/style.css");
    match css_provider.load_from_data(css_src.as_bytes()) {
        Err(err) => println!("Failed to load css provider from data: {}", err),
        Ok(_) => {
            gtk::StyleContext::add_provider_for_screen(
                &screen,
                &css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }
}

fn app_activate(nix_store_res: ExecNixStoreRes, app: gtk::Application) {
    let builder = create_builder();

    let window: gtk::ApplicationWindow = builder.get_object_expect("appWindow");
    window.set_application(Some(&app));

    setup_css(window.clone().upcast());

    let (tree_store, tree_view) = setup_tree_view(builder.clone(), &nix_store_res);

    render_nix_store_res(builder.clone(), tree_store, &nix_store_res);

    // expand the first row of the tree view
    tree_view.expand_row(&gtk::TreePath::new_first(), false);

    connect_menu_buttons(app, builder);

    window.show_all();
}

pub fn run(nix_store_res: ExecNixStoreRes) {
    let uiapp = gtk::Application::new(
        Some("com.github.cdepillabout.nix-query-tree-viewer"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(move |app| app_activate(nix_store_res.clone(), app.clone()));

    // uiapp.run(&env::args().collect::<Vec<_>>());
    uiapp.run(&[]);
}
