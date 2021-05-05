use std::io;

fn main() {
    println!("Enter the text to convert:");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let pig = to_pig_latin(&input.trim());

        match pig {
            Some(s) => {
                println!("{}", s);
                break;
            }
            None => println!("You must enter at least one word"),
        }
    }
}

fn to_pig_latin(s: &str) -> Option<String> {
    let vowels = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u', 'Y', 'y'];
    let mut res = String::new();
    let mut word_count = 0;

    for w in s.split_whitespace() {
        let first = w.chars().next();

        if let Some(first) = first {
            if vowels.contains(&first) {
                res.push_str(w);
                res.push_str("-hay")
            } else {
                res.push_str(&w[1..]);
                res.push('-');
                res.push(first);
                res.push_str("ay");
            }

            if word_count != s.len() {
                res.push(' ');
            }

            word_count += 1;
        }
    }

    if !res.is_empty() {
        Some(res)
    } else {
        None
    }
}
