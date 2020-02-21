use glib::clone;

use super::super::super::super::ui;
use super::super::super::prelude::*;
use super::path;
use crate::nix_query_tree::exec_nix_store::NixStoreRes;
use crate::nix_query_tree::NixQueryEntry;

fn toggle_row_expanded(
    state: &ui::State,
    tree_path: &gtk::TreePath,
    recurse: bool,
) {
    let tree_view = state.get_tree_view();
    if tree_view.row_expanded(tree_path) {
        tree_view.collapse_row(tree_path);
    } else {
        tree_view.expand_row(tree_path, recurse);
    }
}

// Warning: This function assumes that nix_query_entry actually exists in NixStoreRes
fn go_to_path_for_query_entry(
    state: &ui::State,
    nix_store_res: &NixStoreRes,
    nix_query_entry: &NixQueryEntry,
) {
    let option_first_path =
        nix_store_res.lookup_first_query_entry(&nix_query_entry);
    match option_first_path {
        None => panic!(
            "Nothing in our map for this drv.  This should hever happen."
        ),
        Some(first_path) => {
            path::goto(state, &first_path);
        }
    }
}

fn go_to_curr_path_for_query_entry(
    state: &ui::State,
    nix_query_entry: &NixQueryEntry,
) {
    if let Some(nix_store_res) = &*state.read_nix_store_res() {
        go_to_path_for_query_entry(state, nix_store_res, nix_query_entry);
    }
}

fn handle_row_activated(
    state: &ui::State,
    tree_path: gtk::TreePath,
    tree_view_column: &gtk::TreeViewColumn,
) {
    if let Some(nix_store_res) = &*state.read_nix_store_res() {
        let parent_tree_path = path::GtkParentTreePath::new(tree_path.clone());
        match path::is_for_recurse_column_parent(
            state,
            tree_view_column,
            &parent_tree_path,
            nix_store_res,
        ) {
            Some(nix_query_entry) => go_to_path_for_query_entry(
                state,
                nix_store_res,
                &nix_query_entry,
            ),
            _ => toggle_row_expanded(state, &tree_path, false),
        }
    }
}

fn handle_copy_drv_path_menu_item_activated(
    state: &ui::State,
    nix_query_entry: &NixQueryEntry,
) {
    let tree_view = state.get_tree_view();
    if let Some(display) = tree_view.get_display() {
        if let Some(clipboard) = gtk::Clipboard::get_default(&display) {
            clipboard.set_text(&nix_query_entry.to_string_lossy());
            clipboard.store();
        }
    }
}

fn create_copy_drv_path_menu_item(
    state: &ui::State,
    menu: &gtk::Menu,
    event_button: &gdk::EventButton,
    nix_store_res: &NixStoreRes,
) {
    if let Some(nix_query_entry) = path::nix_query_entry_for_event_button(
        state,
        event_button,
        nix_store_res,
    ) {
        let copy_drv_path_menu_item =
            gtk::MenuItem::new_with_label("Copy drv path");

        copy_drv_path_menu_item.connect_activate(
            clone!(@strong state, @strong nix_query_entry => move |_| {
                handle_copy_drv_path_menu_item_activated(&state, &nix_query_entry);
            }),
        );

        menu.append(&copy_drv_path_menu_item);
    }
}

fn handle_search_for_this_menu_item_activated(
    state: &ui::State,
    nix_query_entry: &NixQueryEntry,
) {
    ui::search_for(state, &nix_query_entry);
}

fn create_search_for_this_menu_item(
    state: &ui::State,
    menu: &gtk::Menu,
    event_button: &gdk::EventButton,
    nix_store_res: &NixStoreRes,
) {
    if let Some(nix_query_entry) = path::nix_query_entry_for_event_button(
        state,
        event_button,
        nix_store_res,
    ) {
        let search_for_this_menu_item =
            gtk::MenuItem::new_with_label("Search for this");

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
    menu: &gtk::Menu,
    event_button: &gdk::EventButton,
    nix_store_res: &NixStoreRes,
) {
    if let Some(nix_query_entry) = path::is_event_button_for_recurse_column(
        state,
        event_button,
        nix_store_res,
    ) {
        let goto_first_instance_menu_item =
            gtk::MenuItem::new_with_label("Go to tree instance");

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
    tree_view: &gtk::TreeView,
    event_button: &gdk::EventButton,
) -> Inhibit {
    if let Some(nix_store_res) = &*state.read_nix_store_res() {
        if event_button.get_event_type() == gdk::EventType::ButtonPress
            && event_button.get_button() == 3
        {
            let menu: gtk::Menu = gtk::Menu::new();

            create_copy_drv_path_menu_item(
                state,
                &menu,
                event_button,
                nix_store_res,
            );

            create_search_for_this_menu_item(
                state,
                &menu,
                event_button,
                nix_store_res,
            );

            create_goto_first_instance_menu_item(
                state,
                &menu,
                event_button,
                nix_store_res,
            );

            // only show the menu if there is at least one child
            if menu.get_children().len() >= 1 {
                menu.set_property_attach_widget(Some(&tree_view.clone()));
                menu.show_all();
                menu.popup_at_pointer(Some(&event_button));
            }
        }
    }

    Inhibit(false)
}

pub fn connect(state: &ui::State) {
    state.get_tree_view().connect_row_activated(
        clone!(@strong state => move |_, tree_path, tree_view_column| {
            handle_row_activated(
                &state,
                tree_path.clone(),
                tree_view_column,
            );
        }),
    );

    state.get_tree_view().connect_button_press_event(
        clone!(@strong state => move |tree_view_ref, event_button| {
            handle_button_press_event(
                &state,
                tree_view_ref,
                event_button,
            )
        }),
    );
}
