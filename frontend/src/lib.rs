use leptos::mount_to_body;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    mount_to_body(app::App)
}
