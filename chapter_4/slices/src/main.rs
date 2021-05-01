#![allow(unused_variables)]
fn main() {
    let s = String::from("Hello world!");
    let first = first_word(&s);
    let second = second_word(&s);

    println!("The first word in {} ends is `{}`", s, first);
    println!(
        "The second word in {} is `{}`",
        s, second
    );

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start_index = 0;
    let mut found_first = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && found_first {
            return &s[start_index..i];
        } else if item == b' ' && !found_first {
            start_index = i + 1;
            found_first = true;
        }
    }

    if found_first {
        &s[start_index..]
    } else {
        ""
    }
}
