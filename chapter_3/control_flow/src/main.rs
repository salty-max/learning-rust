#![allow(dead_code)]
fn main() {
    let x = 25;
    println!("Is x greater than 5?, {}", greater_than_five(x));

    is_divisible(x);

    // let is_positive = if x >= 0 { true } else { false }; 
    let is_positive = x >= 0; 
    println!("Is x positive? {}", is_positive);

    count_to_10();
    countdown_from(10);
    countdown_from_five();
    countdown_from_three();
}

fn greater_than_five(x: i32) -> bool {
    x > 5
    // if x > 5 {
    //     true
    // } else {
    //     false
    // }
}

fn is_divisible(x: i32) {
    if x % 4 == 0 {
        println!("{} is divisible by 4", x);
    } else if x % 3 == 0 {
        println!("{} is divisible by 3", x);
    } else if x % 2 == 0 {
        println!("{} is divisible by 2", x);
    } else {
        println!("{} is not divisible by 2, 3 or 4", x);
    }
}

fn infinite_loop() {
    loop {
        println!("Again!");
    }
}

fn count_to_10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result)
}

fn countdown_from(x: u32) {
    let mut number = x;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Happy New Year!");
}

fn countdown_from_five() {
    let mut numbers = [1,2,3,4,5];
    numbers.reverse();

    for number in numbers.iter() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!")
}

fn countdown_from_three() {
    for number in (1..=3).rev() {
        match number {
            3 => println!("Ready..."),
            2 => println!("Set..."),
            _ => println!("GO!!!"),
        }
    }
}
