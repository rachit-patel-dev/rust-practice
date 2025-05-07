use std::vec;

fn main() {
    println!("I'm the OWNER.");

    first();

    second();

    third();

    fourth();

    fifth();
}

fn first() {
    // Using Copy trait

    let x = 10;
    println!("The value of X is {}", x);

    let y = x;
    println!("The value of Y is {}", y);
}

fn second() {
    // Using Copy trait

    let a = "Hello";
    println!("The value of A is {}", a);

    let b = a;
    println!("The value of B is {}", b);
}

fn third() {
    // Ownership transferred to y and x no longer exists
    let x = vec![1, 2, 3];
    let y = x;
    println!("The value of Y vector is {:?}", y);

    // Ownership retain with x and y is a new separate owner
    let x = vec![1, 2, 3];
    let y = x.clone();
    println!("The value of new Y vector is {:?}", y);
    println!("The value of existing X vector is {:?}", x);
}

fn fourth() {
    // Using clone()

    fn modify_vec(mut vec: Vec<i32>) -> Vec<i32> {
        vec.push(4);
        vec
    }

    let x = vec![1, 2, 3];
    let y = modify_vec(x.clone());
    let z = x;

    println!("The value of modified vector Y is {:?}", y);
    println!("The value of original reference vector Z is {:?}", z);
}

fn fifth() {
    // Using clone_from()

    let x = vec![1, 2, 3];
    let mut y = vec![0; x.len()];
    y.clone_from(&x);

    println!("The value of X is {:?}", x);
    println!("The value of Y is {:?}", y);
}
