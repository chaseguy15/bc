// github.com/chaseguy15/bc.git
// git clone url
// doc.rust-lang.org/book OR rust-by-example OR stable

// VECTORS

let v = vec![1f32,3f32,5f32];
// like an array but data is stored in heap with point in stack
// the vector has a pointer, length, and cap in the stack and when len=cap it is reallocated larger

v.push(3); // adds to the end of the vec
v.pop(); // removes last value
v.len(); // gives current length of vector

// MATCH

let x = 5f32;
match x {
    1 => ,
    2 => ,
    3 => ,
    4 => ,
    5 => ,
    _ => ,
};


// LIBRARY

// cargo new <name> --lib

// does not contain a main.rs, lib.rs is entry point
// based mostly on testing

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_test() {
        assereq!();
        asserteq!();
    }
}
// use cargo test to run test functions all over repo
