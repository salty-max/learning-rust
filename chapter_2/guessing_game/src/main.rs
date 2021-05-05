extern crate rand;
use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Guess {
    value: u8,
}

impl Guess {
    pub fn new(value: u8) -> Result<Guess, String> {
        if !(1..=100).contains(&value) {
            return Err(format!(
                "Guess value must be between 1 and 100, got {}",
                value
            ));
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(guess) => guess,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            },
            Err(_) => {
                println!("Please enter a valid value!");
                continue;
            }
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
