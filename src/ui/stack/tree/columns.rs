pub use std::convert::TryFrom;

use super::super::super::super::ui;
use super::super::super::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Column {
    Item = 0,
    Recurse,
}

// Is there some way to derive these types of things?
const LIST: [Column; 2] = [Column::Item, Column::Recurse];
pub const INDICIES: [usize; 2] =
    [Column::Item as usize, Column::Recurse as usize];

impl Column {
    pub fn from_gtk(
        tree_view: gtk::TreeView,
        tree_view_column: gtk::TreeViewColumn,
    ) -> Option<Column> {
        let column_pos: usize = get_tree_view_column_pos(
            tree_view.clone(),
            tree_view_column.clone(),
        );
        Column::try_from(column_pos).ok()
    }
}

impl TryFrom<usize> for Column {
    type Error = usize;

    fn try_from(value: usize) -> Result<Column, usize> {
        if value < INDICIES.len() {
            Ok(LIST[value])
        } else {
            Err(value)
        }
    }
}

fn get_tree_view_column_pos(
    tree_view: gtk::TreeView,
    tree_view_column: gtk::TreeViewColumn,
) -> usize {
    let all_tree_view_columns = tree_view.get_columns();
    let option_pos = all_tree_view_columns
        .iter()
        .position(|col| *col == tree_view_column);
    match option_pos {
        None => panic!("No column matching id.  This should never happen."),
        Some(pos) => pos,
    }
}

fn setup_item_column(state: &ui::State) {
    // let renderer = state.get_cell_renderer_text_item();
    // let column = state.get_tree_view_column_item();
    // column.add_attribute(&renderer, "text", Column::Item as i32);
}

fn setup_link_column(state: &ui::State) {
    // let renderer = state.get_cell_renderer_text_repeat();
    // let column = state.get_tree_view_column_repeat();
    // column.add_attribute(&renderer, "text", Column::Recurse as i32);
}

pub fn change_view_style(state: &ui::State) {
    println!("In change_view_style, starting...");

    let item_renderer = state.get_cell_renderer_text_item();
    let example_renderer = state.get_cell_renderer_text_example();
    let column = state.get_tree_view_column_item();

    match *state.read_view_style() {
        ui::ViewStyle::FullPath => {
            example_renderer.set_visible(false);
            item_renderer.set_visible(true);
        }
        _ => {
            item_renderer.set_visible(false);
            example_renderer.set_visible(true);
        }
    }

    // column.add_attribute(&renderer, "text", 2);

    // println!("In change_view_style, item_renderer is visible: {:?}", item_renderer.get_visible());
    // println!("In change_view_style, example_renderer is visible: {:?}", example_renderer.get_visible());

    // item_renderer.set_visible(false);
    // example_renderer.set_visible(true);

    // println!("In change_view_style, after changing visibility, item_renderer is visible: {:?}", item_renderer.get_visible());
    // println!("In change_view_style, after changing visibility, example_renderer is visible: {:?}", example_renderer.get_visible());
}

pub fn setup(state: &ui::State) {
    setup_item_column(state);
    setup_link_column(state);
}
