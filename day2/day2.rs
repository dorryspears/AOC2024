use std::{fs::read_to_string, io};

fn is_increasing_only(numbers: Vec<i32>) -> bool {
    if numbers.is_empty() {
        return true;
    }

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

fn is_degreasing_only(numbers: Vec<i32>) -> bool {
    if numbers.is_empty() {
        return true;
    }

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff > -1 || diff < -3 {
            return false;
        }
    }

    true
}

fn main() -> io::Result<()> {
    let file_name = "day2/2_input.txt";

    let input = read_to_string(file_name)?;
    let lines = input.lines();
    let mut lists: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        lists.push(numbers);
    }

    let mut count: i32 = 0;
    for nums in lists {
        for i in 0..nums.len() {
            let mut tmp: Vec<i32> = Vec::new();
            for (j, &num) in nums.iter().enumerate() {
                if j != i {
                    tmp.push(num);
                }
            }

            if is_increasing_only(tmp.clone()) || is_degreasing_only(tmp) {
                count += 1;
                break;
            }
        }
    }
    println!("Count: {}", count);

    Ok(())
}
