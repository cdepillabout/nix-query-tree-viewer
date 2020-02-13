
use crate::nix_query_tree::{NixQueryEntry, NixQueryTree, Recurse};
use crate::nix_query_tree::exec_nix_store::{NixStoreRes};
use crate::tree;
use std::collections::VecDeque;
use super::columns::Column;
use super::super::super::prelude::*;

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

pub fn goto(tree_view: gtk::TreeView, first_path: &tree::Path) {
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
    nix_query_tree: &NixQueryTree,
    tree_path: gtk::TreePath,
) -> Option<NixQueryEntry> {
    let path = gtk_tree_path_to_tree_path(tree_path.clone());
    nix_query_tree.lookup(path.clone()).cloned()
}

fn nix_store_res_lookup_gtk_path(
    nix_store_res: &NixStoreRes,
    tree_path: gtk::TreePath,
) -> Option<NixQueryEntry> {
    nix_query_tree_lookup_gtk_path(&nix_store_res.tree, tree_path)
}

pub fn is_for_recurse_column(
    tree_view: gtk::TreeView,
    tree_view_column: gtk::TreeViewColumn,
    tree_path: gtk::TreePath,
    nix_store_res_rc: &NixStoreRes,
) -> Option<NixQueryEntry> {
    let option_column = Column::from_gtk(tree_view.clone(), tree_view_column.clone());
    let option_nix_query_entry_is_recurse =
        nix_store_res_lookup_gtk_path(nix_store_res_rc, tree_path.clone())
            .filter(|nix_query_entry| nix_query_entry.1 == Recurse::Yes);

    match (option_column, option_nix_query_entry_is_recurse) {
        (Some(Column::Recurse), Some(nix_query_entry)) => Some(nix_query_entry),
        _ => None,
    }
}

