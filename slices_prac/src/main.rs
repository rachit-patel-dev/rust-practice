fn main() {
    println!("This is the SLICE practise!");

    slice_the_string();

    let my_string: String = String::from("Hello World");

    let word = get_first_word(&my_string[0..6]);
    println!("{}", word);
    let word = get_first_word(&my_string[..]);
    println!("{}", word);
    let word = get_first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    let word = get_first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = get_first_word(&my_string_literal[..]);
    println!("{}", word);
    let word = get_first_word(my_string_literal);
    println!("{}", word);
}

fn slice_the_string() {
    let main_str: String = String::from("Hello How Are You?");

    let first_word: &str = &main_str[0..5];
    println!("First Word = {}", first_word);

    let last_word: &str = &main_str[14..17];
    println!("Last Word = {}", last_word);

    let alt_first_word: &str = &main_str[..5];
    println!("First Word Again = {}", alt_first_word);

    let alt_last_word: &str = &main_str[14..];
    println!("Last Word Again = {}", alt_last_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
