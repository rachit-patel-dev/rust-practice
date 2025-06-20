fn main() {
    println!("Hello, this is the Vector practise!");

    // let v: Vec<i32> = Vec::new();
    // let v: Vec<i32> = vec![1, 2, 3];

    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The 3rd element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("There is no 3rd element"),
    }

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i)
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i *= 10;
    }
}
