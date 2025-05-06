fn main() {
    println!("This is a LOOOOOOOOOOOOOOOP");

    first();

    second();

    third();
}

fn first() {
    let mut x = 0;

    loop {
        x += 1;
        println!("X is {}", x);

        if x == 5 {
            println!("Value matched!");
            break;
        }
    }
}

fn second() {
    let arr = [10, 20, 30, 40, 50, 60, 70];

    let mut index = 0;

    while index < 5 {
        println!("The value is {}", arr[index]);
        index += 1;
    }
}

fn third() {
    for x in 1..11 {
        println!("The X is {}", x);
        continue;
    }
}
