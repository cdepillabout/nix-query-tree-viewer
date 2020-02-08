pub use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Column {
    Item = 0,
    Recurse,
}

// Is there some way to derive these types of things?
const LIST: [Column; 2] = [Column::Item, Column::Recurse];
pub const INDICIES: [usize; 2] = [Column::Item as usize, Column::Recurse as usize];

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

