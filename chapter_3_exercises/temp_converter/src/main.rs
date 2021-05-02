use std::io;

fn main() {
    println!("Enter the desired temperature in celsius or fahrenheit (i.e. 32F):");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let temperature = input.trim();

        let (temp, unit) = get_temperature_from_string(temperature);

        match unit {
            'F' => {
                println!("{}°F => {}°C", temp, fahrenheit_to_celsius(temp));
                break;
            }
            'C' => {
                println!("{}°C => {}°F", temp, celsius_to_fahrenheit(temp));
                break;
            }
            _ => {
                println!("Please enter a valid temperature (i.e. 32F)");
                continue;
            }
        }
    }
}

fn get_temperature_from_string(temp: &str) -> (f64, char) {
    let bytes = temp.as_bytes();
    let mut value = 0.0;
    let mut unit = ' ';

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'F' || item == b'f' {
            value = temp[..i].parse().expect("Failed to parse");
            unit = 'F';
        } else if item == b'C' || item == b'c' {
            value = temp[..i].parse().expect("Failed to parse");
            unit = 'C';
        }
    }

    (value, unit)
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}
