mod columns;
mod path;
mod signals;
mod store;

use core::cmp::Ordering;
use glib::clone;
use glib::translate::{ToGlibPtr};

use super::super::super::ui;
use super::super::prelude::*;

fn clear(state: &ui::State) {
    let tree_store = state.get_tree_store();
    tree_store.clear();
}

pub fn disable(state: &ui::State) {
    let tree_view: gtk::TreeView = state.get_tree_view();
    tree_view.set_sensitive(false);
}

pub fn enable(state: &ui::State) {
    let tree_view: gtk::TreeView = state.get_tree_view();
    tree_view.set_sensitive(true);
}

fn render_nix_store_res(state: &ui::State) {
    if let Some(res) = &*state.read_nix_store_res() {
        let tree_store = state.get_tree_store();
        store::insert(tree_store, res);
    }
}

pub fn setup(state: &ui::State) {
    columns::setup(state);

    signals::connect(state);
}

/// Low-level (unsafe) function for setting the sorting function.
fn set_sort_func<O: IsA<gtk::TreeSortable>>(
    tree_model_sort: &O,
    sort_func: Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> Ordering + 'static>,
) {
    let sort_func_data: Box<Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> Ordering + 'static>> =
        Box::new(sort_func);

    unsafe extern "C" fn sort_func_func(
        tree_model: *mut gtk_sys::GtkTreeModel,
        tree_iter_a: *mut gtk_sys::GtkTreeIter,
        tree_iter_b: *mut gtk_sys::GtkTreeIter,
        user_data: glib_sys::gpointer,
    ) -> i32 {
        let tree_model: gtk::TreeModel = glib::translate::from_glib_borrow(tree_model);
        let tree_iter_a: gtk::TreeIter = glib::translate::from_glib_borrow(tree_iter_a);
        let tree_iter_b: gtk::TreeIter = glib::translate::from_glib_borrow(tree_iter_b);
        let callback: &Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> Ordering + 'static> =
            &*(user_data as *mut _);

        let res = callback(&tree_model, &tree_iter_a, &tree_iter_b);

        match res {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    let tree_sortable: &gtk::TreeSortable = tree_model_sort.as_ref();
    let gtk_tree_sortable: *mut gtk_sys::GtkTreeSortable = tree_sortable.to_glib_none().0;

    unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
        let _callback: Box<Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> Ordering + 'static>> =
            Box::from_raw(data as *mut _);
    }

    unsafe {
        gtk_sys::gtk_tree_sortable_set_sort_func(
            gtk_tree_sortable,
            0,
            Some(sort_func_func as _),
            Box::into_raw(sort_func_data) as *mut std::ffi::c_void,
            Some(destroy_func as unsafe extern "C" fn(_: *mut std::ffi::c_void)),
        );
    }
}

pub fn change_sort_order(state: &ui::State) {
    let tree_model_sort = state.get_tree_model_sort();

    match *state.read_sort_order() {
        ui::SortOrder::NixStoreOrigOutput => {
            tree_model_sort.set_sort_column_id(gtk::SortColumn::Default, gtk::SortType::Ascending);
        }
        ui::SortOrder::AlphabeticalHash => {
            set_sort_function(state);
            tree_model_sort.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);
        }
        ui::SortOrder::AlphabeticalDrvName => {
            set_sort_function(state);
            tree_model_sort.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);
        }
    }
}

pub fn set_sort_function(state: &ui::State) {
    let tree_model_sort = state.get_tree_model_sort();

    set_sort_func(
        &tree_model_sort,
        Box::new(clone!(@strong state => move|tree_model, tree_model_sort_iter_a, tree_model_sort_iter_b| {
            let sort_order = *state.read_sort_order();
            if let Some(nix_store_res) = &*state.read_nix_store_res() {
                let tree_store: &gtk::TreeStore = tree_model.downcast_ref().expect("tree_model is not a tree_store");

                let child_iter_a = path::GtkChildTreeIter::new(tree_model_sort_iter_a.clone());
                let child_iter_b = path::GtkChildTreeIter::new(tree_model_sort_iter_b.clone());

                let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> =
                    child_iter_a.nix_store_res_lookup(tree_store.clone(), &nix_store_res);
                let option_nix_query_entry_b: Option<crate::nix_query_tree::NixQueryEntry> =
                    child_iter_b.nix_store_res_lookup(tree_store.clone(), &nix_store_res);

                match (option_nix_query_entry_a, option_nix_query_entry_b) {
                    (Some(nix_query_entry_a), Some(nix_query_entry_b)) => {
                        match sort_order {
                            ui::SortOrder::NixStoreOrigOutput => {
                                println!("The sort function should never be called when the sort order is NixStoreOrigOutput!!!");
                                Ordering::Equal
                            }
                            ui::SortOrder::AlphabeticalHash => {
                                nix_query_entry_a.cmp_hash(&nix_query_entry_b)
                            }
                            ui::SortOrder::AlphabeticalDrvName => {
                                nix_query_entry_a.cmp_drv_name(&nix_query_entry_b)
                            }
                        }
                    }
                    _ => panic!("Not able to get an ordering for one of the nix_query_entries.  This should never happen."),
                }
            } else {
                panic!("The nix_store_res in state hasn't been set yet.  This should never happen.");
            }
        }))
    );
}


pub fn redisplay_data(state: &ui::State) {
    clear(state);
    enable(state);

    render_nix_store_res(state);

    // expand the first row of the tree view
    state
        .get_tree_view()
        .expand_row(&gtk::TreePath::new_first(), false);
}
