#![allow(dead_code)]
#![allow(unused_imports)]
// these just make debugging easier and get rid of unneccesary functions

use std::vec::Vec;

mod obj;
mod yours;

use obj::location::*;
use obj::waypoint::*;
use obj::plane::*;
use obj::obstacle::*;
use yours::point::*;

const PI: f32 = 3.141592;

struct Algorithm {
    wp: Waypoint<i8>,
    plane: Plane<i8>,
    obs: Obstacle,
    origin: Location,
}

impl Algorithm {
    fn init(wp: Waypoint<i8>, plane:Plane<i8>, obs: Obstacle, origin: Location) -> Algorithm {
        Algorithm {
            wp,
            plane,
            obs,
            origin,
        }
    }

    // outputs distance of shortest path over obstacle top
    fn find_path(&self, resolution: u8) -> f32 {
        let plane_point = Point::from_location(&self.plane.loc, &self.origin);
        let waypoint = Point::from_location(&self.wp.loc, &self.origin);

        let mut candidates = Vec::new(); //list of path distances

        for i in 0..resolution {
            let phi = 2.0*PI*(i as f32 / resolution as f32);


            let edge = Point::from_obstacle(&self.obs, &self.origin, phi);

            candidates.push(edge.distance(&plane_point) + edge.distance(&waypoint));
        }

        for j in 0..candidates.len() {
            let next = (j + 1) % candidates.len();

            if candidates[j].min(candidates[next]) == candidates[j] {
                candidates.remove(next);
            } else if candidates[j].min(candidates[next]) == candidates[next] {
                candidates.remove(j);
            }
        }
        candidates[0]

        //let out = candidates.into_iter().min();
        //out.expect("no paths to calculate")

        //let mut dist;
        //let out: f32 = for j in 00..candidates.len() {
            //dist = candidates[0];
        //    let next = (j + 1) % candidates.len();

        //    let min: bool = candidates[j] < candidates[next];

        //    let x = if min {
        //        candidates[j]
        //    } else {
        //        candidates[next]
        //    };
            //if x < dist {
            //    dist = x;
            //}
        //};
        //out
        //let out = dist;

        // NEED:
        // obstacle edges as points DONE
        // iteratable distance function DONE
        // shortest selection control DONE

        //8f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn init_test() {
        let origin = Location::new(40.0, 40.0, 0.0);
        let wp_l = Location::new(40.001, 40.001, 5.0);
        let plane_l = Location::new(40.0001, 40.0001, 15.0);
        let obs_l = Location::new(40.0005, 40.0005, 10.0);

        let wp: Waypoint<i8> = Waypoint::new(wp_l);
        let plane: Plane<i8> = Plane::new(plane_l);
        let obs = Obstacle::new(obs_l, 20.0);

        let algo = Algorithm::init(wp, plane, obs, origin);
        assert!((algo.obs.loc.lat - 40.0005) < 0.001);
    }

    #[test]
    fn find_path_test() {
        let origin = Location::new(40.0, 40.0, 0.0);
        let wp_l = Location::new(40.001, 40.001, 5.0);
        let plane_l = Location::new(40.0001, 40.0001, 15.0);
        let obs_l = Location::new(40.0005, 40.0005, 10.0);

        let wp: Waypoint<i8> = Waypoint::new(wp_l);
        let plane: Plane<i8> = Plane::new(plane_l);
        let obs = Obstacle::new(obs_l, 20.0);

        let algo = Algorithm::init(wp, plane, obs, origin);
        assert!((algo.obs.loc.lat - 40.0005) < 0.001);
    }
}
