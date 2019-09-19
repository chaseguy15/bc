use super::Location;

pub struct Waypoint<T> { // NOTE: when you call a Waypoint you have to enter a dummy T of any type
    loc: Location,
    data: Option<T>,
}

impl<T> Waypoint<T> {
    fn new(loc: Location) -> Self {
        Self {
            loc,
            data: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_wp() {
        let (lat, lon, alt) = (0f32, 2f32, 5.3f32);
        let loc = Location::new(lat, lon, alt);

        let wp: Waypoint<u8> = Waypoint::new(loc);

        assert_eq!((0f32, 2f32, 5.3f32),(wp.loc.lat, wp.loc.lon, wp.loc.alt));
        assert_eq!(None, wp.data);
    }
}
