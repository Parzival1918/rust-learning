use std::collections::HashMap;

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
}
