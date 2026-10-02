use lang_server::MumpsLsp;
use monaco_tower_lsp_bridge::initialize_web_worker_lsp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_mumps_lsp() {
    console_error_panic_hook::set_once();
    initialize_web_worker_lsp(MumpsLsp::new);
}
