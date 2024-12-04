use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_line_valid(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if numbers.len() < 2 {
        return false; // A single number or empty line is not valid.
    }

    let increasing = numbers
        .windows(2)
        .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
    let decreasing = numbers
        .windows(2)
        .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);

    increasing || decreasing
}

fn count_valid_lines(filename: &str) -> io::Result<usize> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let valid_lines_count = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| is_line_valid(line))
        .count();

    Ok(valid_lines_count)
}

fn main() -> io::Result<()> {
    let filename = "input.txt"; // Replace with your file name.
    match count_valid_lines(filename) {
        Ok(count) => println!("Valid lines count: {}", count),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    Ok(())
}
