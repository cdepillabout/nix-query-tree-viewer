mod raw;
mod tree;

use std::rc::Rc;

use crate::nix_query_tree::exec_nix_store::{ExecNixStoreRes};

pub fn setup(builder: gtk::Builder, exec_nix_store_res_rc: Rc<ExecNixStoreRes>) {
    tree::setup(builder.clone(), Rc::clone(&exec_nix_store_res_rc));
    raw::setup(builder.clone(), exec_nix_store_res_rc);
}
