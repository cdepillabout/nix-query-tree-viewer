use std::sync::{Arc, RwLock, RwLockReadGuard};

use super::super::nix_query_tree::exec_nix_store::{
    ExecNixStoreRes, NixStoreRes,
};
use super::builder;
use super::prelude::*;

/// Sort order for the tree of nix store paths.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SortOrder {
    NixStoreOrigOutput = 0,
    AlphabeticalHash,
    AlphabeticalDrvName,
}

impl Default for SortOrder {
    fn default() -> Self { SortOrder::NixStoreOrigOutput }
}

impl TryFrom<u32> for SortOrder {
    type Error = u32;

    fn try_from(value: u32) -> Result<SortOrder, u32> {
        match value {
            0 => Ok(SortOrder::NixStoreOrigOutput),
            1 => Ok(SortOrder::AlphabeticalHash),
            2 => Ok(SortOrder::AlphabeticalDrvName),
            n => Err(n),
        }
    }
}

/// View style for an individual nix store path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ViewStyle {
    FullPath = 0,
    HashAndDrvName,
    OnlyDrvName,
}

impl Default for ViewStyle {
    fn default() -> Self { ViewStyle::FullPath }
}

impl TryFrom<u32> for ViewStyle {
    type Error = u32;

    fn try_from(value: u32) -> Result<ViewStyle, u32> {
        match value {
            0 => Ok(ViewStyle::FullPath),
            1 => Ok(ViewStyle::HashAndDrvName),
            2 => Ok(ViewStyle::OnlyDrvName),
            n => Err(n),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Display(ExecNixStoreRes),
}

#[derive(Clone, Debug)]
pub struct State {
    pub app: gtk::Application,
    pub builder: gtk::Builder,
    pub sender: glib::Sender<Message>,
    pub nix_store_res: Arc<RwLock<Option<NixStoreRes>>>,
    pub sort_order: Arc<RwLock<SortOrder>>,
    pub view_style: Arc<RwLock<ViewStyle>>,
}

impl State {
    pub fn new(app: gtk::Application, sender: glib::Sender<Message>) -> Self {
        State {
            app,
            builder: builder::create(),
            sender,
            nix_store_res: Arc::new(RwLock::new(None)),
            sort_order: Default::default(),
            view_style: Default::default(),
        }
    }

    pub fn read_nix_store_res(&self) -> RwLockReadGuard<Option<NixStoreRes>> {
        self.nix_store_res.read().unwrap()
    }

    pub fn read_sort_order(&self) -> RwLockReadGuard<SortOrder> {
        self.sort_order.read().unwrap()
    }

    pub fn read_view_style(&self) -> RwLockReadGuard<ViewStyle> {
        self.view_style.read().unwrap()
    }

    pub fn write_nix_store_res(&self, new_nix_store_res: NixStoreRes) {
        let state_option_nix_store_res: &mut Option<NixStoreRes> = &mut *self.nix_store_res.write().unwrap();
        *state_option_nix_store_res = Some(new_nix_store_res);
    }

    pub fn write_sort_order(&self, new_sort_order: SortOrder) {
        let state_sort_order: &mut SortOrder = &mut *self.sort_order.write().unwrap();
        *state_sort_order = new_sort_order;
    }

    pub fn write_view_style(&self, new_view_style: ViewStyle) {
        let state_view_style: &mut ViewStyle = &mut *self.view_style.write().unwrap();
        *state_view_style = new_view_style;
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

    pub fn get_search_entry(&self) -> gtk::SearchEntry {
        self.builder.get_object_expect("searchEntry")
    }

    pub fn get_search_button(&self) -> gtk::Button {
        self.builder.get_object_expect("searchButton")
    }

    pub fn get_tree_store(&self) -> gtk::TreeStore {
        self.builder.get_object_expect("treeStore")
    }

    pub fn get_tree_model_sort(&self) -> gtk::TreeModelSort {
        self.builder.get_object_expect("treeModelSort")
    }

    pub fn get_sort_combo_box(&self) -> gtk::ComboBoxText {
        self.builder.get_object_expect("sortComboBox")
    }

    pub fn get_view_combo_box(&self) -> gtk::ComboBoxText {
        self.builder.get_object_expect("viewComboBox")
    }
}
