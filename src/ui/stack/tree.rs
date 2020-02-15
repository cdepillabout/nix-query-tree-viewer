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
    println!("render_nix_store_res, starting, about to lock...");
    if let Some(res) = &*state.read_nix_store_res() {
        let tree_store = state.get_tree_store();
        store::insert(tree_store, res);
    }
    println!("render_nix_store_res, ending, releasing lock");
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
    println!("\tset_sort_func, starting...");
    let sort_func_data: Box<Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> i32 + 'static>> =
        Box::new(sort_func);

    unsafe extern "C" fn sort_func_func(
        tree_model: *mut gtk_sys::GtkTreeModel,
        tree_iter_a: *mut gtk_sys::GtkTreeIter,
        tree_iter_b: *mut gtk_sys::GtkTreeIter,
        user_data: glib_sys::gpointer,
    ) -> i32 {
        println!("\t\tsort_func_func, starting...");
        let tree_model: gtk::TreeModel = glib::translate::from_glib_borrow(tree_model);
        let tree_iter_a: gtk::TreeIter = glib::translate::from_glib_borrow(tree_iter_a);
        let tree_iter_b: gtk::TreeIter = glib::translate::from_glib_borrow(tree_iter_b);
        let callback: &Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> i32 + 'static> =
            &*(user_data as *mut _);

        println!("\t\tsort_func_func, calling callback...");
        let res = callback(&tree_model, &tree_iter_a, &tree_iter_b);
        println!("\t\tsort_func_func, after callback , got {}", res);
        res
    }

    let x: &gtk::TreeSortable = tree_model_sort.as_ref();
    let y: *mut gtk_sys::GtkTreeSortable = x.to_glib_none().0;

    unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
        let _callback: Box<Box<dyn Fn(&gtk::TreeModel, &gtk::TreeIter, &gtk::TreeIter) -> i32 + 'static>> =
            Box::from_raw(data as *mut _);
    }

    println!("\tset_sort_func, before calling inner unsafe...");
    unsafe {
        gtk_sys::gtk_tree_sortable_set_sort_func(
            y,
            0,
            Some(sort_func_func as _),
            Box::into_raw(sort_func_data) as *mut std::ffi::c_void,
            Some(destroy_func as unsafe extern "C" fn(_: *mut std::ffi::c_void)),
        );
    }
    println!("\tset_sort_func, after calling inner unsafe")
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

    println!("redisplay_data, starting...");

    let tree_model_sort = state.get_tree_model_sort();

    set_sort_func(
        &tree_model_sort,
        Box::new(clone!(@strong state => move|tree_model, tree_model_sort_iter_a, tree_model_sort_iter_b| {
            println!("\t\t\tyo my rust func, starting, about to lock...");
            if let Some(nix_store_res) = &*state.read_nix_store_res() {
                println!("\t\t\tyo my rust func, after locking");
                let tree_store: &gtk::TreeStore = tree_model.downcast_ref().expect("tree_model is not a tree_store");

                let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> =
                        path::nix_store_res_lookup_gtk_tree_iter(&nix_store_res, tree_store.clone(), tree_model_sort_iter_a.clone());

                let option_nix_query_entry_b: Option<crate::nix_query_tree::NixQueryEntry> =
                        path::nix_store_res_lookup_gtk_tree_iter(&nix_store_res, tree_store.clone(), tree_model_sort_iter_b.clone());

                println!("\t\t\tyo my rust func, got nix store res thing...");
                println!("\t\t\tyo my rust func, nix_query_entry_a = {:?}", &option_nix_query_entry_a);
                println!("\t\t\tyo my rust func, nix_query_entry_b = {:?}", &option_nix_query_entry_b);

            // let o_res: Option<_> = {
            //     let mutex_guard_res: std::sync::MutexGuard<_> = state.nix_store_res.lock().unwrap();
            //     mutex_guard_res.deref().clone()
            // };
            // println!("\t\t\tyo my rust func, after locking");
            // if let Some(res) = o_res {

            //     let tree_store: &gtk::TreeStore = tree_model.downcast_ref().expect("tree_model is not a tree_store");

            //     let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> =
            //             path::nix_store_res_lookup_gtk_tree_iter(&res, tree_store.clone(), tree_model_sort_iter_a.clone());

            //     let option_nix_query_entry_b: Option<crate::nix_query_tree::NixQueryEntry> =
            //             path::nix_store_res_lookup_gtk_tree_iter(&res, tree_store.clone(), tree_model_sort_iter_b.clone());

            //     println!("\t\t\tyo my rust func, got nix store res thing...");
            //     println!("\t\t\tyo my rust func, nix_query_entry_a = {:?}", &option_nix_query_entry_a);
            //     println!("\t\t\tyo my rust func, nix_query_entry_b = {:?}", &option_nix_query_entry_b);

                // let tree_store = state.get_tree_store();

                // let tree_model_sort: &gtk::TreeModelSort = tree_model.dynamic_cast_ref().expect("YO NOT ACTUALLY TREE MODEL SORT");

                // let tree_store_iter_a: gtk::TreeIter = tree_model_sort.convert_iter_to_child_iter(tree_model_sort_iter_a);
                // let tree_store_iter_b: gtk::TreeIter = tree_model_sort.convert_iter_to_child_iter(tree_model_sort_iter_b);

                // let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> =
                //         path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_store_iter_a);
                // let option_nix_query_entry_b: Option<crate::nix_query_tree::NixQueryEntry> =
                //         path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_store_iter_b);

                // println!("\t\t\tyo my rust func, got nix store res thing...");
                // println!("\t\t\tyo my rust func, nix_query_entry_a = {:?}", &option_nix_query_entry_a);
                // println!("\t\t\tyo my rust func, nix_query_entry_b = {:?}", &option_nix_query_entry_b);
            }
            0
        }))
    );

    // tree_model_sort.set_sort_func(
    //     gtk::SortColumn::Index(0),
    //     clone!(@strong state => move |tree_model_sort, tree_model_sort_iter_a, tree_model_sort_iter_b| {
    //         println!("hello1");
    //         Ordering::Equal
    //     }),
    // );

    tree_model_sort.set_sort_column_id(gtk::SortColumn::Index(0), gtk::SortType::Ascending);


    // tree_model_sort.set_default_sort_func(clone!(@strong state => move |tree_model_sort, tree_model_sort_iter_a, tree_model_sort_iter_b| {
    //     println!("hello1");
        // if let Some(res) = &*state.nix_store_res.lock().unwrap() {

            // let tree_store = state.get_tree_store();

            // let tree_store_iter_a: gtk::TreeIter = tree_model_sort.convert_iter_to_child_iter(tree_model_sort_iter_a);
            // let tree_store_iter_b: gtk::TreeIter = tree_model_sort.convert_iter_to_child_iter(tree_model_sort_iter_b);

            // let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> =
            //         path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_store_iter_a);
            // let option_nix_query_entry_b: Option<crate::nix_query_tree::NixQueryEntry> =
            //         path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_store_iter_b);

            // println!("in set_default_sorc_func, got nix store res thing...");
            // println!("\tin set_default_sort_func, nix_query_entry_a = {:?}", &option_nix_query_entry_a);
            // println!("\tin set_default_sort_func, nix_query_entry_b = {:?}", &option_nix_query_entry_b);
        // }
        // Ordering::Equal
    // }));
    //     println!("in set_default_sorc_func...");
    //     if let Some(res) = &*state.nix_store_res.lock().unwrap() {
    //         println!("\tin set_default_sorc_func, got nix store res thing...");
    //         let option_nix_query_entry_a: Option<crate::nix_query_tree::NixQueryEntry> = path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_iter_a.clone());
    //         let option_nix_query_entry_b = path::nix_store_res_lookup_gtk_tree_iter(res, tree_store.clone(), tree_iter_b.clone());

    //         let option_path_a: Option<&std::path::Path> = option_nix_query_entry_a.as_ref().map(|x| x.path());
    //         let option_path_b: Option<&std::path::Path> = option_nix_query_entry_b.as_ref().map(|x| x.path());

    //         let option_gtk_tree_path_a: Option<gtk::TreePath> = tree_store.get_path(&tree_iter_a);
    //         let option_gtk_tree_path_b: Option<gtk::TreePath> = tree_store.get_path(&tree_iter_b);

    //         let option_my_tree_path_a: Option<crate::tree::Path> = option_gtk_tree_path_a.map(|p| path::gtk_tree_path_to_tree_path(p));
    //         let option_my_tree_path_b: Option<crate::tree::Path> = option_gtk_tree_path_b.map(|p| path::gtk_tree_path_to_tree_path(p));

    //         let option_blahblah_a: Option<_> = option_my_tree_path_a.clone().and_then(|p| res.tree.lookup(p));
    //         let option_blahblah_b: Option<_> = option_my_tree_path_b.clone().and_then(|p| res.tree.lookup(p));
    //         // nix_query_tree.lookup(path.clone()).cloned()

    //         let option_iter_str_a = tree_store.get_string_from_iter(&tree_iter_a).as_ref().map(|x| String::from(x.as_str()));
    //         let option_iter_str_b = tree_store.get_string_from_iter(&tree_iter_b).as_ref().map(|x| String::from(x.as_str()));

    //         println!("\tin set_default_sort_func, nix_query_entry_a = {:?}, iter = {:?}, my_tree_path = {:?}, blah = {:?}", &option_path_a, option_iter_str_a, option_my_tree_path_a, option_blahblah_a);
    //         println!("\tin set_default_sort_func, nix_query_entry_b = {:?}, iter = {:?}, my_tree_path = {:?}, blah = {:?}", &option_path_b, option_iter_str_b, option_my_tree_path_b, option_blahblah_b);
    //         println!("");

    //         match (option_nix_query_entry_a, option_nix_query_entry_b) {
    //             (Some(nix_query_entry_a), Some(nix_query_entry_b)) => {
    //                 nix_query_entry_a.path().cmp(nix_query_entry_b.path())
    //             }
    //             _ => Ordering::Equal,
    //         }
    //     } else {
    //         Ordering::Equal
    //     }
    // }));

    // tree_store.set_sort_column_id(gtk::SortColumn::Default, gtk::SortType::Ascending);

    println!("redisplay_data, before expand tree_view...");

    // expand the first row of the tree view
    state
        .get_tree_view()
        .expand_row(&gtk::TreePath::new_first(), false);

    println!("redisplay_data, after expand tree_view...");
}
