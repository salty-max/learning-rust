use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // get() returns an Option<T>
    if let Some(score) = score {
        println!("{} team score: {}", team_name, score);
    }

    scores.insert(team_name, 30); // Value override

    for (k, v) in &scores {
        println!("{} team score: {}", k, v);
    }

    scores.entry(String::from("Red")).or_insert(15); // Insert entry if not existing
    scores.entry(String::from("Blue")).or_insert(15); // Insert entry if not existing

    println!("{:#?}", scores);

    let text = "hello world wonderful world";

    let mut string_map = HashMap::new();

    // Update value based on an old one
    for word in text.split_whitespace() {
        let count = string_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", string_map);
}

fn _ownership_and_hashmaps() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // because values types that doesn't implement the copy trait
    // are moved into the hashmap on insert
    // println!("{}" field_name);   // This will not compile
}
