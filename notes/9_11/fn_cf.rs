// note: this will not compile because of example_function being empty


fn main() { // main is entry point of Rust, only things called in main are run

    // functions are declared by fn
    fn example_function(param:i32) -> (bool) { // snake case: should be lowercase and '_' separated
        // functions must have type declared parameters
        // some functions have an output, only the type is declared
    }

    function_one(); // see bottom with function_one and function_two
    // this will print:
    // function 2
    // function 1
    // function 2


    // CONTROL FLOW
    // control is based around if-else statements

    let x = 5f32;
    if  x > 3f32 { // if statements use boolean to evaluate
        let dif = x - 3f32;
        println!("x is {} greater than 3",dif);
    } else if x > 0f32 { // else if allows for multiple conditions
        println!("x is less than 3 but greater than positive.");
    } else { // else gives the final condition
        println!("x is negative.");
    }

    // since control flow is made of block expressions, we can use it in let statements
    let homework:bool = true;
    let grade = if homework {
        100
    } else (
        70
    );

    // loops are used to run a block repeatedly until manually canceled
    loop {
        println!("over and");
    }

    // loops can be stopped with a break
    let mut i = 1u8;
    loop {
        i = i + 1;
        if i == 15 { // == is used to check sameness
            break
        }
    };

    // while is used to run iterations of a possibly unknown range of inputs
    // you can think of this as when i is dependent on the output of the loop
    i = 1u8;
    while i < 15 {
        i = i + 1;
    }

    // for is used to run iterations on a known range of inputs
    // you can think of this as when i is an independent variable
    for i = (0..15) {
    }
}




fn function_one() { // function_one is called in main
    function_two();
    println!("function 1");
    function_two();
}

fn function_two() { // function_two is called in function_one which is called in main
    println!("function 2");
}
