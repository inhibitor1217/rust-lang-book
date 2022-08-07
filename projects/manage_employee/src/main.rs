use std::{collections::HashMap, io};

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Input 'Add {{name}} to {{department}}' to add an employee to a department.");
        println!("Input 'Show {{department}}' to show all employees in a department.");
        println!("Input 'Show all' to show all employees in the company by department.");
        println!("Input 'quit' to exit.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let query = input.split_whitespace().collect::<Vec<_>>();

        match &query[..] {
            ["quit"] => break,
            ["Add", name, "to", department] => {
                employees
                    .entry(department.to_string())
                    .or_insert(Vec::new())
                    .push(name.to_string());
            }
            ["Show", "all"] => {
                for (department, members) in employees.iter() {
                    println!("{}: {}", department, members.join(", "));
                }
            }
            ["Show", department] => {
                if let Some(members) = employees.get(&department.to_string()) {
                    println!("{}: {}", department, members.join(", "));
                } else {
                    println!("No members in {}", department);
                }
            }
            _ => {
                println!("Please input a valid query.");
            }
        };
    }
}
