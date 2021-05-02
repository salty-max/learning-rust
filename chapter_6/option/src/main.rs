fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    println!("5 + 1 = {:?}", six);

    let some_u8_value = 7u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        7 => println!("seven"),
        _ => (),
    }
    let another_u8_value = 5u8;
    only_prints_five(Some(another_u8_value));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn only_prints_five(x: Option<u8>) {
    if let Some(5) = x {
        println!("Oh, a five!");
    }
}
