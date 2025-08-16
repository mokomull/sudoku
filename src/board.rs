use thiserror::Error;
use wasm_bindgen::prelude::*;

use Coordinate::*;

pub enum Coordinate {
    Row(u8),
    Column(u8),
    Block(u8),
}

pub struct Location {
    x: u8,
    y: u8,
}

impl Location {
    pub fn try_new(x: u8, y: u8) -> Result<Self, LocationError> {
        if x > 8 || y > 8 {
            return Err(LocationError::BadIndex);
        }

        Ok(Self { x, y })
    }

    fn block(&self) -> u8 {
        match (self.x / 3, self.y / 3) {
            (0, 0) => 0,
            (0, 1) => 1,
            (0, 2) => 2,
            (1, 0) => 3,
            (1, 1) => 4,
            (1, 2) => 5,
            (2, 0) => 6,
            (2, 1) => 7,
            (2, 2) => 8,
            _ => panic!("this Location has corrupt data: {:?}", (self.x, self.y)),
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Coordinate> {
        [Row(self.x), Column(self.y), Block(self.block())].into_iter()
    }
}

#[derive(Debug, Error)]
pub enum LocationError {
    #[error("indexes must be between 0 and 8, inclusive")]
    BadIndex,
}

#[derive(Clone, Copy)]
struct Cell {
    // a bitmask representing which values this cell may have.  bit 1 (i.e. the value two)
    // represents the digit '1', bit 2 (i.e. the value four) represents the digit '2', ..., bit 9
    // (value 512) represents the digit '9'.  Bit 0 (i.e. value one) is set if this cell is "solved"
    // -- only one other bit should be set.
    allowed: u16,
}

impl Cell {
    fn to_js(&self) -> crate::Cell {
        if self.allowed & 1 != 0 {
            let masked = self.allowed & !1;
            if !masked.count_ones() != 1 {
                panic!(
                    "Cell is \"solved\" but it has multiple bits set: 0x{:x}",
                    self.allowed
                );
            }
            crate::Cell::Solved(masked.trailing_zeros().to_string())
        } else {
            crate::Cell::Choices(
                (1..=9)
                    .into_iter()
                    .filter_map(|bit| {
                        if self.allowed & (1 << bit) != 0 {
                            Some(bit.to_string())
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        }
    }
}

#[wasm_bindgen]
pub struct Board {
    cells: [[Cell; 9]; 9],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Cell {
                // all nine values allowed
                allowed: 0b11_1111_1110,
            }; _]; _],
        }
    }
}

#[wasm_bindgen]
impl Board {
    pub fn to_js(&self) -> Vec<crate::Cell> {
        self.cells
            .iter()
            .flat_map(|row| row.iter().map(|cell| cell.to_js()))
            .collect()
    }
}
