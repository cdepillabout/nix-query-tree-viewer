use glib::clone;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use crate::nix_query_tree;
use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes, NixStoreRes};
use crate::nix_query_tree::{NixQueryEntry};
use super::path;
use super::statusbar;
use super::super::tree;
use super::super::super::prelude::*;
use super::super::super::stack;
use super::super::super::super::ui;

fn toggle_row_expanded(tree_view: gtk::TreeView, tree_path: gtk::TreePath, recurse: bool) {
    if tree_view.row_expanded(&tree_path) {
        tree_view.collapse_row(&tree_path);
    } else {
        tree_view.expand_row(&tree_path, recurse);
    }
}

// Warning: This function assumes that nix_query_entry actually exists in NixStoreRes
fn go_to_path_for_query_entry(
    tree_view: gtk::TreeView,
    nix_store_res: &NixStoreRes,
    nix_query_entry: &NixQueryEntry,
) {
    let option_first_path =
        nix_store_res.lookup_first_query_entry(&nix_query_entry);
    match option_first_path {
        None => panic!("Nothing in our map for this drv.  This should hever happen."),
        Some(first_path) => {
            path::goto(tree_view, &first_path);
        }
    }
}

fn go_to_curr_path_for_query_entry(state: &ui::State, nix_query_entry: &NixQueryEntry) {
    if let Some(nix_store_res) = &*state.nix_store_res.lock().unwrap() {
        let tree_view = state.get_tree_view();
        go_to_path_for_query_entry(tree_view, nix_store_res, nix_query_entry);
    }
}

fn handle_row_activated(
    state: &ui::State,
    tree_view: gtk::TreeView,
    tree_path: gtk::TreePath,
    tree_view_column: gtk::TreeViewColumn,
) {
    if let Some(nix_store_res) = &*state.nix_store_res.lock().unwrap() {
        match path::is_for_recurse_column(
            tree_view.clone(),
            tree_view_column.clone(),
            tree_path.clone(),
            nix_store_res,
        ) {
            Some(nix_query_entry) => go_to_path_for_query_entry(
                tree_view,
                nix_store_res,
                &nix_query_entry,
            ),
            _ => toggle_row_expanded(tree_view.clone(), tree_path.clone(), false),
        }
    }
}

fn handle_search_for_this_menu_item_activated(state: &ui::State) {
    stack::disable(state);

    ui::search_for(state, Path::new("/nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10"));
}

fn create_search_for_this_menu_item(state: &ui::State) -> gtk::MenuItem {
    let search_for_this_menu_item = gtk::MenuItem::new_with_label("Search for this");

    search_for_this_menu_item.connect_activate(clone!(@strong state => move |_| {
        handle_search_for_this_menu_item_activated(&state);
    }));

    search_for_this_menu_item
}

fn handle_button_press_event(
    state: &ui::State,
    tree_view: gtk::TreeView,
    event_button: gdk::EventButton,
) -> Inhibit {
    if let Some(nix_store_res) = &*state.nix_store_res.lock().unwrap() {
        if event_button.get_event_type() == gdk::EventType::ButtonPress
            && event_button.get_button() == 3
        {
            let menu: gtk::Menu = gtk::Menu::new();
            let search_for_this_menu_item = create_search_for_this_menu_item(state);
            menu.append(&search_for_this_menu_item);

            let (x, y) = event_button.get_position();
            if let Some((Some(tree_path), Some(tree_view_column), _, _)) =
                tree_view.get_path_at_pos(x as i32, y as i32)
            {
                if let Some(nix_query_entry) = path::is_for_recurse_column(
                    tree_view.clone(),
                    tree_view_column,
                    tree_path,
                    nix_store_res,
                ) {
                    let go_to_first_instance_menu_item =
                        gtk::MenuItem::new_with_label("Go to first instance");

                    go_to_first_instance_menu_item.connect_activate(
                        clone!(@strong state, @strong nix_query_entry =>
                            move |_|
                                go_to_curr_path_for_query_entry(&state, &nix_query_entry)
                        )
                    );

                    menu.append(&go_to_first_instance_menu_item);
                }
            }

            menu.set_property_attach_widget(Some(&tree_view.clone()));
            menu.show_all();
            menu.popup_at_pointer(Some(&event_button));
        }
    }

    Inhibit(false)
}

pub fn connect(state: &ui::State) {
    state.get_tree_view().connect_row_activated(
        clone!(@strong state => move |tree_view_ref, tree_path, tree_view_column| {
            handle_row_activated(
                &state,
                tree_view_ref.clone(),
                tree_path.clone(),
                tree_view_column.clone(),
            );
        })
    );

    state.get_tree_view().connect_button_press_event(
        clone!(@strong state => move |tree_view_ref, event_button| {
            handle_button_press_event(
                &state,
                tree_view_ref.clone(),
                event_button.clone(),
            )
        })
    );
}

