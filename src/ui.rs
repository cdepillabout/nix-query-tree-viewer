mod builder;
mod statusbar;
mod switcher;

pub mod prelude;

use glib::clone;
use std::path::Path;
use std::rc::Rc;

use super::nix_query_tree::exec_nix_store::{ExecNixStoreRes, NixStoreErr};

use prelude::*;

fn connect_menu_buttons(app: gtk::Application, builder: gtk::Builder) {
    let about_menu_item: gtk::MenuItem = builder.get_object_expect("aboutMenuItem");
    let about_dialog: gtk::AboutDialog = builder.get_object_expect("aboutDialog");
    about_menu_item.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    let quit_menu_item: gtk::MenuItem = builder.get_object_expect("quitMenuItem");
    quit_menu_item.connect_activate(clone!(@weak app => move |_| {
        app.quit();
    }));
}

fn render_nix_store_err(builder: gtk::Builder, nix_store_path: &Path, nix_store_err: &NixStoreErr) {
    let error_dialog: gtk::MessageDialog = builder.get_object_expect("errorDialog");
    let error_msg = &format!(
        "Error running `nix-store --query --tree {}`:\n\n{}",
        nix_store_path.to_string_lossy(),
        nix_store_err
    );
    error_dialog.set_property_secondary_text(Some(error_msg));
    error_dialog.run();
    error_dialog.destroy();
    statusbar::show_msg(
        builder,
        &format!(
            "Error running `nix-store --query --tree {}`",
            nix_store_path.to_string_lossy()
        ),
    );
}

fn create_builder() -> gtk::Builder {
    let glade_src = include_str!("../glade/ui.glade");
    gtk::Builder::new_from_string(glade_src)
}

fn setup_css(window: gtk::Window) {
    let screen: gdk::Screen = match window.get_screen() {
        Some(screen) => screen,
        None => {
            println!("Failed to get the screen for window.");
            return;
        }
    };
    let css_provider = gtk::CssProvider::new();
    let css_src = include_str!("../style/style.css");
    match css_provider.load_from_data(css_src.as_bytes()) {
        Err(err) => println!("Failed to load css provider from data: {}", err),
        Ok(_) => {
            gtk::StyleContext::add_provider_for_screen(
                &screen,
                &css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }
}

fn app_activate(exec_nix_store_res: ExecNixStoreRes, app: gtk::Application) {
    let builder = create_builder();

    let window: gtk::ApplicationWindow = builder.get_object_expect("appWindow");
    window.set_application(Some(&app));

    setup_css(window.clone().upcast());

    let exec_nix_store_res = Rc::new(exec_nix_store_res);

    switcher::setup(builder.clone(), exec_nix_store_res);

    connect_menu_buttons(app, builder);

    window.show_all();
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
