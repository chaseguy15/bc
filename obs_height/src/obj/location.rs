pub struct Location {
    pub lat: f32,
    pub lon: f32,
    pub alt: f32,
}

impl Location {
    pub fn new(lat: f32, lon: f32, alt: f32) -> Location {
        Location {
            lat,
            lon,
            alt,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_loc() {
        let (lat, lon, alt) = (0f32, 2f32, 5.3f32);
        let loc = Location::new(lat, lon, alt);

        assert_eq!((0f32, 2f32, 5.3f32),(loc.lat, loc.lon, loc.alt));
    }
}
