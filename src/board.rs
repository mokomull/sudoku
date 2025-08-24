#[cfg(test)]
mod test;

use std::ops::{Index, IndexMut};

use thiserror::Error;
use wasm_bindgen::prelude::*;

use Coordinate::*;

use crate::rules::{Hint, RULES};

#[derive(Debug)]
pub enum Coordinate {
    Row(u8),
    Column(u8),
    Block(u8),
}

impl Coordinate {
    pub fn locations(&self) -> impl Iterator<Item = Location> {
        (0..9).map(move |i| match *self {
            Row(x) => Location { x, y: i },
            Column(y) => Location { x: i, y },
            Block(b) => {
                let dx = i / 3;
                let dy = i % 3;

                // this is the inverse of the function in Location::block()
                let top_left = match b {
                    0 => (0, 0),
                    1 => (0, 3),
                    2 => (0, 6),
                    3 => (3, 0),
                    4 => (3, 3),
                    5 => (3, 6),
                    6 => (6, 0),
                    7 => (6, 3),
                    8 => (6, 6),
                    _ => panic!("we somehow got a block number that isn't real: {self:?}"),
                };

                Location {
                    x: top_left.0 + dx,
                    y: top_left.1 + dy,
                }
            }
        })
    }

    fn others_except(&self, location: Location) -> impl Iterator<Item = Location> {
        self.locations().filter(move |l| l != &location)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub struct Location {
    pub x: u8,
    pub y: u8,
}

impl Location {
    pub fn try_new(x: u8, y: u8) -> Result<Self, LocationError> {
        if x > 8 || y > 8 {
            return Err(LocationError::BadIndex);
        }

        Ok(Self { x, y })
    }

    fn block(&self) -> u8 {
        // this is the inverse of Coordinate::locations().
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

#[derive(serde::Serialize, Debug, Error)]
#[wasm_bindgen]
pub enum LocationError {
    #[error("indexes must be between 0 and 8, inclusive")]
    BadIndex,
}

#[derive(serde::Serialize, Debug, Error)]
#[wasm_bindgen]
pub enum ValueError {
    #[error("the value of a cell must be between 1 and 9, inclusive")]
    BadValue,
}

// TODO: this really doesn't need to be an enum of enums, unless I *actually* use LocationError or
// ValueError somewhere else.  If not, then just collapse this and we can get rid of tsify.
#[derive(serde::Serialize, tsify::Tsify, Debug, Error)]
#[tsify(into_wasm_abi)]
pub enum SolveError {
    #[error(transparent)]
    Location(#[from] LocationError),
    #[error(transparent)]
    Value(#[from] ValueError),
    #[error("the cell does not allow this value")]
    WrongValueForCell,
}

#[derive(Clone, Copy)]
pub struct Cell {
    // a bitmask representing which values this cell may have.  bit 1 (i.e. the value two)
    // represents the digit '1', bit 2 (i.e. the value four) represents the digit '2', ..., bit 9
    // (value 512) represents the digit '9'.  Bit 0 (i.e. value one) is set if this cell is "solved"
    // -- only one other bit should be set.
    allowed: u16,
}

impl Cell {
    /// Returns Some(bitmask) if the cell is unsolved, or None if the cell is already solved.  This
    /// somewhat leaks the abstraction, but bit 1 (i.e. the value two) represents the digit '1', bit
    /// 2 (i.e. the value four) represents the digit '2', ..., bit 9 (value 512) represents the
    /// digit '9'.
    pub fn unsolved_allowed_values(&self) -> Option<u16> {
        if self.allowed & 0x1 != 0 {
            None
        } else {
            Some(self.allowed)
        }
    }

    fn to_js(self) -> crate::Cell {
        if self.allowed & 1 != 0 {
            let masked = self.allowed & !1;
            if masked.count_ones() != 1 {
                panic!(
                    "Cell is \"solved\" but it has multiple bits set: 0x{:x}",
                    self.allowed
                );
            }
            crate::Cell::Solved(masked.trailing_zeros().to_string())
        } else {
            crate::Cell::Choices(
                (1..=9)
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

    fn allows(&self, value: u8) -> bool {
        assert!(
            (1..=9).contains(&value),
            "{}, got {value}",
            ValueError::BadValue
        );

        self.allowed & (1 << value) > 0
    }

    fn mark_solved(&mut self, value: u8) {
        assert!(
            (1..=9).contains(&value),
            "{}, got {value}",
            ValueError::BadValue
        );

        self.allowed = 1 | (1 << value);
    }

    fn remove(&mut self, value: u8) -> bool {
        assert!(
            (1..=9).contains(&value),
            "{}, got {value}",
            ValueError::BadValue
        );

        let had_it = (self.allowed & (1 << value)) > 0;
        self.allowed &= !(1 << value);

        had_it
    }
}

struct UndoNode {
    location: Location,
    value: u8,
    previous_allowed: u16,
    affected_neighbors: Vec<Location>,
}

#[wasm_bindgen]
pub struct Board {
    cells: [[Cell; 9]; 9],
    undo_stack: Vec<UndoNode>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Cell {
                // all nine values allowed
                allowed: 0b11_1111_1110,
            }; _]; _],
            undo_stack: vec![],
        }
    }
}

impl Index<Location> for Board {
    type Output = Cell;

    fn index(&self, location: Location) -> &Self::Output {
        &self.cells[usize::from(location.x)][usize::from(location.y)]
    }
}

impl IndexMut<Location> for Board {
    fn index_mut(&mut self, location: Location) -> &mut Self::Output {
        &mut self.cells[usize::from(location.x)][usize::from(location.y)]
    }
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_js(&self) -> Vec<crate::Cell> {
        self.cells
            .iter()
            .flat_map(|row| row.iter().map(|cell| cell.to_js()))
            .collect()
    }

    pub fn mark_cell_solved(&mut self, row: u8, column: u8, value: u8) -> Result<(), SolveError> {
        let location = Location::try_new(row, column)?;

        if !(1..=9).contains(&value) {
            return Err(ValueError::BadValue.into());
        }

        if !self[location].allows(value) {
            return Err(SolveError::WrongValueForCell);
        }

        let mut affected_neighbors = vec![];
        for coordinate in location.coordinates() {
            for other in coordinate.others_except(location) {
                if self[other].remove(value) {
                    affected_neighbors.push(other);
                }
            }
        }

        let cell = &mut self[location];
        let previous_allowed = cell.allowed;
        cell.mark_solved(value);

        self.undo_stack.push(UndoNode {
            location,
            value,
            previous_allowed,
            affected_neighbors,
        });

        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), JsError> {
        let Some(undo_node) = self.undo_stack.pop() else {
            return Err(JsError::new("undo stack is empty"));
        };

        for other in undo_node.affected_neighbors {
            self[other].allowed |= 1 << undo_node.value;
        }

        self[undo_node.location].allowed = undo_node.previous_allowed;

        Ok(())
    }

    pub fn hints(&self) -> Vec<Hint> {
        RULES.iter().flat_map(|&rule| rule.check(self)).collect()
    }
}
