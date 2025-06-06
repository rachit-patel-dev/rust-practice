#[derive(Debug)]
enum UsState {
    Alabama,
    // Alaska,
    // California,
    // Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("We're in Penny!");
            1
        }
        Coin::Nickel => {
            println!("We're in Nickel!");
            5
        }
        Coin::Dime => {
            println!("We're in Dime!");
            10
        }
        Coin::Quarter(state) => {
            println!("We're in Quarter!");
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    println!("Hello, world!");

    println!("1 Penny = {} Cents", value_in_cents(Coin::Penny));
    println!("1 Nickel = {} Cents", value_in_cents(Coin::Nickel));
    println!("1 Dime = {} Cents", value_in_cents(Coin::Dime));
    println!(
        "1 Quarter = {} Cents",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six = {:?}", six);
    println!("None = {:?}", none);

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alabama)) {
        println!("{desc}");
    }
}
