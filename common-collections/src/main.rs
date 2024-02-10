use std::collections::HashMap;
use rand::seq::IteratorRandom;
use dialoguer::{Select, Input, Confirm};
use dialoguer::theme::ColorfulTheme;

fn main() {
    // 1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let mut integers: Vec<usize> = vec![1, 3, 4, 6, 7, 32, 2 , 11, 2, 2, 8];

    integers.sort(); // sort the vector

    println!("Sorted vec: {integers:?}");

    let mid_pos = integers.len() / 2;
    println!("Median is: {}", integers[mid_pos]);

    // Find the occurrence of every value
    let mut occurrence: HashMap<usize, usize> = HashMap::new();

    for val in &integers {
        let item = occurrence.entry(*val).or_insert(0);
        *item += 1;
    }
    
    let mut max_val: usize = 0;
    let mut max_occ: usize = 0;
    for (key, val) in occurrence {
        if val > max_occ {
            max_occ = val;
            max_val = key;
        }
    }
    println!("Mode is: {max_val}");

    // 2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, 
    // so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead 
    // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

    let sentence = "This is my test pig latin converter";
    let mut pig_sentence = String::new();

    for word in sentence.split_whitespace() {
        // if starts with vowel
        if word.to_lowercase().starts_with(['a', 'e', 'i', 'o', 'u']) {
            pig_sentence.push_str(word);
            pig_sentence.push_str("-hay ");
        } else {
            let mut word_it = word.chars();
            let first = word_it.next().unwrap();
            pig_sentence.push_str(word_it.as_str());
            pig_sentence.push('-');
            pig_sentence.push_str(&first.to_lowercase().to_string()[..]);
            pig_sentence.push_str("ay ");
        }
    }
    println!("{pig_sentence}");

    // 3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department 
    // or all people in the company by department, sorted alphabetically.

    let mut employees: HashMap<String, String> = HashMap::new();
    let departments = ["HR", "Cleaning", "IT", "Sales", "Customer support"]; 
    let rand_names = ["Ronald", "Abel", "Mirin", "Yule", "Brian"];
    let rand_surnames = ["Smithson", "Juan", "Roller", "Bowl", "Kale"];
    let actions = ["Add employee", "Show all employees", "Show employees of department", "Exit"];

    // Add some initial employees with rand crate
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let name: &str = rand_names.iter().choose(&mut rng).unwrap();
        let surname: &str = rand_surnames.iter().choose(&mut rng).unwrap();
        let department: &str = departments.iter().choose(&mut rng).unwrap();
    
        employees.insert(String::from(format!("{name} {surname}")), String::from(department.to_string()));
    }

    loop {
        let action = Select::with_theme(&ColorfulTheme::default()).items(&actions).default(0)
            .interact().unwrap();

        match action {
            0 => add_employee(&mut employees, &departments),
            1 => show_all(&employees),
            2 => show_department(&employees, &departments),
            3 => break,
            _ => {
                println!("Undefined selection");
                break;
            }
        };
    }
}

fn add_employee(employees: &mut HashMap<String, String>, deps: &[&str]) {
    let name: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Name and surname of employee")
        .interact_text().unwrap();
    let dep = Select::with_theme(&ColorfulTheme::default()).items(deps).default(0).interact().unwrap();
    let dep = deps[dep];

    match employees.insert(name, String::from(dep)) {
        Some(old_dep) => println!("Employee has been moved from {}", old_dep),
        None => println!("New employee added!"),
    };
}

fn show_all(employees: &HashMap<String, String>) {
    let mut emp_vec: Vec<(&String, &String)> = employees.iter().collect();
    emp_vec.sort_by(|a, b| a.1.cmp(b.1));
    
    println!("---");
    for (name, dep) in emp_vec {
        println!("{}: {}", name, dep);
    }
    println!("---");
}

fn show_department(employees: &HashMap<String, String>, deps: &[&str]) {
    let dep = Select::with_theme(&ColorfulTheme::default()).items(deps).default(0).interact().unwrap();
    let filtered_employees = employees.iter().filter(|(_, val)| val.eq(&deps[dep]));
    let mut filter_vec: Vec<(&String, &String)> = filtered_employees.collect();
    filter_vec.sort_by(|a, b| a.0.cmp(b.0));

    println!("---");
    for (name, dep) in filter_vec {
        println!("{}: {}", name, dep);
    }
    println!("---");
}