fn main() {
    println!("This is a LOOOOOOOOOOOOOOOP");

    first();

    second();

    third();

    fourth();
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

    // -------

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
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

    // ------

    for number in (1..4).rev() {
        println!("The number is {}", number);
    }
}

fn fourth() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);
}
