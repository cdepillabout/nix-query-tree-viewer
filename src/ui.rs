mod builder;
mod css;
mod menu;
mod stack;
mod state;
mod statusbar;
mod toolbar;

pub mod prelude;

pub use state::{Message, SortOrder, State, ViewStyle};

use glib::clone;
use std::path::Path;
use std::thread;

use super::nix_query_tree::exec_nix_store::NixStoreErr;

use prelude::*;

fn render_nix_store_err(
    state: &State,
    nix_store_path: &Path,
    nix_store_err: &NixStoreErr,
) {
    statusbar::show_msg(
        state,
        &format!(
            "Error running `nix-store --query --tree {}`",
            nix_store_path.to_string_lossy()
        ),
    );

    let error_dialog: gtk::MessageDialog = state.get_error_dialog();
    let error_msg = &format!(
        "Error running `nix-store --query --tree {}`:\n\n{}",
        nix_store_path.to_string_lossy(),
        nix_store_err
    );
    error_dialog.set_property_secondary_text(Some(error_msg));
    error_dialog.run();
    error_dialog.hide();
}

fn search_for(state: &State, nix_store_path: &Path) {
    // nix-store --query --tree /nix/store/jymg0kanmlgbcv35wxd8d660rw0fawhv-hello-2.10.drv
    // nix-store --query --tree /nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10

    disable(state);

    statusbar::show_msg(
        state,
        &format!("Searching for {}...", nix_store_path.display()),
    );

    let nix_store_path_buf = nix_store_path.to_path_buf();
    thread::spawn(clone!(@strong state.sender as sender => move || {
        let exec_nix_store_res =
            super::nix_query_tree::exec_nix_store::run(&nix_store_path_buf);

        sender
            .send(Message::Display(exec_nix_store_res))
            .expect("sender is already closed.  This should never happen");
    }));
}

fn set_sort_order(state: &State, new_sort_order: SortOrder) {
    state.write_sort_order(new_sort_order);

    stack::change_sort_order(state);
}

pub fn set_view_style(state: &State, new_view_style: ViewStyle) {
    state.write_view_style(new_view_style);

    stack::change_view_style(state);
}

fn redisplay_data(state: &State) {
    statusbar::clear(state);
    stack::redisplay_data(state);
}

fn disable(state: &State) {
    stack::disable(state);
    toolbar::disable(state);
}

fn enable(state: &State) {
    stack::enable(state);
    toolbar::enable(state);
}

fn handle_msg_recv(state: &State, msg: Message) {
    enable(state);

    match msg {
        Message::Display(exec_nix_store_res) => match exec_nix_store_res.res {
            Err(nix_store_err) => {
                render_nix_store_err(
                    state,
                    &exec_nix_store_res.nix_store_path,
                    &nix_store_err,
                );
            }
            Ok(nix_store_res) => {
                state.write_nix_store_res(nix_store_res);
                redisplay_data(state);
            }
        },
    }
}

fn app_activate(app: &gtk::Application) {
    let (sender, receiver) =
        glib::MainContext::channel(glib::source::PRIORITY_DEFAULT);

    let state = State::new(app, &sender);

    let window: gtk::ApplicationWindow = state.get_app_win();
    window.set_application(Some(&state.app));

    css::setup(window.upcast_ref());
    menu::setup(&state);
    toolbar::setup(&state);
    stack::setup(&state);

    window.show_all();

    receiver.attach(None, clone!(@strong state => move |msg| {
        handle_msg_recv(&state, msg);
        glib::source::Continue(true)
    }));

    // Do the initial search and display the results.
    let opts = crate::opts::Opts::parse_from_args();
    search_for(&state, &opts.nix_store_path);
}

pub fn run() {
    let uiapp = gtk::Application::new(
        Some("com.github.cdepillabout.nix-query-tree-viewer"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(move |app| app_activate(&app));

    // uiapp.run(&env::args().collect::<Vec<_>>());
    uiapp.run(&[]);
}
