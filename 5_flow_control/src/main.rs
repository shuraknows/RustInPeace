fn main() {

    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // number should be boolean
    // otherwise error will be thrown
    //
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }



    // multiple conditions

    let number = 6;

    // just in case multiple conditions are bad practice
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // we can assign a value to a variable based on a condition
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");


    // in this case, the types of the different arms must be the same
    // otherwise, the compiler will throw an error
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    // ###########################################
    // ############## LOOPS ######################

    // will run forever
    // loop {
    //     println!("again!");
    // }


    // return values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    // YOU CAN CONTROL MULTIPLE LABELS USING
    // LOOP LABELS


    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");



    // WHILE LOOPS
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");



    // FOR WITHOUT FOR
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }


    // ACTUALLY FOR
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }


    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
