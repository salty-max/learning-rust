#![allow(unused_variables)]
fn main() {
    {                       // s in not valid here, it's not yet declared
        let mut s = String::from("hello");    // s is valid from this point forward
        s.push_str(", world!");
        println!("{}", s);
    }                       // this scope is now over, s is no longer valid

    let x = 5;  // Bind the value 5 to x
    let y = x;  // Copy x value to y
    println!("x is {}, y is {}", x, y);
    // Copy is possible because an integer value has a known, fixed size
    // and is stored on the stack

    // Move example
    {
        let s1 = String::from("hello");
        let s2 = s1; // value owned by s1 is moved to s2, making s2 invalidated
        
        // Because s1 is now invalidated, it cannot be borrowed anymore
        // println!("s1 is: {}", s1); // This would fail
    }

    // Clone example
    {
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // Creates a copy of s1 value into s2

        println!("s1 is: {}, s2 is {}", s1, s2);
    }
}
