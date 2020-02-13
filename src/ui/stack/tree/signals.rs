use glib::clone;
use std::path::Path;

use super::super::super::super::ui;
use super::super::super::prelude::*;
use super::super::super::stack;
use super::path;
use crate::nix_query_tree::exec_nix_store::NixStoreRes;
use crate::nix_query_tree::NixQueryEntry;

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
    let option_first_path = nix_store_res.lookup_first_query_entry(&nix_query_entry);
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
            Some(nix_query_entry) => {
                go_to_path_for_query_entry(tree_view, nix_store_res, &nix_query_entry)
            }
            _ => toggle_row_expanded(tree_view.clone(), tree_path.clone(), false),
        }
    }
}

fn handle_search_for_this_menu_item_activated(state: &ui::State, nix_query_entry: &NixQueryEntry) {
    stack::disable(state);

    ui::search_for(state, &nix_query_entry.path());
}

fn create_search_for_this_menu_item(
    state: &ui::State,
    tree_view: gtk::TreeView,
    menu: gtk::Menu,
    event_button: gdk::EventButton,
    nix_store_res: &NixStoreRes,
) {
    if let Some(nix_query_entry) =
        path::nix_query_entry_for_event_button(tree_view.clone(), event_button, nix_store_res)
    {
        let search_for_this_menu_item = gtk::MenuItem::new_with_label("Search for this");

        search_for_this_menu_item.connect_activate(
            clone!(@strong state, @strong nix_query_entry => move |_| {
                handle_search_for_this_menu_item_activated(&state, &nix_query_entry);
            }),
        );

        menu.append(&search_for_this_menu_item);
    }
}

fn create_goto_first_instance_menu_item(
    state: &ui::State,
    tree_view: gtk::TreeView,
    menu: gtk::Menu,
    event_button: gdk::EventButton,
    nix_store_res: &NixStoreRes,
) {
    if let Some(nix_query_entry) =
        path::is_event_button_for_recurse_column(tree_view.clone(), event_button, nix_store_res)
    {
        let goto_first_instance_menu_item = gtk::MenuItem::new_with_label("Go to first instance");

        goto_first_instance_menu_item.connect_activate(
            clone!(@strong state, @strong nix_query_entry =>
                move |_|
                    go_to_curr_path_for_query_entry(&state, &nix_query_entry)
            ),
        );

        menu.append(&goto_first_instance_menu_item);
    }
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

            create_search_for_this_menu_item(
                state,
                tree_view.clone(),
                menu.clone(),
                event_button.clone(),
                nix_store_res,
            );

            create_goto_first_instance_menu_item(
                state,
                tree_view.clone(),
                menu.clone(),
                event_button.clone(),
                nix_store_res,
            );

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
        }),
    );

    state.get_tree_view().connect_button_press_event(
        clone!(@strong state => move |tree_view_ref, event_button| {
            handle_button_press_event(
                &state,
                tree_view_ref.clone(),
                event_button.clone(),
            )
        }),
    );
}
