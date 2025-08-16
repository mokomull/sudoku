use super::*;

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
