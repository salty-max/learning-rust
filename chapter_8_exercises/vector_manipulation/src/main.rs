use std::collections::HashMap;

fn main() {
    let v1 = vec![1, 1, 4, 3, 3, 2, 4, 3, 5];
    let v2 = vec![1, 1, 4, 3, 4, 2, 4, 6, 5, 0];

    println!("v1 => {:?}", v1);
    println!("Mean of v1: {}", mean(&v1));
    println!("Median of v1: {}", median(&v1));
    println!("Mode of v1: {}", mode(&v1));

    println!("\n");

    println!("v2 => {:?}", v2);
    println!("Mean of v2: {}", mean(&v2));
    println!("Median of v2: {}", median(&v2));
    println!("Mode of v2: {}", mode(&v2));
}

fn mean(v: &[i32]) -> f64 {
    // 1. Sum the numbers in the vector
    // 2. Divide by the length of the vector

    let sum = v.iter().sum::<i32>() as f64;

    sum / v.len() as f64
}

fn median(v: &[i32]) -> f64 {
    // 1. Sort the vector
    // 2. Return middle number
    // If the vector has en even length then
    // return the mean of the two middle numbers

    let mut sorted = v.to_vec();
    let middle = sorted.len() / 2;

    sorted.sort_unstable();

    match sorted.len() % 2 {
        0 => mean(&[sorted[middle - 1], sorted[middle]]),
        _ => sorted[middle] as f64,
    }
}

fn mode(v: &[i32]) -> i32 {
    let mut map = HashMap::new();
    let mut max_occur = 0;
    let mut mode = 0;

    for &i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    for (key, value) in map {
        if value > max_occur {
            max_occur = value;
            mode = key;
        }
    }
    mode
}
