#![allow(unused_variables)]
#![allow(clippy::ptr_arg)]
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is: {}", s1, len);

    let mut s2 = String::from("Bonjour");
    exclamate(&mut s2);
    println!("{}", s2);

    let r1 = &mut s2;
    // let r2 = &mut s2;    // Only one mutable reference allowed per scope
    
    // println!("{}, {}", r1, r2);

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.

fn exclamate(some_string: &mut String) {    // s is a mutable reference to a String
    some_string.push_str(" !");             // Here, the referenced string is modified
}

fn dangle() -> &String {    // dangle returns a reference to a String
    let s = String::from("Hello");  // s is a new String

    &s  // returns a reference to a String, s
}   // Here, s goes out of scope, and is dropped. Its memory goes away. DANGER!

 