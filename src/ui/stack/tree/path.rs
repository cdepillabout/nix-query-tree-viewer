use super::super::super::prelude::*;
use super::super::super::super::ui;
use super::columns::Column;
use crate::nix_query_tree::exec_nix_store::NixStoreRes;
use crate::nix_query_tree::{NixQueryEntry, NixQueryTree, Recurse};
use crate::tree;
use std::collections::VecDeque;

pub fn gtk_tree_path_to_tree_path(gtk_tree_path: &gtk::TreePath) -> tree::Path {
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

pub fn goto(tree_view: gtk::TreeView, first_path: &tree::Path) {
    let first_gtk_path = tree_path_to_gtk_tree_path(first_path);
    let col = tree_view.get_column(Column::Item as i32);

    // Open recursively upward from this new path.
    tree_view.expand_to_path(&first_gtk_path);

    // Scroll to the newly opened path.
    tree_view.scroll_to_cell(
        Some(&first_gtk_path),
        col.as_ref(),
        true,
        0.5,
        0.5,
    );

    let tree_selection: gtk::TreeSelection = tree_view.get_selection();
    // Select the newly opened path.
    tree_selection.select_path(&first_gtk_path);
}

fn nix_query_tree_lookup_gtk_path(
    nix_query_tree: &NixQueryTree,
    tree_path: gtk::TreePath,
) -> Option<NixQueryEntry> {
    let path = gtk_tree_path_to_tree_path(&tree_path);
    nix_query_tree.lookup(path.clone()).cloned()
}

pub fn nix_store_res_lookup_gtk_tree_iter(
    nix_store_res: &NixStoreRes,
    tree_store: gtk::TreeStore,
    tree_iter: gtk::TreeIter,
) -> Option<NixQueryEntry> {
    let tree_path = tree_store.get_path(&tree_iter)?;
    nix_query_tree_lookup_gtk_path(&nix_store_res.tree, tree_path)
}

fn nix_store_res_lookup_gtk_path(
    nix_store_res: &NixStoreRes,
    tree_path: gtk::TreePath,
) -> Option<NixQueryEntry> {
    nix_query_tree_lookup_gtk_path(&nix_store_res.tree, tree_path)
}

fn event_button_to_tree_path_column(
    tree_view: gtk::TreeView,
    event_button: gdk::EventButton,
) -> Option<(gtk::TreePath, gtk::TreeViewColumn)> {
    let (x, y) = event_button.get_position();
    if let Some((Some(tree_path), Some(tree_view_column), _, _)) =
        tree_view.get_path_at_pos(x as i32, y as i32)
    {
        Some((tree_path, tree_view_column))
    } else {
        None
    }
}

fn event_button_to_real_tree_path_column(
    state: &ui::State,
    event_button: gdk::EventButton,
) -> Option<(gtk::TreePath, gtk::TreeViewColumn)> {
    let tree_view = state.get_tree_view();
    let tree_model_sort = state.get_tree_model_sort();
    event_button_to_tree_path_column(tree_view, event_button)
        .map(|(tree_path, tree_view_column)| {
            println!("In event_button_to_real_tree_path_column, in map callback, tree_path: {:?}", gtk_tree_path_to_tree_path(&tree_path));
            let child_tree_path = tree_model_sort
                .convert_path_to_child_path(&tree_path)
                .expect("tree_path should always be able to be converted to a child_tree_path");
            println!("In event_button_to_real_tree_path_column, in map callback, child_tree_path: {:?}", gtk_tree_path_to_tree_path(&child_tree_path));
            (child_tree_path, tree_view_column)
        })
}

fn event_button_to_real_tree_path(
    state: &ui::State,
    event_button: gdk::EventButton,
) -> Option<gtk::TreePath> {
    event_button_to_real_tree_path_column(state, event_button)
        .map(|tuple| tuple.0)
}

fn is_for_recurse_column(
    state: &ui::State,
    tree_view_column: gtk::TreeViewColumn,
    tree_path: gtk::TreePath,
    nix_store_res: &NixStoreRes,
) -> Option<NixQueryEntry> {
    let tree_view = state.get_tree_view();
    let option_column =
        Column::from_gtk(tree_view.clone(), tree_view_column.clone());
    let option_nix_query_entry_is_recurse =
        nix_store_res_lookup_gtk_path(nix_store_res, tree_path.clone())
            .filter(|nix_query_entry| nix_query_entry.1 == Recurse::Yes);

    match (option_column, option_nix_query_entry_is_recurse) {
        (Some(Column::Recurse), Some(nix_query_entry)) => Some(nix_query_entry),
        _ => None,
    }
}

pub fn is_for_real_recurse_column(
    state: &ui::State,
    tree_view_column: gtk::TreeViewColumn,
    tree_path: gtk::TreePath,
    nix_store_res: &NixStoreRes,
) -> Option<NixQueryEntry> {
    let tree_model_sort = state.get_tree_model_sort();
    let child_tree_path = tree_model_sort
        .convert_path_to_child_path(&tree_path)
        .expect("tree_path should always be able to be converted to a child_tree_path");
    is_for_recurse_column(state, tree_view_column, child_tree_path, nix_store_res)
}

pub fn is_event_button_for_recurse_column(
    state: &ui::State,
    event_button: gdk::EventButton,
    nix_store_res: &NixStoreRes,
) -> Option<NixQueryEntry> {
    event_button_to_real_tree_path_column(state, event_button).and_then(
        |(tree_path, tree_view_column)| {
            is_for_recurse_column(
                state,
                tree_view_column,
                tree_path,
                nix_store_res,
            )
        },
    )
}

pub fn nix_query_entry_for_event_button(
    state: &ui::State,
    event_button: gdk::EventButton,
    nix_store_res: &NixStoreRes,
) -> Option<NixQueryEntry> {
    event_button_to_real_tree_path(state, event_button).and_then(
        |tree_path| {
            println!("In nix_query_entry_for_event_button, in and_then callback, tree_path: {:?}", gtk_tree_path_to_tree_path(&tree_path));
            nix_store_res_lookup_gtk_path(nix_store_res, tree_path.clone())
        },
    )
}
