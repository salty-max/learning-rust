#![allow(clippy::many_single_char_names)]
#![allow(unused_variables)]
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    
    // Conversion by shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length is: {}", spaces);
    
    let x = 2.;
    let y: f32 = 3.;
    let z = x + y;
    println!("The value of z is: {}", z);
    
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("The value of x is: {}, the value of y is: {}, and the value of z is: {}", x, y, z);
    println!("The value of a is: {}, the value of b is: {}, and the value of c is: {}", a, b, c);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let arr: [u8;5] = [1,2,3,4,5];

    let bunch_of_trees = [3;5];
}
