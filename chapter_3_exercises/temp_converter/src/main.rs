use std::io;

fn main() {
    println!("Enter the desired temperature in celsius or fahrenheit (i.e. 32.0F):");

    let mut res = String::new();
    'main: loop {
        io::stdin()
            .read_line(&mut res)
            .expect("Unable to read line");

        let value = &res[..res.len() - 2];

        let temp: f64 = value.trim().parse().expect("Unable to parse string");
        let bytes = res.as_bytes();

        for &item in bytes.iter() {
            if item == b'F' {
                println!("{}°F is equal to {}°C", temp, to_celsius(temp));
                break 'main;
            } else if item == b'C' {
                println!("{}°C is equal to {}°F", temp, to_fahrenheit(temp));
                break 'main;
            }
        }
    }
}

fn to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}

fn to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}
