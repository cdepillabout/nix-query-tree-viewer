
use std::sync::Arc;

use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes};

use super::super::prelude::*;
use super::super::super::ui;

pub fn setup(state: &ui::State, exec_nix_store_res: Arc<ExecNixStoreRes>) {
    let text_buffer: gtk::TextBuffer = builder.get_object_expect("rawTextBuffer");

    // TODO: This is super ugly.
    let text: String = match &exec_nix_store_res.res {
        Err(nix_store_err) => nix_store_err.to_string(),
        Ok(nix_store_res_rc) => String::clone(&nix_store_res_rc.raw),
    };

    text_buffer.set_text(&text);
}

