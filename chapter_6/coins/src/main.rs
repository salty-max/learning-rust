#![allow(dead_code)]

#[derive(Debug)]
enum UsState {
    California,
    Nebraska,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let penny = Coin::Penny;
    println!("A penny is worth {} cents.", value_in_cents(&penny));

    let quarter = Coin::Quarter(UsState::Nebraska);
    println!("A quarter is worth {} cents.", value_in_cents(&quarter));
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
