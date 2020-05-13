#[macro_use] extern crate prettytable;

use prettytable::{Table, format};
use std::collections::HashMap;
use std::io::{self, stdin, stdout, Write};

fn main() {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    welcome_message();
    loop {
        let cmd = input(">> ").expect("Error reading input");
        let cmd: Vec<&str> = cmd.split_whitespace().collect();
        if cmd.len() > 0 {
            match cmd[0] {
                "add" => add_emp(&cmd, &mut data),
                "retrieve" => retrieve(&cmd, &mut data),
                "quit" => {
                    println!("Bye!");
                    break;
                }
                _ => println!("Unknown command"),
            }
        }
    }
}

fn add_emp(cmd: &Vec<&str>, map: &mut HashMap<String, Vec<String>>) {
    if cmd.len() == 4 && cmd[2] == "to" {
        let employee = cmd[1].to_lowercase();
        let department = cmd[3].to_lowercase();
        map.entry(department).or_insert(vec![]).push(employee);
    } else {
        println!("ADD <employee> TO <department>")
    }
}

fn retrieve(cmd: &Vec<&str>, map: &mut HashMap<String, Vec<String>>) {
    if cmd.len() == 2 {
        let dep = cmd[1];
        if dep == "all" {
            retrieve_all(map);
        } else {
            retrieve_dep(dep, map);
        }
    } else {
        println!("RETRIEVE <department> or RETRIEVE ALL")
    }
}

fn retrieve_all(map: &mut HashMap<String, Vec<String>>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Employee", "Department"]);
    for (dep, v) in map.iter_mut() {
        v.sort();
        for employee in v.iter() {
            table.add_row(row![employee, dep]);
        }
    }
    print!("\n");
    table.printstd();
    print!("\n");
}

fn retrieve_dep(dep: &str, map: &HashMap<String, Vec<String>>) {
    match map.get(dep) {
        Some(value) => {
            for employee in value {
                println!("\n· {}", employee);
            }
        },
        None => println!("The department {} is not registered", dep),
    }
}

fn welcome_message() {
    println!("\nVALID COMMANDS: ");
    println!("\n · ADD <employee> TO <department> -> Add a new employee to a department (If the department doesn't exists it will be created)");
    println!("\n · RETRIEVE <department> -> Show a list of the people in the department");
    println!("\n · RETRIEVE ALL -> Show a list of all the people in the company by department, sorted alphabetically");
    println!("\n · QUIT -> Stop the execution");
    print!("\n");
}

fn input(message: &str) -> io::Result<String> {
    print!("{}", message);
    stdout().flush()?;
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim().to_string().to_lowercase();
    Ok(buffer)
}
