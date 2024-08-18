fn main() {
    // declared via fn
    // functions should have snake_case names
    another_function();
    parametrized_function(123);
    multiple_params_function("foo", 123);
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
