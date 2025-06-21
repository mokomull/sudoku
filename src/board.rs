use thiserror::Error;

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
