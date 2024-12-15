use std::{
    fs::{self, read_to_string},
    io,
    ops::Add,
};

fn main() -> io::Result<()> {
    let file_name = "day1/1.txt";
    // let file_name = "day1/test.txt";

    let input = fs::read_to_string(file_name)?;
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

    let mut count: i32 = 0;

    let mut it1 = lists_1.iter().peekable();
    let mut it2 = lists_2.iter();

    while it1.peek().is_some() {
        let num1 = it1.next().unwrap();
        let num2 = it2.next().unwrap();

        // println!("{0} and {1}", num1, num2);
        count += (num1 - num2).abs();
    }

    println!("{}", count);
    Ok(())
}
