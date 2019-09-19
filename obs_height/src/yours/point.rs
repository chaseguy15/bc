use Location;
use Obstacle;

const RADIUS: f32 = 6_378_000.0;

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            x,
            y,
            z,
        }
    }

    pub fn from_location(loc: &Location, origin: &Location) -> Point {
        Point {
            x: 2.0*RADIUS*((loc.lat).cos()*(loc.lon-origin.lon).sin()).asin(),
            y: RADIUS*(loc.lat-origin.lat),
            z: loc.alt,
        }
    }

    pub fn distance(&self, other: &Point) -> f32 {
        ((&self.x - other.x).powi(2) + (&self.y - other.y).powi(2) + (&self.z - other.z).powi(2)).sqrt()
    }

    pub fn from_obstacle(obs: &Obstacle ,origin: &Location, theta: f32) -> Point {
        // theta in radians describes counter-clockwise angle above constant center.y line
        let center = Point::from_location(&obs.loc, origin);
        let r = obs.radius;

        Point {
            x: center.x + r*theta.cos(),
            y: center.y + r*theta.sin(),
            z: center.z,
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_point() {
        let (x, y, z) = (13f32, 45f32, 10.7f32);
        let p = Point::new(x, y, z);

        assert_eq!((13f32, 45f32, 10.7f32),(p.x, p.y, p.z));
    }

    #[test]
    fn test_x_from_location() {
        let loc = Location::new(1f32, 1f32, 1f32);
        let origin = Location::new(0f32, 0f32, 0f32);

        let p = Point::from_location(&loc, &origin);

        assert!((63.78f32 - p.x) < 1f32);
    }

    #[test]
    fn test_y_from_location() {
        let loc = Location::new(0.00001f32, 0.00001f32, 1f32);
        let origin = Location::new(0f32, 0f32, 0f32);

        let p = Point::from_location(&loc, &origin);

        assert!((63.78f32 - p.y) < 1f32);
    }

    #[test]
    fn test_z_from_location() {
        let loc = Location::new(1f32, 1f32, 1f32);
        let origin = Location::new(0f32, 0f32, 0f32);

        let p = Point::from_location(&loc, &origin);

        assert!((1f32 - p.z) < 0.00001f32);
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(1f32, 2.3f32, 7f32);
        let p2 = Point::new(-2.6f32, 5f32, 1.1f32);

        let d = p1.distance(&p2);

        assert!((6.68281f32 - d) < 0.001f32);
    }
}
