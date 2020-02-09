mod builder;
mod css;
mod menu;
mod statusbar;
mod stack;

pub mod prelude;

use std::path::Path;
use std::sync::Arc;

use super::nix_query_tree::exec_nix_store::{ExecNixStoreRes, NixStoreErr};

use prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Message {
    Display(ExecNixStoreRes),
}

#[derive(Clone, Debug)]
struct State {
    app: gtk::Application,
    builder: gtk::Builder,
    sender: glib::Sender<Message>,
}

impl State {
    fn new(app: gtk::Application, sender: glib::Sender<Message>) -> Self {
        State {
            app,
            builder: builder::create(),
            sender,
        }
    }

    fn get_app_win(&self) -> gtk::ApplicationWindow {
        self.builder.get_object_expect("appWindow")
    }

    fn get_about_menu_item(&self) -> gtk::MenuItem {
        self.builder.get_object_expect("aboutMenuItem")
    }

    fn get_quit_menu_item(&self) -> gtk::MenuItem {
        self.builder.get_object_expect("quitMenuItem")
    }
    
    fn get_about_dialog(&self) -> gtk::AboutDialog {
        self.builder.get_object_expect("aboutDialog")
    }

    fn get_error_dialog(&self) -> gtk::MessageDialog {
        self.builder.get_object_expect("errorDialog")
    }

    fn get_raw_text_buffer(&self) -> gtk::TextBuffer {
        self.builder.get_object_expect("rawTextBuffer")
    }

    fn get_statusbar(&self) -> gtk::StatusBar {
        self.builder.get_object_expect("statusbar")
    }
}

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
