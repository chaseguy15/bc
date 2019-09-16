fn main() {
    // this is how you comment


    // VARIABLES

    const PI :f32 = 3.141592; // const or static can be used to define a constant structure
    static EULERS :f32 = 2.71828;

    let x = 5f32; // let is the standard variable declaration
    // 5f32 is the literal type representation of 5 as an f32. literals are reccomended for types

    // x is immutable which means this will not work:
    //x = x + 1;

    // the only way to change x is called 'shadowing':
    let x = 6f32;
    // shadowing writes the variable again in a new stack block, aka it wastes memory
    // instead, variables which are changing should be made mutable


    let mut y = 4u8;
    // y is mutable which means the value can be changed without a declaration:
    y = y + 1; // this would be allowed by rust and now y = 5u8;

    // variables are constrained to the block they are declared in, ie that is their scope
    fn scope() {
        let scoped_x = 3i32;

        println!("{}", scoped_x); //this will print 3
    }
    //println!("{}", scoped_x); // this will not print 3 because scoped_x has gone out of scoped


    // EXPRESSIONS
    // an expression is a line which evaluates, aka it has an output
    x + 1f32;
    x; // the semicolons suppress output and turn the expression into a statement
    // most lines in Rust are statements

    let x = { // these curly brackets represent a block
        // blocks are also expressions and evaluate to the last expression inside the block
        5f32;
        4u8;
        false
    }; // x will evaluate to false because it is the last expression in the block


    // PRIMITIVE TYPES

    // integers
    let i = 5i8; // signed integer types come in 8-64 bit, denoted "i8,i16,i32,i64"
    let u = 5u8; // unsigned integer types come in 8-64 bit as well, denoted "u8,u16,u32,u64"

    // float
    let f = 5f32; // float point types come in f32 and f64. f32 is single-precision, 64 is double
    // f32 is about the same speed as f64 so the choice comes down to memory and precision

    // bool, char, string
    let tru = true;
    let fal = false; // boolean simply come in true and false and are mostly used for control flow

    let c = 'z'; // char types represent single characters of UTF-8, not just ASCII
    let s = "string"; // strings represent ASCII strings of characters

    // you can also alias types, aka rename them

    type inch = u32; // this gives a new name to u32 that can be useful in defining the variable

    let foot :inch = 12; // this now defines the value of foot more clearly to the reader


    // COMPOUND TYPES

    let tup: (u8,i16,f32,bool,char) = (5,-2,6.1,true,'s'); // tuples are groups of types
    // tuples are very powerful as they are operated and allocated together

    let first = tup.0; // tuples are accessed but '.' and start at an index of 0

    let (x,y,z,a,b) = tup; //setting a tuple equal to a set of variables is called "destructuring"

    // array
    let a: [i32;3] = [5,2,1]; // arrays are all the same type and have a set size
    // all elements of an array must be filled
    // arrays are useful for indexing through a set easily, saved in a STACK
    // another type, similar to arrays that is useful for linked lists is a vector, for later


    // SYSTEM LEVEL MEMORY ALLOCATION
    // most languages you use allocated values in a stack
    // the stack is a fast, effective way to allocate and deallocate and use FILO ordering

    // when a function is called, all of its variables are allocated in order in the stack
    fn stacking() {
        let x = 1f32;
        let y= 2f32; // x and y are put in the stack together in the functions allocated space
    }
    // when this function is over, both x and y are deallocated from the stack

    // the stack can be slowed down significantly because of FILO
    fn first() {
        let x = 1f32; // x is allocated

        second(); // this now takes slightly longer because Rust has to go down the stack to find x
    }

    fn second() {
        let y = 2f32;
        let z = 3f32; // y and z are allocated on top of x
    }
    // there is an alternative to the stack, called the heap which we will talk about later


    // CASTING

    // type casting on primitives in Rust uses the declaration 'as'
    // some other custom types like String have associated conversion functions
    // in general, if you want conversions for custom types, you have to create the function

    let dec = 65.63f32;

    let int = dec as i8; // it is not recommended to go from a more precise to an unprecise type
    // or from a singed to unsigned type because of truncatiion error and overflow
    println!("{}",int);
}
