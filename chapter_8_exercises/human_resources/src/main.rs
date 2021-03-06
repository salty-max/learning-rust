use std::{collections::HashMap, io};

#[derive(Debug)]
struct HR {
    pub employees_list: HashMap<String, Vec<String>>,
}

impl HR {
    fn new() -> HR {
        HR {
            employees_list: HashMap::new(),
        }
    }

    fn add_employee(&mut self, command: &str) -> String {
        let mut split_command = command.split_whitespace();
        let employee = split_command.nth(1);
        let department = split_command.last();

        if let Some(employee) = employee {
            let employee = capitalize(employee);
            if let Some(department) = department {
                let employees = self
                    .employees_list
                    .entry(department.to_lowercase())
                    .or_insert_with(Vec::new);
                employees.push(employee);

                return String::from("Employee successfully added");
            }
        }

        String::from("Failed to add employee")
    }

    fn get_department_personnel(&self, command: &str) -> Option<(String, &Vec<String>)> {
        let department = command.split_whitespace().nth(1);
        match department {
            Some(department) => match self.employees_list.get(&department.to_lowercase()) {
                Some(d) => Some((String::from(department), &d)),
                None => None,
            },
            None => None,
        }
    }
}

fn main() {
    let mut hr = HR::new();
    loop {
        println!("Enter a command (i.e. \"Add <Person> to <Department>\")");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.trim().to_lowercase();
        let instruction = command.split_whitespace().next();

        match instruction {
            Some("quit") => break,
            Some("get") => match hr.get_department_personnel(&command) {
                Some((department, list)) => println!("{}: {:?}", capitalize(&department), list),
                None => println!("No department with this name"),
            },
            Some("add") => println!("{}", hr.add_employee(&command)),
            _ => println!("Invalid command!"),
        }
    }
}

fn capitalize(s: &str) -> String {
    format!("{}{}", &s[0..1].to_uppercase(), &s[1..])
}
