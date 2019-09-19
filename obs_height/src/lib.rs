#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
// these just make debugging easier and get rid of unneccesary functions

mod obj;
mod yours;

use obj::waypoint::Waypoint;
use obj::plane::Plane;
use obj::obstacle::Obstacle;
use obj::point::Point;

struct Algorithm {
    wp: Waypoint<Point>,
    plane: Plane<Obstacle>,
    obs: Obstacle,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
