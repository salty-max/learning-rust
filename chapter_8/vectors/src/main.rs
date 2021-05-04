fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);
    v1.push(6);

    let third = &v1[2];
    println!("The third element is {}", third);

    match v1.get(1) {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element"),
    }

    loop_over_vector();
    loop_over_mut_vector();

    // Storing value into an enum type allows to store values of
    // different types in a vector.
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

fn loop_over_vector() {
    let v3 = vec!["I", "am", "Groot"];
    for i in &v3 {
        println!("{}", i);
    }
}

fn loop_over_mut_vector() {
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{}", i);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
