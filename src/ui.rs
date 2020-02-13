mod builder;
mod css;
mod menu;
mod stack;
mod state;
mod statusbar;

pub mod prelude;

pub use state::{Message, State};

use glib::clone;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

use super::nix_query_tree::exec_nix_store::{ExecNixStoreRes, NixStoreErr};

use prelude::*;


fn render_nix_store_err(state: &State, nix_store_path: &Path, nix_store_err: &NixStoreErr) {
    let error_dialog: gtk::MessageDialog = state.get_error_dialog();
    let error_msg = &format!(
        "Error running `nix-store --query --tree {}`:\n\n{}",
        nix_store_path.to_string_lossy(),
        nix_store_err
    );
    error_dialog.set_property_secondary_text(Some(error_msg));
    error_dialog.run();
    error_dialog.destroy();
    statusbar::show_msg(
        state,
        &format!(
            "Error running `nix-store --query --tree {}`",
            nix_store_path.to_string_lossy()
        ),
    );
}

fn search_for(state: &State, nix_store_path: &Path) {
    // nix-store --query --tree /nix/store/jymg0kanmlgbcv35wxd8d660rw0fawhv-hello-2.10.drv
    // nix-store --query --tree /nix/store/qy93dp4a3rqyn2mz63fbxjg228hffwyw-hello-2.10

    statusbar::show_msg(state, &format!("Searching for {}...", nix_store_path.display()));

    let nix_store_path_buf = nix_store_path.to_path_buf();
    thread::spawn(clone!(@strong state.sender as sender => move || {
        let exec_nix_store_res = super::nix_query_tree::exec_nix_store::run(&nix_store_path_buf);
        sender.send(Message::Display(exec_nix_store_res));
    }));
}

fn redisplay_data(state: &State) {
    statusbar::clear(state);
    stack::redisplay_data(state);
}

fn handle_msg_recv(state: &State, msg: Message) {
    match msg {
        Message::Display(exec_nix_store_res) => {
            match exec_nix_store_res.res {
                Err(_) => todo!(),
                Ok(nix_store_res) => {
                    *state.nix_store_res.lock().unwrap() = Some(nix_store_res);
                    redisplay_data(state);
                },
            }
        }
    }
}

fn app_activate(app: gtk::Application) {
    let (sender, receiver) = glib::MainContext::channel(glib::source::PRIORITY_DEFAULT);

    let state = State::new(app, sender);

    let window: gtk::ApplicationWindow = state.get_app_win();
    window.set_application(Some(&state.app));

    css::setup(window.clone().upcast());

    // let exec_nix_store_res = Arc::new(exec_nix_store_res);

    stack::setup(&state);

    menu::setup(&state);

    window.show_all();

    let state_clone = state.clone();
    receiver.attach(None, move |msg| {
        handle_msg_recv(&state_clone, msg);
        glib::source::Continue(true)
    });

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

    uiapp.connect_activate(move |app| app_activate(app.clone()));

    // uiapp.run(&env::args().collect::<Vec<_>>());
    uiapp.run(&[]);
}
