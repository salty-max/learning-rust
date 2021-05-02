use std::io;

fn main() {
    println!("What Fibonacci number would you like?");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Please input a number."),
    };

    let fibonacci = nth_fibonacci(n);

    println!("The {} fibonacci number is {}", n, fibonacci);
}

fn nth_fibonacci(n: u32) -> u32 {
    // 0 1 1 2 3 5 8 13 21 34 55 89
    if n == 1 {
        return 1;
    } else if n == 0 {
        return 0;
    }

    nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}
