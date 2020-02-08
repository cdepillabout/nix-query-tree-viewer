
use gtk::prelude::*;
use super::prelude::*;

pub fn show_msg(builder: gtk::Builder, msg: &str) {
    let statusbar: gtk::Statusbar = builder.get_object_expect("statusbar");
    statusbar.remove_all(0);
    statusbar.push(0, msg);
}

