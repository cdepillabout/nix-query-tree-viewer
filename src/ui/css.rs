use super::prelude::*;

pub fn setup(window: &gtk::Window) {
    let screen: gdk::Screen = match window.get_screen() {
        Some(screen) => screen,
        None => {
            println!("Failed to get the screen for window.");
            return;
        }
    };
    let css_provider = gtk::CssProvider::new();
    let css_src = include_str!("../../style/style.css");
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
