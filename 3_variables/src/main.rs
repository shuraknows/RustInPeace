fn main() {
    // ===============================================================
    // ==================== Variables  ===============================
    // ===============================================================

    // Constants are always immutable
    // add mut if you like to make it mutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);


    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");


    // Shadowing with different types
    let spaces = "   ";

    println!("The value of spaces before shadowing is: \"{spaces}\"");

    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

    // ===============================================================
    // ==================== Data types ===============================
    // ===============================================================

    // Scalar types
    // =============

    // Integer types
    // u = unsigned, i = signed
    // u8, i8,
    // u16, i16,
    // u32, i32,
    // u64, i64,
    // u128, i128
    // isize, usize - arch dependent (32 or 64 bit)

    // signed
    //-(2 ^ (n - 1)) to  2 ^ (n - 1) - 1

    // unsigned
    // 0 to 2 ^ n - 1
    // n is a bit length of the integer type

    // Floating point types
    // =====================

    // f32, f64
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Numeric operations
    // =====================


    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;


    // Boolean type
    // ============

    let _t = true;
    let _f: bool = false; // with explicit type annotation


    // Character type
    // ==============

    // ! single quotes
    // char type is 4 bytes in size and represents a Unicode Scalar Value

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';



    // ===============================================================
    // Compound types
    // ===============================================================

    // Tuple type
    // ==========

    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // to get the value of a tuple
    let (_, y, _) = tup;
    println!("The value of y is: {}", y);


    // also we can get access to a tuple element by using a period (.)
    // followed by the index of the value we want to access
    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = tup2.0;
    let _six_point_four = tup2.1;
    let _one = tup2.2;

    // tuple without value (empty tuple)
    let _tup3: () = ();


    // Array type
    // ==========
    // An array is a collection of elements of the same type
    // fixed length

    let _arr1 = [1, 2, 3, 4, 5];
    let _arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October",  "November", "December"];

    // Accessing array elements

    let _arr3 = [3; 5]; // same as [3, 3, 3, 3, 3]
    let _arr_first = _arr3[0];
    let _arr_second = _arr3[1];



}