use std::sync::{Arc, Mutex};

use super::super::nix_query_tree::exec_nix_store::{
    ExecNixStoreRes, NixStoreRes,
};
use super::builder;
use super::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Display(ExecNixStoreRes),
}

#[derive(Clone, Debug)]
pub struct State {
    pub app: gtk::Application,
    pub builder: gtk::Builder,
    pub sender: glib::Sender<Message>,
    pub nix_store_res: Arc<Mutex<Option<NixStoreRes>>>,
}

impl State {
    pub fn new(app: gtk::Application, sender: glib::Sender<Message>) -> Self {
        State {
            app,
            builder: builder::create(),
            sender,
            nix_store_res: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_app_win(&self) -> gtk::ApplicationWindow {
        self.builder.get_object_expect("appWindow")
    }

    pub fn get_about_menu_item(&self) -> gtk::MenuItem {
        self.builder.get_object_expect("aboutMenuItem")
    }

    pub fn get_quit_menu_item(&self) -> gtk::MenuItem {
        self.builder.get_object_expect("quitMenuItem")
    }

    pub fn get_about_dialog(&self) -> gtk::AboutDialog {
        self.builder.get_object_expect("aboutDialog")
    }

    pub fn get_error_dialog(&self) -> gtk::MessageDialog {
        self.builder.get_object_expect("errorDialog")
    }

    pub fn get_raw_text_buffer(&self) -> gtk::TextBuffer {
        self.builder.get_object_expect("rawTextBuffer")
    }

    pub fn get_statusbar(&self) -> gtk::Statusbar {
        self.builder.get_object_expect("statusbar")
    }

    pub fn get_tree_view(&self) -> gtk::TreeView {
        self.builder.get_object_expect("treeView")
    }

    pub fn get_tree_view_column_item(&self) -> gtk::TreeViewColumn {
        self.builder.get_object_expect("treeViewColumnItem")
    }

    pub fn get_tree_view_column_repeat(&self) -> gtk::TreeViewColumn {
        self.builder.get_object_expect("treeViewColumnRepeat")
    }

    pub fn get_cell_renderer_text_item(&self) -> gtk::CellRendererText {
        self.builder.get_object_expect("cellRendererTextItem")
    }

    pub fn get_cell_renderer_text_repeat(&self) -> gtk::CellRendererText {
        self.builder.get_object_expect("cellRendererTextRepeat")
    }
}
