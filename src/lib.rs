mod board;
mod rules;

#[derive(serde::Serialize, tsify::Tsify)]
#[tsify(into_wasm_abi)]
pub enum Cell {
    Solved(String),
    Choices(String),
}
