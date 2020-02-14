mod columns;
mod path;
mod signals;
mod store;

use core::cmp::Ordering;
use glib::clone;

use super::super::super::ui;
use super::super::prelude::*;

fn setup_store(state: &ui::State) -> gtk::TreeStore {
    let tree_store: gtk::TreeStore = gtk::TreeStore::new(&[
        glib::types::Type::String,
        glib::types::Type::String,
    ]);

    state.get_tree_view().set_model(Some(&tree_store));

    tree_store
}

fn clear(state: &ui::State) {
    let tree_view = state.get_tree_view();

    let option_tree_model: Option<gtk::TreeModel> = tree_view.get_model();
    let option_tree_store: Option<gtk::TreeStore> = option_tree_model
        .clone()
        .and_then(|tree_model: gtk::TreeModel| tree_model.downcast().ok());

    if let Some(tree_store) = option_tree_store {
        tree_store.clear();
    }
}

pub fn disable(state: &ui::State) {
    let tree_view: gtk::TreeView = state.get_tree_view();
    tree_view.set_sensitive(false);
}

pub fn enable(state: &ui::State) {
    let tree_view: gtk::TreeView = state.get_tree_view();
    tree_view.set_sensitive(true);
}

fn render_nix_store_res(state: &ui::State, tree_store: gtk::TreeStore) {
    if let Some(res) = &*state.nix_store_res.lock().unwrap() {
        store::insert(tree_store, res);
    }
}

pub fn setup(state: &ui::State) {
    columns::setup(state);

    signals::connect(state);
}

pub fn redisplay_data(state: &ui::State) {
    clear(state);
    enable(state);

    // TODO: Need to use a TreeModelSort and not sort the underlying data...
    let tree_store = setup_store(state);

    render_nix_store_res(state, tree_store.clone());

    tree_store.set_default_sort_func(clone!(@strong state => move |tree_store, tree_iter_a, tree_iter_b| {
        println!("in set_default_sorc_func...");
        if let Some(res) = &*state.nix_store_res.lock().unwrap() {
            println!("\tin set_default_sorc_func, got nix store res thing...");
            let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> = path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_iter_a.clone());
            let option_nix_query_entry_b = path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_iter_b.clone());

            let option_path_a: Option<&std::path::Path> = option_nix_query_entry_a.as_ref().map(|x| x.path());
            let option_path_b: Option<&std::path::Path> = option_nix_query_entry_b.as_ref().map(|x| x.path());

            let option_gtk_tree_path_a: Option<gtk::TreePath> = tree_store.get_path(&tree_iter_a);
            let option_gtk_tree_path_b: Option<gtk::TreePath> = tree_store.get_path(&tree_iter_b);

            let option_my_tree_path_a: Option<crate::tree::Path> = option_gtk_tree_path_a.map(|p| path::gtk_tree_path_to_tree_path(p));
            let option_my_tree_path_b: Option<crate::tree::Path> = option_gtk_tree_path_b.map(|p| path::gtk_tree_path_to_tree_path(p));

            let option_blahblah_a: Option<_> = option_my_tree_path_a.clone().and_then(|p| res.tree.lookup(p));
            let option_blahblah_b: Option<_> = option_my_tree_path_b.clone().and_then(|p| res.tree.lookup(p));
            // nix_query_tree.lookup(path.clone()).cloned()

            let option_iter_str_a = tree_store.get_string_from_iter(&tree_iter_a).as_ref().map(|x| String::from(x.as_str()));
            let option_iter_str_b = tree_store.get_string_from_iter(&tree_iter_b).as_ref().map(|x| String::from(x.as_str()));

            println!("\tin set_default_sort_func, nix_query_entry_a = {:?}, iter = {:?}, my_tree_path = {:?}, blah = {:?}", &option_path_a, option_iter_str_a, option_my_tree_path_a, option_blahblah_a);
            println!("\tin set_default_sort_func, nix_query_entry_b = {:?}, iter = {:?}, my_tree_path = {:?}, blah = {:?}", &option_path_b, option_iter_str_b, option_my_tree_path_b, option_blahblah_b);
            println!("");

            match (option_nix_query_entry_a, option_nix_query_entry_b) {
                (Some(nix_query_entry_a), Some(nix_query_entry_b)) => {
                    nix_query_entry_a.path().cmp(nix_query_entry_b.path())
                }
                _ => Ordering::Equal,
            }
        } else {
            Ordering::Equal
        }
    }));

    tree_store.set_sort_column_id(gtk::SortColumn::Default, gtk::SortType::Ascending);

    // expand the first row of the tree view
    state
        .get_tree_view()
        .expand_row(&gtk::TreePath::new_first(), false);
}
