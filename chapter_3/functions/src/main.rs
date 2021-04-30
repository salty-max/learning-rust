#![allow(unused_variables)]
fn main() {
    println!("Hello, world!");

    another_function(5, 5);
    expression_example();

    println!("Five is {}", five());
    println!("10 + 1 = {}", plus_one(10));

}

fn another_function(x: u32, y: u32) {
    println!("The sum of x and y is: {}", x + y);
}

fn expression_example() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
