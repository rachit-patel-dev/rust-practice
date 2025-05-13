fn main() {
    println!("Data Types in RUST!");

    // Integer
    let int_no = 2;
    println!("Int no. is {}", int_no);

    // Boolean
    let is_okay = true;
    println!("Bool is {}", is_okay);

    // Float
    let flt_no = 12.58;
    println!("Float no. is {}", flt_no);

    // String
    let str_val = "Nice";
    println!("String is {}", str_val);

    // Char
    let char_val = "X";
    println!("Char is {}", char_val);

    // Tuple
    let mut tup: (i32, i32, u8, u16, f64) = (10, 20, 30, 40, 50.25);
    println!("Tuple is {:?}", tup);
    println!("Tuple value on 1st index is {}", tup.1);
    tup.1 = 25;
    println!("Tuple value on 1st index after change is {}", tup.1);

    // Array
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array is {:?}", arr);
    println!("Array value on 2nd index is {}", arr[2]);
    arr[2] = 2;
    println!("Array value on 2nd index after change is {}", arr[2]);

    // ------------

    // Float
    let x = 2.5; // f64
    let y: f32 = 3.12; // f32
    println!("{}", x);
    println!("{}", y);

    // Numeric Operations

    // Addition
    let sum = 5 + 10;
    println!("Sum = {}", sum);

    // Subtraction
    let diff = 95.4 - 4.3;
    println!("Diff = {}", diff);

    // Multiplication
    let multi = 4 * 30;
    println!("Multiply = {}", multi);

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("Quotient = {}", quotient);
    println!("Truncated = {}", truncated);

    // Remainder
    let remainder = 43 % 5;
    println!("Remainder = {}", remainder);

    // Bool
    let apple = true;
    let car: bool = false;
    println!("Apple is {}", apple);
    println!("Car is {}", car);

    // Char
    let small = 'z';
    let big: char = 'Z';
    let emoji = '🦀';
    println!("Small = {}", small);
    println!("Big = {}", big);
    println!("Emoji = {}", emoji);

    // Tuple
    let tup = (500, 34.5, 1);
    let (x, y, z) = tup;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!("First element = {}", tup.0);
    println!("Second element = {}", tup.1);
    println!("Third element = {}", tup.2);

    let mut a = (2, 5);
    a.0 = 3;
    a.1 *= 2;
    println!("Updated tuple = {:?}", a);
}
