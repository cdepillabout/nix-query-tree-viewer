mod builder;
mod css;
mod menu;
mod stack;
mod state;
mod statusbar;

pub mod prelude;

pub use state::{Message, State};

use std::path::Path;
use std::sync::Arc;

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

fn app_activate(exec_nix_store_res: ExecNixStoreRes, app: gtk::Application) {
    let (sender, receiver) = glib::MainContext::channel(glib::source::PRIORITY_DEFAULT);

    let state = State::new(app, sender);

    let window: gtk::ApplicationWindow = state.get_app_win();
    window.set_application(Some(&app));

    css::setup(window.clone().upcast());

    let exec_nix_store_res = Arc::new(exec_nix_store_res);

    stack::setup(&state, exec_nix_store_res);

    menu::connect_signals(state);

    window.show_all();

    receiver.attach(None, |Message::Display(exec_nix_store_res)| glib::source::Continue(true));
}

pub fn run(nix_store_res: ExecNixStoreRes) {
    let uiapp = gtk::Application::new(
        Some("com.github.cdepillabout.nix-query-tree-viewer"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(move |app| app_activate(nix_store_res.clone(), app.clone()));

    // uiapp.run(&env::args().collect::<Vec<_>>());
    uiapp.run(&[]);
}
