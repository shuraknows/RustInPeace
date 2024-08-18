fn main() {
    // declared via fn
    // functions should have snake_case names
    another_function();
    parametrized_function(123);
    multiple_params_function("foo", 123);


    // #################################################
    // ########## Statements vs Expressions ############
    // #################################################

    // statements - instructions that perform some action and do not return a value
    // expressions - evaluate to a resulting value

    // Statements
    let _y = 6; // statement
    // function definitions are also statements
    // In Rust unlike other languages assignment does not return a value

    // Expressions
    // Expressions evaluate to a resulting value

    let xyz = {
        let xyz = 3;
        xyz + 1 // this is an expression  WITHOUT SEMICOLON!!
       // xyz // even this is an expression
    };

    println!("The value of y is: {xyz}");

    // !!!!!!!!!!!!
    // Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement,
    // and it will then not return a value.


    // #################################################
    // ########## Functions with return values #########
    // #################################################
    let xxx = five();
    println!("The value of xxx is: {}", xxx);
    println!("The value of xxx is: {}", plus_five(3));
}

fn plus_five(value: i32) -> i32 {
    value + 5
}

fn five() -> i32 {
    5
}

fn multiple_params_function(key: &str, value: i32) {
    println!("[{key}] => {value}");
}

fn parametrized_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}
