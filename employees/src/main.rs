use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company = HashMap::new();
    println!("Input your command and press enter to run.");
    println!("Run `Add Name to Department` to add employees.");
    println!("Run `List Company` to list all employees.");
    loop {
        print!("\n$> ");
        let _ = io::stdout().flush();
        let command = read_command().trim().to_string();
        if command.as_str() == "List Company" {
            println!("{:?}", company);
            continue;
        }

        let parts: Vec<&str> = command.split(" ").collect();
        if parts.len() != 4 {
            println!("Invalid Command!");
            continue;
        }

        if parts.first().unwrap() == &"Add" {
            let list = company
                .entry(parts.last().unwrap().to_string())
                .or_insert(vec![]);
            list.push(parts.get(1).unwrap().to_string());
            println!("Added.");
            continue;
        }
    }
}

fn read_command() -> String {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line.");
    command
}
