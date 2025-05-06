fn main() {
    println!("Hello If-Else Condition!");

    first();

    second();

    third();
}

fn first() {
    let numb = 12;
    if numb > 10 {
        println!("The number is greater than 10.");
    } else {
        println!("The number is less than 10.");
    }
}

fn second() {
    let numb = 3;
    if numb % 2 == 0 {
        println!("Number is divisible by 2.");
    } else if numb % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if numb % 4 == 0 {
        println!("Number is divisible by 4.");
    } else {
        println!("Number is NOT divisible by 2, 3 & 4.");
    }
}

fn third() {
    let condition = true;
    let numb = if condition { 10 } else { 20 };
    println!("The value of numb is {}", numb);
}
