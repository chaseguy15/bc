fn main() {
    //there are two custom types in Rust: structs and enums


    // STRUCT

    // a struct is declared with a name and fields
    struct Student {
        name: String,
        age: u8,
    }
    // a structs fields can be primitive or custom types, including other structs
    // 3 kinds: tuples structs, C structs, unit structs

    // unit structs are fieldless (useful for generics)
    struct unit;

    // tuples structs are basically named tuples
    struct Pair(i32,i32);

    // C structs are normal structures, defined with fields
    struct Point {
        x: f32,
        y: f32,
    }

    // there are a few ways to instantiate (populate) a new struct:
    let name = "Chase";
    let age = 20;

    let chase = Student {name,age};

    let chase: Student = Student{name,age};

    // to access the fields, use '.'
    println!("I am {} years old.",chase.age);

    // sometimes it is useful to destructure into variables
    let age_of_chase = chase.age;
    println!("age_of_chase: {}",age_of_chase);

    let Student{name,age} = chase;
    println!("destructured tuple: {} {}",name,age);


    // ENUM
    // an enum is a type which can vary
    // any kind of structure can exist within an enum
    // the enum variations are completely independent

    enum Human {
        // c-like
        Student { major: str, school: str, age: u8 },
        Parent { children: u8, married: bool },
        // unit-like
        Dead,
        // tuple-struct
        Retired(retired_years, age),
    }
    // enums essentially represent an a set possible structs

    // in Rust, both structs and enums are considered "Objects"

    // OOP
    // object oriented programming is a model for programming where data
    // manipulation revolves around objects rather than functions and cf

    // objects must have data and methods for manipulating data
    // other ideas pertaining to OOP are visilbility/privacy, inhertiance, and polymorphism

    // in Rust, implemented structs are objects amd enums allow for polymorphism (shape changing)
    // generics allow for high levels of inheritance as well

    // WRITE ABOUT METHODS AND ASSOCIATED FUNCTIONS

    // METHODS
    // methods are functions attached to objects
    // methods are called by implementing the object

    impl Student {
        fn new(name: String, age: u8) -> (Student) {
            Student{name, age}
        }

        fn display(&self) -> (String) {
            format!("{}, age {}", self.name, self.age)
        }
        // here we implemented 2 methods for student, one to create a new student and one to
        // display the information of a student
    }

    // here are demonstrations of how these implementations would work, using "::" and "." notation
    let name: String = from_string("Chase");
    let age: u8 = 20;
    let student_chase = Student::new(name,age);

    let show = student_chase.display();
    println!("{}", show);
}
