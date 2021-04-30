use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io};

fn main() {
    let mut secret_number = String::new();
    println!("Please enter the secret number");

    io::stdin()
        .read_line(&mut secret_number)
        .expect("Failed to read line");

    let secret_number: i32 = secret_number
        .trim()
        .parse()
        .expect("Failed to parse string");

    println!("The secret number is {}", secret_number);

    let mut min_guess = 1;
    let mut max_guess = 100;
    let mut guess_count = 0;

    loop {
        let guess = thread_rng().gen_range(min_guess..=max_guess);
        println!("Computer guessed {}", guess);
        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Too big");
                max_guess = guess;
            }
            Ordering::Less => {
                println!("Too small");
                min_guess = guess;
            }
            Ordering::Equal => {
                println!("Computer wins in {} turns", guess_count);
                break;
            }
        }
    }
}
