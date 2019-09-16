// OWNERSHIP
// ownership in Rust is a way to prevent memory problems

// all variables in Rust have a scope that is kept track of
// in some languages, these are hard to keep track of and cause errors that are not caught by the compiler


// as we talked about before, when a variable is called in a function, it is destroyed once it goes
// out of scope
fn fun() {
    let x = 5f32;
}

// only one thing in rust can EVER own a resource at a time
let a = [1f32,2f32];

let b = a;
// at this point we can not use a again because b has taken ownership of its resources

// similarly, a function takes ownership of its parameters
fn on_a(in: [f32;2]) {
    ...
}

let a = [1f32,2f32];
on_a(a);
// again, a no longer has ownership of its resources at this point
// note, primitive types can be "copied" because of the "Copy" trait they have

// to get around this, we would have to give ownership back every time we use it which would suck
// to allow something to be operated on and still useful, we usually operate on a reference


// REFERENCES
// references are used as short lived copies of the resources of a type


&x // this is a shared reference
&mut x // this is a mutable reference

// 1. a reference cannot "outlive" the thing it is referencing
// 2. if you have a mutable reference, it is the only reference you can have
// these prevent "data racing" and "use after free"


// LIFETIMES
// lifetimes can be used to prevent dangling pointers or "use after free" cases

fn skip_prefix(line: &str, prefix: &str) -> &str {
    // ...
}

let line = "lang:en=Hello World!";
let lang = "en";

let v;
{
    let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
    v = skip_prefix(line, p.as_str());  //  |
}                                       // -+ `p` goes out of scope.
println!("{}", v);
// here v uses both line and p which have 2 different lifetimes. line is still alive but p has
// already gone out of scope. the println! now uses v which has 2 lifetimes which is not allowed
// to fix these problems, we can explicitly show the lifetime of a reference using an apostrophe
fn skip_prefix<'a,'b>(line: &'a str, prefix: &'b str) -> &'a str {
// this clearly shows that the output, v in this case, has the same lifetime as line, 'a
// and a different lifetime than p, 'b

// NOTE: the letter used in a lifetime is ambiguous, they are just for comparison

// lifetimes are also needed in structs that uses references
struct Student<'a> {
    name: String,
    age: &'a u8,
}

// a lifetime is needed in every call that uses a type with a lifetime on it
impl<'a> Student<'a> {
    fn age(&self) -> &'a u8 {self.age}
}
