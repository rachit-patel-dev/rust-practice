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
}
