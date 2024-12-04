use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn calculate_sum(input: &str) -> i64 {
    // Define regex patterns for mul, do, and don't instructions
    let mul_re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut total_sum = 0;
    let mut mul_enabled = true; // At the start, `mul` instructions are enabled.

    // Process each token in the input line
    let tokens = input.split(|c: char| !c.is_alphanumeric() && c != '(' && c != ',' && c != ')');
    for token in tokens {
        if do_re.is_match(token) {
            // Enable future mul instructions
            mul_enabled = true;
        } else if dont_re.is_match(token) {
            // Disable future mul instructions
            mul_enabled = false;
        } else if mul_enabled {
            // Process mul instructions only if enabled
            if let Some(caps) = mul_re.captures(token) {
                if let (Ok(x), Ok(y)) = (caps[1].parse::<i64>(), caps[2].parse::<i64>()) {
                    total_sum += x * y;
                }
            }
        }
    }

    total_sum
}

fn process_file(filename: &str) -> io::Result<i64> {
    // Open the file and read its content line by line
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // Combine all lines into a single string
    let content = reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .join("\n");

    // Calculate the sum using the provided logic
    Ok(calculate_sum(&content))
}

fn main() -> io::Result<()> {
    let filename = "input.txt"; // Replace with your file name

    match process_file(filename) {
        Ok(total_sum) => println!("Sum of all multiplications: {}", total_sum),
        Err(e) => eprintln!("Error processing file: {}", e),
    }

    Ok(())
}
