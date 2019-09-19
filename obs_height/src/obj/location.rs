use super::point::*;

const R_EARTH: f32 = 6_378_000.0;

pub struct Location {
    pub lat: f32,
    pub lon: f32,
    pub alt: f32,
}

impl Location {
    pub fn new(lat: f32, lon: f32, alt: f32) -> Self {
        Self {
            lat,
            lon,
            alt,
        }
    }

    pub fn location_to_point(self, origin: Location) -> Point {
        Point {
            x: 2.0 * R_EARTH *
            (self.lat.cos() * ((self.lon - origin.lon)/2.0).sin()).asin(),
            y: R_EARTH * (self.lat - origin.lat),
            z: self.alt,
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
        let origin = Location::new(0f32, 0f32, 0f32);

        let point: Point = loc.location_to_point(origin);
        println!("{} is the x-coordinate, {} is the difference", &point.x, &point.x - 12_756_000.0);
        assert!((point.x - 12_756_000.0) < 1;

    }
}
