use std::{collections::HashMap, fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file_name = "day1/1.txt";
    // let file_name = "day1/test.txt";

    let input = read_to_string(file_name)?;
    let lines = input.lines();
    let mut lists_1: Vec<i32> = Vec::new();
    let mut lists_2: Vec<i32> = Vec::new();

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if numbers.len() == 2 {
            lists_1.push(numbers[0]);
            lists_2.push(numbers[1]);
        }
    }

    lists_1.sort();
    lists_2.sort();

    let mut hash_of_2: HashMap<i32, i32> = HashMap::new();

    let mut count: i32 = 0;

    let mut it1 = lists_1.iter().peekable();
    let mut it2 = lists_2.iter().peekable();

    while it2.peek().is_some() {
        let num2 = *it2.next().unwrap();

        if hash_of_2.contains_key(&num2) {
            let current_count = *hash_of_2.get(&num2).unwrap();
            hash_of_2.insert(num2, current_count + 1);
        } else {
            hash_of_2.insert(num2, 1);
        }
    }

    while it1.peek().is_some() {
        let num1 = *it1.next().unwrap();

        if hash_of_2.contains_key(&num1) {
            let freq = hash_of_2.get(&num1).unwrap();
            count += freq * num1;
        }
    }

    println!("{}", count);
    Ok(())
}
