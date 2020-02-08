mod column;

use glib::clone;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::rc::Rc;

use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes, NixStoreRes};
use crate::nix_query_tree::{NixQueryDrv, NixQueryEntry, NixQueryTree, Recurse};
use crate::tree;
use crate::tree::Tree;

use super::super::prelude::*;

use column::Column;

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
    let this_iter: gtk::TreeIter = tree_store.insert_with_values(
        parent.as_ref(),
        None,
        &column::INDICIES
            .iter()
            .map(|&i| i as u32)
            .collect::<Vec<u32>>(),
        &[&drv_str, &recurse_str],
    );
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

pub fn insert_into_tree_store(tree_store: gtk::TreeStore, nix_store_res: &NixStoreRes) {
    let nix_query_tree: &NixQueryTree = &nix_store_res.tree;
    let tree: &Tree<NixQueryEntry> = &nix_query_tree.0;
    insert_child_into_tree_store(tree_store, None, tree);
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

fn get_tree_view_column_pos(
    tree_view: gtk::TreeView,
    tree_view_column: gtk::TreeViewColumn,
) -> usize {
    let all_tree_view_columns = tree_view.get_columns();
    let option_pos = all_tree_view_columns
        .iter()
        .position(|col| *col == tree_view_column);
    match option_pos {
        None => panic!("No column matching id.  This should never happen."),
        Some(pos) => pos,
    }
}

fn gtk_tree_path_to_tree_path(gtk_tree_path: gtk::TreePath) -> tree::Path {
    tree::Path(
        gtk_tree_path
            .get_indices()
            .iter()
            .map(|i| *i as usize)
            // our tree::Path will only ever have one item at the root, so we can drop the first
            // item from the gtk::TreePath.
            .skip(1)
            .collect::<VecDeque<usize>>(),
    )
}

fn tree_path_to_gtk_tree_path(path: &tree::Path) -> gtk::TreePath {
    let mut vec_indices: Vec<i32> = path.0.iter().map(|&u| u as i32).collect();
    vec_indices.insert(0, 0);
    let gtk_tree_path = gtk::TreePath::new_from_indicesv(&vec_indices);
    gtk_tree_path
}

fn tree_view_go_to_path(tree_view: gtk::TreeView, first_path: &tree::Path) {
    let first_gtk_path = tree_path_to_gtk_tree_path(first_path);
    let col = tree_view.get_column(Column::Item as i32);

    // Open recursively upward from this new path.
    tree_view.expand_to_path(&first_gtk_path);

    // Scroll to the newly opened path.
    tree_view.scroll_to_cell(Some(&first_gtk_path), col.as_ref(), true, 0.5, 0.5);

    let tree_selection: gtk::TreeSelection = tree_view.get_selection();
    // Select the newly opened path.
    tree_selection.select_path(&first_gtk_path);
}

fn nix_query_tree_lookup_gtk_path(
    nix_query_tree: Rc<NixQueryTree>,
    tree_path: gtk::TreePath,
) -> Option<NixQueryEntry> {
    let path = gtk_tree_path_to_tree_path(tree_path.clone());
    nix_query_tree.lookup(path.clone()).cloned()
}

fn gtk_tree_view_column_to_column(
    tree_view: gtk::TreeView,
    tree_view_column: gtk::TreeViewColumn,
) -> Option<Column> {
    let column_pos: usize = get_tree_view_column_pos(tree_view.clone(), tree_view_column.clone());
    Column::try_from(column_pos).ok()
}

fn nix_store_res_lookup_gtk_path(
    nix_store_res: Rc<NixStoreRes>,
    tree_path: gtk::TreePath,
) -> Option<NixQueryEntry> {
    nix_query_tree_lookup_gtk_path(Rc::clone(&nix_store_res.tree), tree_path)
}

fn nix_store_res_lookup_first_query_entry(
    nix_store_res_rc: Rc<NixStoreRes>,
    nix_query_entry: &NixQueryEntry,
) -> Option<tree::Path> {
    nix_store_res_rc
        .map
        .lookup_first(&nix_query_entry.0)
        .cloned()
}

fn gtk_tree_path_is_for_recurse_column(
    tree_view: gtk::TreeView,
    tree_view_column: gtk::TreeViewColumn,
    tree_path: gtk::TreePath,
    nix_store_res_rc: Rc<NixStoreRes>,
) -> Option<NixQueryEntry> {
    let option_column = gtk_tree_view_column_to_column(tree_view.clone(), tree_view_column.clone());
    let option_nix_query_entry_is_recurse =
        nix_store_res_lookup_gtk_path(nix_store_res_rc, tree_path.clone())
            .filter(|nix_query_entry| nix_query_entry.1 == Recurse::Yes);

    match (option_column, option_nix_query_entry_is_recurse) {
        (Some(Column::Recurse), Some(nix_query_entry)) => Some(nix_query_entry),
        _ => None,
    }
}

// This function assumes that nix_query_entry actually exists in NixStoreRes
fn go_to_path_for_query_entry(
    tree_view: gtk::TreeView,
    nix_store_res_rc: Rc<NixStoreRes>,
    nix_query_entry: &NixQueryEntry,
) {
    let option_first_path =
        nix_store_res_lookup_first_query_entry(nix_store_res_rc, &nix_query_entry);
    match option_first_path {
        None => panic!("Nothing in our map for this drv.  This should hever happen."),
        Some(first_path) => {
            tree_view_go_to_path(tree_view, &first_path);
        }
    }
}

fn handle_row_activated(
    tree_view: gtk::TreeView,
    tree_path: gtk::TreePath,
    tree_view_column: gtk::TreeViewColumn,
    nix_store_res_rc: Rc<NixStoreRes>,
) {
    match gtk_tree_path_is_for_recurse_column(
        tree_view.clone(),
        tree_view_column.clone(),
        tree_path.clone(),
        Rc::clone(&nix_store_res_rc),
    ) {
        Some(nix_query_entry) => go_to_path_for_query_entry(
            tree_view,
            Rc::clone(&nix_store_res_rc),
            &nix_query_entry,
        ),
        _ => toggle_row(tree_view.clone(), tree_path.clone(), false),
    }
}

fn create_search_for_this_menu_item(
    builder: gtk::Builder,
    tree_view: gtk::TreeView,
    exec_nix_store_res_rc: Rc<ExecNixStoreRes>,
) -> gtk::MenuItem {
    let search_for_this_menu_item = gtk::MenuItem::new_with_label("Search for this");

    // TODO: Shouldn't have to clone exec_nix_store_res
    search_for_this_menu_item.connect_activate(clone!(@weak tree_view, @weak builder => move |_| {
        clear(tree_view.clone());
        render_tree_store(builder, tree_view, Rc::clone(&exec_nix_store_res_rc));
    }));

    search_for_this_menu_item
}

fn handle_button_press_event(
    builder: gtk::Builder,
    tree_view: gtk::TreeView,
    event_button: gdk::EventButton,
    nix_store_res_rc: Rc<NixStoreRes>,
) -> Inhibit {
    if event_button.get_event_type() == gdk::EventType::ButtonPress
        && event_button.get_button() == 3
    {
        let menu: gtk::Menu = gtk::Menu::new();
        // TODO: this nix store exec thing is really hacky...
        let exec_nix_store_res_rc = Rc::new(ExecNixStoreRes {
            nix_store_path: PathBuf::from(""),
            res: Ok(Rc::clone(&nix_store_res_rc)),
        });
        let search_for_this_menu_item = create_search_for_this_menu_item(
            builder.clone(),
            tree_view.clone(),
            exec_nix_store_res_rc,
        );
        menu.append(&search_for_this_menu_item);

        let (x, y) = event_button.get_position();
        if let Some((Some(tree_path), Some(tree_view_column), _, _)) =
            tree_view.get_path_at_pos(x as i32, y as i32)
        {
            if let Some(nix_query_entry) = gtk_tree_path_is_for_recurse_column(
                tree_view.clone(),
                tree_view_column,
                tree_path,
                Rc::clone(&nix_store_res_rc),
            ) {
                let go_to_first_instance_menu_item =
                    gtk::MenuItem::new_with_label("Go to first instance");

                go_to_first_instance_menu_item.connect_activate(
                    clone!(@strong nix_query_entry, @weak tree_view =>
                        move |_|
                            go_to_path_for_query_entry(tree_view, Rc::clone(&nix_store_res_rc), &nix_query_entry)
                    )
                );

                menu.append(&go_to_first_instance_menu_item);
            }
        }

        menu.set_property_attach_widget(Some(&tree_view.clone()));
        menu.show_all();
        menu.popup_at_pointer(Some(&event_button));
    }

    Inhibit(false)
}

fn connect_signals(
    builder: gtk::Builder,
    tree_view: gtk::TreeView,
    exec_nix_store_res: Rc<ExecNixStoreRes>,
) {
    // Only connect signals to the tree when we successfully ran
    // nix-store.
    if let Ok(nix_store_res_rc) = &exec_nix_store_res.res {
        let nix_store_res_rc_cloned: Rc<NixStoreRes> = Rc::clone(nix_store_res_rc);
        tree_view.connect_row_activated(move |tree_view_ref, tree_path, tree_view_column| {
            handle_row_activated(
                tree_view_ref.clone(),
                tree_path.clone(),
                tree_view_column.clone(),
                Rc::clone(&nix_store_res_rc_cloned),
            );
        });

        let nix_store_res_rc_cloned: Rc<NixStoreRes> = Rc::clone(nix_store_res_rc);
        tree_view.connect_button_press_event(move |tree_view_ref, event_button| {
            handle_button_press_event(
                builder.clone(),
                tree_view_ref.clone(),
                event_button.clone(),
                Rc::clone(&nix_store_res_rc_cloned),
            )
        });
    }
}

fn create_store(tree_view: gtk::TreeView) -> gtk::TreeStore {
    let tree_store: gtk::TreeStore =
        gtk::TreeStore::new(&[glib::types::Type::String, glib::types::Type::String]);

    tree_view.set_model(Some(&tree_store));

    tree_store
}

fn render_tree_store(
    builder: gtk::Builder,
    tree_view: gtk::TreeView,
    exec_nix_store_res: Rc<ExecNixStoreRes>,
) {
    let tree_store = create_store(tree_view);

    render_nix_store_res(builder.clone(), tree_store, exec_nix_store_res);
}

pub fn setup_tree_view(
    builder: gtk::Builder,
    exec_nix_store_res_rc: Rc<ExecNixStoreRes>,
) -> gtk::TreeView {
    let tree_view: gtk::TreeView = builder.get_object_expect("treeView");

    create_columns(tree_view.clone());

    connect_signals(builder.clone(), tree_view.clone(), exec_nix_store_res_rc);

    tree_view
}

fn clear(tree_view: gtk::TreeView) {
    // let none_tree_model: Option<&gtk::TreeModel> = None;
    // tree_view.set_model(none_tree_model);

    let option_tree_model: Option<gtk::TreeModel> = tree_view.get_model();
    let option_tree_store: Option<gtk::TreeStore> = option_tree_model
        .clone()
        .and_then(|tree_model: gtk::TreeModel| tree_model.downcast().ok());
    println!(
        "tree_model: {:?}, tree_store: {:?}",
        option_tree_model, option_tree_store
    );

    if let Some(tree_store) = option_tree_store {
        tree_store.clear();
    }
}

fn render_nix_store_res(
    builder: gtk::Builder,
    tree_store: gtk::TreeStore,
    nix_store_res: Rc<ExecNixStoreRes>,
) {
    match &nix_store_res.res {
        // Err(err) => render_nix_store_err(builder, &nix_store_res.nix_store_path, err),
        Err(err) => (),
        Ok(res) => insert_into_tree_store(tree_store, res),
    }
}

pub fn setup(builder: gtk::Builder, exec_nix_store_res_rc: Rc<ExecNixStoreRes>) {
    let tree_view = setup_tree_view(builder.clone(), Rc::clone(&exec_nix_store_res_rc));

    render_tree_store(
        builder.clone(),
        tree_view.clone(),
        Rc::clone(&exec_nix_store_res_rc),
    );

    // expand the first row of the tree view
    tree_view.expand_row(&gtk::TreePath::new_first(), false);
}
