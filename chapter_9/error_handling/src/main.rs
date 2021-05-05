use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    // let _f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {}", e),
    //         },
    //         other => {
    //             panic!("Problem opening the file: {:?}", other)
    //         }
    //     },
    // };

    // Cleaner version suing closures
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {}", error);
            })
        } else {
            panic!("Problem opening the file: {}", error)
        }
    });
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
