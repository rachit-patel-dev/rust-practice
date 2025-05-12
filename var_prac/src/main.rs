fn main() {
    println!("Hello, Var!");

    let mut a = 10;
    println!("{}", a);

    a = 20;
    println!("{}", a);

    let b = "My name is John!";
    println!("{}", b);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECINDS: u32 = 60 * 60 * 3;
    println!("3 hours = {} seconds", THREE_HOURS_IN_SECINDS);

    let q = 5;
    let q = q + 1;
    {
        let q = q * 2;
        println!("The inner value of q is {}", q);
    }
    println!("The outer value of q is {}", q);
}
