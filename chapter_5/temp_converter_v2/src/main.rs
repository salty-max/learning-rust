use std::io;

#[derive(Debug)]
struct Temperature {
    value: f64,
    unit: char,
}

impl Temperature {
    fn new(value: f64, unit: char) -> Temperature {
        Temperature { value, unit }
    }
}

fn main() {
    println!("Enter the desired temperature in celsius or fahrenheit (i.e. 32F):");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        let temperature = get_temperature_from_string(input);

        match temperature.unit {
            'F' => {
                println!(
                    "{}°F => {}°C",
                    temperature.value,
                    fahrenheit_to_celsius(temperature.value)
                );
                break;
            }
            'C' => {
                println!(
                    "{}°C => {}°F",
                    temperature.value,
                    celsius_to_fahrenheit(temperature.value)
                );
                break;
            }
            _ => {
                println!("Please enter a valid temperature (i.e. 32F)");
                continue;
            }
        }
    }
}

fn get_temperature_from_string(temp: &str) -> Temperature {
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

    Temperature::new(value, unit)
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}
