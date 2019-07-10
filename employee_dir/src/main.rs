extern crate regex;

use regex::Regex;
use std::io;
use std::collections::HashMap;

fn main() {
    let add_re = Regex::new("Add (.*) to (.*)").expect("Couldn't compile add_re");
    let directory_re = Regex::new("Directory").expect("Couldn't compile directory_re");
    let list_re = Regex::new("List (.*)").expect("Couldn't compile list_re");
    let mut command = String::new();
    let mut department_dirs: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("\nPlease enter a command.");

        io::stdin().read_line(&mut command).expect("Failed to read line");

        if list_re.is_match(&command) {
            for cap in list_re.captures_iter(&command) {
                let dept = String::from(&cap[1]);
                if department_dirs.contains_key(&dept) {
                    println!("Employees in {}", &dept);
                    let dir = department_dirs.get_mut(&dept)
                        .expect("Couldn't get department dir");
                    dir.sort(); 
                    for employee in dir.iter() {
                        println!("  {}", employee);
                    }
                } else {
                    println!("Unknown department {}", &dept);
                }
            }
        } else if directory_re.is_match(&command) {
            println!("Employees by Department");
            for (dept, dir) in department_dirs.iter_mut() {
                println!("  {}", &dept);
                dir.sort(); 
                for employee in dir.iter() {
                    println!("    {}", employee);
                }
            }
        } else if add_re.is_match(&command) {
            for cap in add_re.captures_iter(&command) {
                let employee = String::from(&cap[1]);
                let dept = String::from(&cap[2]);
                let dir = department_dirs.entry(dept).or_insert(Vec::new());
                dir.push(employee);
                println!("Done.");
            }
        } else {
            println!("Command not understood.");
            println!("Commands:");
            println!("Add <employee> to <department>");
            println!("List <department>");
        }
        &command.clear();
    }
}
