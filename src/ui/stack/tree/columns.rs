pub use std::convert::TryFrom;

use super::super::super::super::ui;
use super::super::super::prelude::*;

/// These correspond to actual columns in our data model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Column {
    FullPath = 0,
    Recurse,
    HashAndDrvName,
    OnlyDrvName,
}

impl TryFrom<usize> for Column {
    type Error = usize;

    fn try_from(value: usize) -> Result<Column, usize> {
        if value < Self::INDICIES.len() {
            Ok(Self::LIST[value])
        } else {
            Err(value)
        }
    }
}

impl Column {
    // Is there some way to derive these types of things?
    const LIST: [Column; 4] = [
        Column::FullPath,
        Column::Recurse,
        Column::HashAndDrvName,
        Column::OnlyDrvName,
    ];
    pub const INDICIES: [usize; 4] = [
        Column::FullPath as usize,
        Column::Recurse as usize,
        Column::HashAndDrvName as usize,
        Column::OnlyDrvName as usize,
    ];
}

pub fn change_view_style(state: &ui::State) {
    let item_renderer = state.get_cell_renderer_text_item();
    let column = state.get_tree_view_column_item();

    column.clear_attributes(&item_renderer);

    match *state.read_view_style() {
        ui::ViewStyle::FullPath => {
            column.add_attribute(
                &item_renderer,
                "text",
                Column::FullPath as i32,
            );
        }
        ui::ViewStyle::HashAndDrvName => {
            column.add_attribute(
                &item_renderer,
                "text",
                Column::HashAndDrvName as i32,
            );
        }
        ui::ViewStyle::OnlyDrvName => {
            column.add_attribute(
                &item_renderer,
                "text",
                Column::OnlyDrvName as i32,
            );
        }
    }

    // Tree needs to be redrawn because changing the renderer on a column don't seem to cause a
    // redraw.
    state.get_tree_view().queue_draw();
}
