pub use std::convert::TryFrom;

use super::super::super::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Column {
    Item = 0,
    Recurse,
}

// Is there some way to derive these types of things?
const LIST: [Column; 2] = [Column::Item, Column::Recurse];
pub const INDICIES: [usize; 2] = [Column::Item as usize, Column::Recurse as usize];

impl Column {
    pub fn from_gtk(
        tree_view: gtk::TreeView,
        tree_view_column: gtk::TreeViewColumn,
    ) -> Option<Column> {
        let column_pos: usize = get_tree_view_column_pos(tree_view.clone(), tree_view_column.clone());
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

