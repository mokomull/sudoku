use itertools::Either;
use itertools::Itertools;

use crate::board::Board;
use crate::board::Coordinate::*;
use crate::board::Location;

pub trait Rule {
    fn check(&self, board: &Board) -> Vec<Hint>;
}

pub static RULES: &[&(dyn Rule + Sync)] = &[&DigitsCovered {}];

pub struct DigitLocation {
    location: Location,
    // None if the cause is the whole cell; a non-empty Vec of digits (integers valued 1 through 9
    // inclusive) if it is specific digits within the cell.
    digit: Option<Vec<u8>>,
}

pub struct Hint {
    cause: Vec<DigitLocation>,
    // effect: the possibilities that have been ruled-out due to this Rule.
    effect: Vec<DigitLocation>,
}

// Detect if digits are already covered in the same row, column, or block... e.g. if three cells
// contain only 1, 2, or 3 as allowed choices, then we know that no other cells in that
// row/column/block can be a 1, 2, or a 3.
struct DigitsCovered {}

impl Rule for DigitsCovered {
    fn check(&self, board: &Board) -> Vec<Hint> {
        let mut hints = vec![];

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
                            let digits = (1..=9).filter(|&i| overlap & (1 << i) != 0).collect();
                            affected.push(DigitLocation {
                                location: other,
                                digit: Some(digits),
                            });
                        }
                    }

                    if !affected.is_empty() {
                        hints.push(Hint {
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
