use std::collections::HashMap;
use std::env;
use std::fs;

fn read_file_and_return_two_lists(file_name: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        // Parse the first part into i32 and push to list1
        if let Some(part1) = parts.next() {
            list1.push(part1.parse::<i32>().expect("Failed to parse part1 as i32"));
        }

        // Parse the second part into i32 and push to list2
        if let Some(part2) = parts.next() {
            list2.push(part2.parse::<i32>().expect("Failed to parse part2 as i32"));
        }
    }

    (list1, list2)
}

fn calculate_weighted_sum(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    // Count occurrences of numbers in list2
    let mut counts = HashMap::new();
    for &num in list2 {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the weighted sum for list1
    list1
        .iter()
        .map(|&num| num * counts.get(&num).unwrap_or(&0))
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (list1, list2) = read_file_and_return_two_lists(&args[1]);

    let sum = calculate_weighted_sum(&list1, &list2);

    println!("The similarity score is: {}", sum);
}
