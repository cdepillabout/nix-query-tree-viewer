mod columns;
mod path;
mod signals;
mod store;

use core::cmp::Ordering;
use glib::clone;
use glib::translate::ToGlibPtr;
use std::ops::Deref;

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

// fn set_sort_func(
//     &self,
//     sort_func: Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>,
// ) {
//     let sort_func_data: Box_<Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> =
//         Box_::new(sort_func);
//     unsafe extern "C" fn sort_func_func(
//         row1: *mut gtk_sys::GtkListBoxRow,
//         row2: *mut gtk_sys::GtkListBoxRow,
//         user_data: glib_sys::gpointer,
//     ) -> libc::c_int {
//         let row1 = from_glib_borrow(row1);
//         let row2 = from_glib_borrow(row2);
//         let callback: &Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>> =
//             &*(user_data as *mut _);
//         let res = if let Some(ref callback) = *callback {
//             callback(&row1, &row2)
//         } else {
//             panic!("cannot get closure...")
//         };
//         res
//     }
//     let sort_func = if sort_func_data.is_some() {
//         Some(sort_func_func as _)
//     } else {
//         None
//     };
//     unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
//         let _callback: Box_<Option<Box_<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> =
//             Box_::from_raw(data as *mut _);
//     }
//     let destroy_call3 = Some(destroy_func as _);
//     unsafe {
//         gtk_sys::gtk_list_box_set_sort_func(
//             self.as_ref().to_glib_none().0,
//             sort_func,
//             Box_::into_raw(sort_func_data) as *mut _,
//             destroy_call3,
//         );
//     }
// }

fn set_sort_func<O: IsA<gtk::TreeSortable>>(
    tree_model_sort: &O,
    sort_func: Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> i32 + 'static>,
) {
    let sort_func_data: Box<Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> i32 + 'static>> =
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
        let callback: &Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> i32 + 'static> =
            &*(user_data as *mut _);

        let res = callback(&tree_model, &tree_iter_a, &tree_iter_b);
        res
    }

    let x: &gtk::TreeSortable = tree_model_sort.as_ref();
    let y: *mut gtk_sys::GtkTreeSortable = x.to_glib_none().0;

    unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
        let _callback: Box<Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> i32 + 'static>> =
            Box::from_raw(data as *mut _);
    }

    unsafe {
        gtk_sys::gtk_tree_sortable_set_sort_func(
            y,
            0,
            Some(sort_func_func as _),
            Box::into_raw(sort_func_data) as *mut std::ffi::c_void,
            Some(destroy_func as unsafe extern "C" fn(_: *mut std::ffi::c_void)),
        );
    }
}

// gtk_sys::gtk_tree_sortable_set_default_sort_func(
//     self.as_ref().to_glib_none().0,
//     Some(trampoline::<Self, F>),
//     into_raw(sort_func),
//     Some(destroy_closure::<Self, F>),
// )

// pub unsafe extern "C" fn gtk_tree_sortable_set_sort_func(
//     sortable: *mut GtkTreeSortable,
//     sort_func: GtkTreeIterCompareFunc,
//     user_data: gpointer,
//     destroy: GDestroyNotify
// )


pub fn redisplay_data(state: &ui::State) {
    clear(state);
    enable(state);


    render_nix_store_res(state);

    let tree_model_sort = state.get_tree_model_sort();

    // TODO: Uncomment this.
    // set_sort_func(
    //     &tree_model_sort,
    //     Box::new(clone!(@strong state => move|tree_model, tree_model_sort_iter_a, tree_model_sort_iter_b| {
    //         if let Some(nix_store_res) = &*state.read_nix_store_res() {
    //             let tree_store: &gtk::TreeStore = tree_model.downcast_ref().expect("tree_model is not a tree_store");

    //             let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> =
    //                     path::nix_store_res_lookup_gtk_tree_iter(&nix_store_res, tree_store.clone(), tree_model_sort_iter_a.clone());

    //             let option_nix_query_entry_b: Option<crate::nix_query_tree::NixQueryEntry> =
    //                     path::nix_store_res_lookup_gtk_tree_iter(&nix_store_res, tree_store.clone(), tree_model_sort_iter_b.clone());

    //             // println!("\t\t\tyo my rust func, got nix store res thing...");
    //             // println!("\t\t\tyo my rust func, nix_query_entry_a = {:?}", &option_nix_query_entry_a);
    //             // println!("\t\t\tyo my rust func, nix_query_entry_b = {:?}", &option_nix_query_entry_b);

    //             match (option_nix_query_entry_a, option_nix_query_entry_b) {
    //                 (Some(nix_query_entry_a), Some(nix_query_entry_b)) => {
    //                     match nix_query_entry_a.path().cmp(nix_query_entry_b.path()) {
    //                         Ordering::Less => -1,
    //                         Ordering::Equal => 0,
    //                         Ordering::Greater => 1,
    //                     }
    //                 }
    //                 _ => panic!("Not able to get an ordering for one of the nix_query_entries.  This should never happen."),
    //             }
    //         } else {
    //             panic!("The nix_store_res in state hasn't been set yet.  This should never happen.");
    //         }
    //     }))
    // );

    // TODO: When this is enabled, the right clicks no longer go to the right item.  Maybe I have
    // to use the child iter thing now???
    tree_model_sort.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);

    // expand the first row of the tree view
    state
        .get_tree_view()
        .expand_row(&gtk::TreePath::new_first(), false);
}
