use lang_server::MumpsLsp;
use monaco_tower_lsp_bridge::{PublishMessageCallback, TowerLspWasmBridge};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_mumps_lsp(publish: PublishMessageCallback) -> TowerLspWasmBridge {
    console_error_panic_hook::set_once();
    TowerLspWasmBridge::new(publish, MumpsLsp::new)
}
