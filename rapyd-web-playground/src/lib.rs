mod utils;

use rapyd_fine_grained_compilation_output::app;
use wasm_bindgen::prelude::*;
use web_sys::{window, Node, Element};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run(anchor: Element) {
    utils::set_panic_hook();
    app::mount(anchor);
}
