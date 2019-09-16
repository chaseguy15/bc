use super::Location;

struct Plane<T> { // NOTE: when you call a Plane you have to enter a dummy T of any type
    loc: Location,
    data: Option<T>,
}

impl<T> Plane<T> {
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
    fn test_plane() {
        let (lat, lon, alt) = (0f32, 2f32, 5.3f32);
        let loc = Location::new(lat, lon, alt);

        let p3: Plane<u8> = Plane::new(loc);

        assert_eq!((0f32, 2f32, 5.3f32),(p3.loc.lat, p3.loc.lon, p3.loc.alt));
        assert_eq!(None, p3.data);
    }
}
