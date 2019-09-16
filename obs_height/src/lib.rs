#![allow(dead_code)]
#![allow(unused_imports)]
// these just make debugging easier and get rid of unneccesary functions


mod obj;
mod yours;

use self::obj::*;
use self::yours::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
