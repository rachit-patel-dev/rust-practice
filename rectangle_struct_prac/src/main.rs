#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 20,
    };

    println!("The area of rectangle is {}", rect1.area());
}
