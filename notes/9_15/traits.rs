// GENERICS
// generics are used to generalize types for broad cases

// you can define generic type parameters for functions for example to allow all generic types
fn foo<T>(in: T) {
    ...
}
// this is a generic function because <T> is defined before (T) so it will take any T type
// this is uesful for allowing functions to do similar operations on different types

// basically, a generic is a set of types

// this set can be constrained with type traits


// TRAITS
// traits are very useful tool in Rust that allow us to define the behavior of a type

struct Student {
    name: String,
    age: u8
}

// a trait is defined with the trait call and defines the functions the types with this trait have
trait HasName {
    fn get_name(&self) -> String;
}
// for example, any type with the trait "HasName" could have a function that got its name


// a trait is implemented using "impl <trait> for <type>"
impl HasName for Student {
    fn get_name(&self) -> String {
        self.name.into()
    }
}

// here we have assigned HasName to Student and defined exactly how "get_name" works for this type

// we can use similar traits on different types. For example:

trait HasArea {
    fn area(&self) -> f32;
}


struct Square {
    side_length: f32
}

impl HasArea for Square {
    area(&self) -> f32 {
        self.side_length * self.side_length
    }
}

struct Circle {
    radius: f32,
}

impl HasArea for Circle {
    area(&self) -> f32 {
        3.141592f32 * (self.radius * self.radius)
    }
}

// here we can see the trait "HasArea" being assigned to two different types
// these two types can be used in any function with a generic trait call for "HasArea"

fn double_area<T: HasArea> (in: T) -> f32 {
    2f32 * T.area()
}

// double_area is consider generic now but is constrained to types with the HasArea trait
// traits can be applied to any type in Rust, including pre-existing primitives and customs


// MORE GENERICS


// generics can be used on any type in Rust
struct Point<T> {
    x: T,
    y: T,
}
// here Point is generic to T but x and y must both be of T

impl<T> Point<T> {
    fn get_x<T>(self) -> T {
        self.x.into()
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T,E> {
    Ok(T),
    Err(E),
}
