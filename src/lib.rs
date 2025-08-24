mod board;
mod rules;

#[derive(serde::Serialize, tsify::Tsify)]
#[tsify(into_wasm_abi)]
pub enum Cell {
    Solved(String),
    Choices(String),
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn install_panic_hook() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
}
