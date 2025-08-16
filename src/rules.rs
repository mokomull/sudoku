use crate::board::Board;
use crate::board::Location;

pub trait Rule {
    fn check(&self, board: &Board) -> Vec<Hint>;
}

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
