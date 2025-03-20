use std::collections::HashMap;

use crate::stdin_helper;

pub struct Interface {
    src: HashMap<String, Vec<String>>,
}
impl Interface {
    pub fn new() -> Self {
        Self {
            src: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        println!("+------------------------------------------+");
        println!("| Welcome to the Department Employee Names |");
        println!("+------------------------------------------+");
        println!("");

        let break_line = "+-------------------------+";
        loop {
            println!("{break_line}");
            println!("| Please choose an option |");
            println!("{break_line}");
            println!("| 1. Add                  |");
            println!("| 2. Retrieve             |");
            println!("{break_line}");

            match stdin_helper::request_valid_u8() {
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
        let employer = stdin_helper::stdin_str();

        println!("Please enter the department name: ");
        let department = stdin_helper::stdin_str();

        println!(
            "Inserting employer {} into department {}...",
            employer, department
        );
        self.src
            .entry(department.clone())
            .or_insert(Vec::new())
            .push(employer);

        if let Some(employers) = self.src.get_mut(&department) {
            employers.sort();
        }
        println!("success!\n");
    }

    fn retrieve(&self) {
        println!("Choose your option:\n1. All\n2. Department");
        match stdin_helper::request_valid_u8() {
            1 => self.retrieve_all(),
            2 => self.retrieve_depart(),
            _ => println!("Invalid option"),
        }
    }

    fn retrieve_all(&self) {
        let mut departments: Vec<&String> = self.src.keys().collect();
        departments.sort();

        for department in departments {
            self.print_depart(department);
        }
    }

    fn retrieve_depart(&self) {
        println!("Please choose a department: ");
        for dep in self.src.keys() {
            println!("{dep}");
        }

        let depart = stdin_helper::stdin_str();
        self.print_depart(&depart);
    }

    fn print_depart(&self, depart: &str) {
        if let Some(employers) = self.src.get(depart) {
            print!("\n");
            println!("| ------ Department: \"{depart}\" ------ |");
            print!("|\n");
            for (i, employer) in employers.iter().enumerate() {
                println!("| {}. {employer}", i + 1)
            }
            print!("\n");
        } else {
            println!("404: Department not found")
        }
    }
}
