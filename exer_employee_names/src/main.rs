/*  https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
    for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
    department or all people in the company by department, sorted alphabetically.
*/

use std::{collections::HashMap, io};

fn main() {
    let mut interface = Interface::new();
    interface.run();
}

struct Interface {
    src: HashMap<String, Vec<String>>,
}
impl Interface {
    pub fn new() -> Self {
        Self {
            src: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the Department Employee Names");
        loop {
            println!("Please choose an option:\n1. Add\n2. Retrieve");
            match Self::stdin_valid_n() {
                1 => self.add(),
                2 => self.retrieve(),
                _ => {
                    println!("invalid option");
                    continue;
                }
            }
        }
    }

    fn add(&mut self) {
        println!("Please enter the employer name: ");
        let employer = Self::stdin_str();

        println!("Please enter the department name: ");
        let department = Self::stdin_str();

        println!(
            "Inserting employer {} into department {}...",
            employer, department
        );
        self.src
            .entry(department)
            .or_insert(Vec::new())
            .push(employer);

        println!("success!");
    }

    fn retrieve(&self) {
        loop {
            println!("Choose your option:\n1. All\n2. Department");
            match Self::stdin_valid_n() {
                1 => {
                    let mut departments: Vec<&String> = self.src.keys().collect();
                    departments.sort();

                    for department in departments {
                        print!("\n");
                        let mut employers = self.src[department].clone();
                        employers.sort();

                        println!("### Department {department} ###");
                        for (i, employer) in employers.iter().enumerate() {
                            println!("{}. {employer}", i + 1)
                        }
                        print!("\n");
                    }
                    break;
                }
                2 => {
                    println!("Please choose a department: ");
                    for dep in self.src.keys() {
                        println!("{dep}");
                    }
                    let depart = Self::stdin_str();
                    let mut employers = self.src[&depart].clone();
                    employers.sort();

                    print!("\n");
                    println!("### Department {depart} ###");
                    for (i, employer) in employers.iter().enumerate() {
                        println!("{}. {employer}", i + 1)
                    }
                    print!("\n");
                    break;
                }
                _ => {
                    println!("Invalid option");
                    continue;
                }
            }
        }
    }

    fn stdin_valid_n() -> u8 {
        loop {
            match Self::stdin_str().parse::<u8>() {
                Ok(val) => {
                    return val;
                }
                Err(_) => {
                    println!("Invalid number");
                    continue;
                }
            };
        }
    }

    fn stdin_str() -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        return s.trim().to_string();
    }
}
