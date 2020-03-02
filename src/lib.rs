#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

pub mod nix_query_tree;
pub mod tree;

mod opts;
mod ui;

pub fn default_main() {
    ui::run();
}
