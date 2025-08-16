use std::collections::BTreeSet;

use super::*;

use itertools::Itertools;

#[test]
/// This is an exhaustive test that, for all 81 valid locations, the Row, Column, and Block that the
/// cell belongs to would iterate through that location.
fn coordinate_locations_must_contain_location() {
    for x in 0..9 {
        for y in 0..9 {
            let location = Location { x, y };

            for coordinate in location.coordinates() {
                assert!(
                    coordinate.locations().any(|l| l == location),
                    "{coordinate:?} does not contain {location:?}"
                );
            }
        }
    }
}

#[test]
/// This is one example Block, which is used to debug coordinate_locations_must_contain_location.
fn locations_in_block_0() {
    // block 0 is the top left block, so should contain x = (0..=2), y = (0..=2)
    let expected_locations = (0..=2)
        .cartesian_product(0..=2)
        .map(|(x, y)| Location { x, y })
        .collect::<BTreeSet<_>>();
    let actual_locations = Block(0).locations().collect::<BTreeSet<_>>();
    assert_eq!(expected_locations, actual_locations);
}
