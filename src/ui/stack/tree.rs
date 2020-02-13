mod columns;
mod path;
mod signals;
mod store;

use super::super::prelude::*;
use super::super::super::ui;

fn setup_store(state: &ui::State) -> gtk::TreeStore {
    let tree_store: gtk::TreeStore =
        gtk::TreeStore::new(&[glib::types::Type::String, glib::types::Type::String]);

    state.get_tree_view().set_model(Some(&tree_store));

    tree_store
}

fn clear(state: &ui::State) {
    let tree_view = state.get_tree_view();

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

pub fn disable(state: &ui::State) {
    let tree_view: gtk::TreeView = state.get_tree_view();
    tree_view.set_sensitive(false);
}

pub fn enable(state: &ui::State) {
    let tree_view: gtk::TreeView = state.get_tree_view();
    tree_view.set_sensitive(true);
}

fn render_nix_store_res(
    state: &ui::State,
    tree_store: gtk::TreeStore,
) {
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
    let tree_store = setup_store(state);

    render_nix_store_res(state, tree_store);

    // expand the first row of the tree view
    state.get_tree_view().expand_row(&gtk::TreePath::new_first(), false);
}
