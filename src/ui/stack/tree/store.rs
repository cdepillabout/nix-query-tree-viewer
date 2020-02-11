
use crate::nix_query_tree::{NixQueryDrv, NixQueryEntry, NixQueryTree, Recurse};
use crate::nix_query_tree::exec_nix_store::{NixStoreRes};
use crate::tree::Tree;

use super::columns;
use super::super::super::prelude::*;

fn insert_child(
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
        &columns::INDICIES
            .iter()
            .map(|&i| i as u32)
            .collect::<Vec<u32>>(),
        &[&drv_str, &recurse_str],
    );
    insert_children(tree_store, this_iter, children);
}

fn insert_children(
    tree_store: gtk::TreeStore,
    parent: gtk::TreeIter,
    children: &[Tree<NixQueryEntry>],
) {
    for child in children {
        let _: &Tree<NixQueryEntry> = child;
        insert_child(tree_store.clone(), Some(parent.clone()), child)
    }
}

pub fn insert(tree_store: gtk::TreeStore, nix_store_res: &NixStoreRes) {
    let nix_query_tree: &NixQueryTree = &nix_store_res.tree;
    let tree: &Tree<NixQueryEntry> = &nix_query_tree.0;
    insert_child(tree_store, None, tree);
}

