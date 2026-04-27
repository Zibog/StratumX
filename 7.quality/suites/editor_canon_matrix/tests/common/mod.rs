#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::manual_is_multiple_of
)]
use stratumx_test_support::{EditorPanel, EditorProduct, ToolMode};

pub fn editor() -> EditorProduct {
    EditorProduct::new_demo("Editor Test").unwrap()
}
