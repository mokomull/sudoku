use std::collections::BTreeSet;

use itertools::Either;
use itertools::Itertools;
use wasm_bindgen::prelude::*;

use crate::board::Board;
use crate::board::Coordinate::*;
use crate::board::Location;

pub trait Rule {
    fn check(&self, board: &Board) -> Vec<Hint>;
}

pub static RULES: &[&(dyn Rule + Sync)] = &[&OnlyOneAllowedValue {}, &DigitsCovered {}];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub struct DigitLocation {
    pub location: Location,
    // None if the cause is the whole cell; a non-empty Vec of digits (integers valued 1 through 9
    // inclusive) if it is specific digits within the cell.
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub digit: Option<Vec<u8>>,
}

#[wasm_bindgen]
pub struct Hint {
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub description: String,
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub identity: String,
    #[wasm_bindgen(getter_with_clone)]
    pub cause: Vec<DigitLocation>,
    // effect: the possibilities that have been ruled-out due to this Rule.
    #[wasm_bindgen(getter_with_clone)]
    pub effect: Vec<DigitLocation>,
}

// Detect if digits are already covered in the same row, column, or block... e.g. if three cells
// contain only 1, 2, or 3 as allowed choices, then we know that no other cells in that
// row/column/block can be a 1, 2, or a 3.
struct DigitsCovered {}

impl Rule for DigitsCovered {
    fn check(&self, board: &Board) -> Vec<Hint> {
        let mut hints = vec![];
        let mut reported = BTreeSet::new();

        for i in 0..9 {
            for coordinate in [Row(i), Column(i), Block(i)] {
                let locations = coordinate.locations().collect::<Vec<_>>();

                // evaluate the whole power set of locations.  no need checking 0b000_000_000 😁
                'next_mask: for mask in 1_u16..512 {
                    let mut allowed = 0;

                    let (covering, others): (Vec<_>, Vec<_>) =
                        locations.iter().enumerate().partition_map(|(i, &loc)| {
                            if mask & (1 << i) != 0 {
                                Either::Left(loc)
                            } else {
                                Either::Right(loc)
                            }
                        });

                    for &location in &covering {
                        match board[location].unsolved_allowed_values() {
                            Some(x) => allowed |= x,
                            // there's an already-solved cell among the cells we've chosen for this
                            // round, which means we're definitely not going to find our condition
                            // -- we're guaranteed that a solved cell only has one bit set.  so
                            // let's just skip any of the extra work.
                            None => continue 'next_mask,
                        }

                        if allowed.count_ones() as usize > covering.len() {
                            // we've already got more allowed values than squares to put them in, so
                            // we can't draw any conclusions about this set
                            continue 'next_mask;
                        }
                    }

                    // If we have *fewer* allowed values than cells to put them in, then that's a
                    // logical impossibility.  But let's ignore it for now.  TODO: maybe report to
                    // the UI?
                    if allowed.count_ones() as usize != covering.len() {
                        continue 'next_mask;
                    }

                    let mut affected = vec![];
                    for &other in &others {
                        let Some(other_allowed) = board[other].unsolved_allowed_values() else {
                            // There's nothing to be changed for an already-solved cell, so move
                            // along here.
                            continue;
                        };

                        let overlap = other_allowed & allowed;
                        if overlap != 0 {
                            let digits = bits_to_digits(overlap);
                            affected.push(DigitLocation {
                                location: other,
                                digit: Some(digits),
                            });
                        }
                    }

                    if !affected.is_empty() && reported.insert((covering.clone(), affected.clone()))
                    {
                        hints.push(Hint {
                            description: format!(
                                "{} cells containing values {}",
                                covering.len(),
                                bits_to_digits(allowed).into_iter().join(", ")
                            ),
                            identity: format!("digitscovered: {:?} {:?}", covering, affected),
                            cause: covering
                                .into_iter()
                                .map(|location| DigitLocation {
                                    location,
                                    digit: None,
                                })
                                .collect(),
                            effect: affected,
                        })
                    }
                }
            }
        }

        hints
    }
}

fn bits_to_digits(allowed: u16) -> Vec<u8> {
    (1..=9).filter(|&i| allowed & (1 << i) != 0).collect()
}

// Emits a hint when a cell is not solved, but has only one allowed digit.
struct OnlyOneAllowedValue {}

impl Rule for OnlyOneAllowedValue {
    fn check(&self, board: &Board) -> Vec<Hint> {
        let mut hints = vec![];

        for x in 0..9 {
            for y in 0..9 {
                let location = Location { x, y };

                let Some(allowed) = board[location].unsolved_allowed_values() else {
                    continue;
                };

                if allowed.count_ones() == 1 {
                    let value = allowed.trailing_zeros() as u8;
                    hints.push(Hint {
                        description: format!("cell can only contain the value {}", value),
                        identity: format!("onlyoneallowedvalue: {:?} {}", location, value),
                        cause: vec![DigitLocation {
                            location,
                            digit: Some(vec![value]),
                        }],
                        effect: vec![],
                    })
                }
            }
        }

        hints
    }
}
