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

fn sort_lists(list1_sorted: &mut Vec<i32>, list2_sorted: &mut Vec<i32>) {
    list1_sorted.sort();
    list2_sorted.sort();
}

fn calculate_differences(list1: &Vec<i32>, list2: &Vec<i32>) -> Vec<i32> {
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (mut list1, mut list2) = read_file_and_return_two_lists(&args[1]);
    sort_lists(&mut list1, &mut list2);
    let differences = calculate_differences(&list1, &list2);

    let sum: i32 = differences.iter().sum();

    println!("The sum is: {}", sum);
}
