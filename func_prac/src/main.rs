fn main() {
    println!("Hello, world!");

    // Simple fn call
    first_fn();

    // Fn call with single parameter
    second_fn(20);

    // Fn call with multiple parameters
    third_fn(30, 'A');

    // Fn call for expression
    expr_fn();

    // Fn call with return value
    let a = rtn_fn();
    println!("The returned value is {}", a);
}

// Simple fn
fn first_fn() {
    println!("This is the 1st fn!");
}

// Fn with single parameter
fn second_fn(x: i32) {
    println!("The value is {}", x);
}

// Fn with multiple parameters
fn third_fn(x: i32, y: char) {
    println!("The 1st value is {} and the 2nd value is {}", x, y);
}

// Expression
fn expr_fn() {
    let y = {
        let x = 15;
        x + 5
    };
    println!("The final value is {}", y);
}

// Fn with return value
fn rtn_fn() -> i32 {
    20 + 30
}
