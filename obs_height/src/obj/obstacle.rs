use super::location::*;

pub struct Obstacle {
    pub loc: Location,
    pub radius: f32,
}

impl Obstacle {
    pub fn new(loc: Location, radius: f32) -> Self {
        Self {
            loc,
            radius,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_obs() {
        let (lat, lon, alt) = (0f32, 2f32, 5.3f32);
        let loc = Location::new(lat, lon, alt);
        let r = 5f32;

        let obs = Obstacle::new(loc, r);

        assert_eq!((0f32, 2f32, 5.3f32),(obs.loc.lat, obs.loc.lon, obs.loc.alt));
        assert_eq!(5f32, obs.radius);
    }
}
