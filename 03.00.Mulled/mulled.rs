use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn calculate_sum(input: &str) -> i32 {
    // Define a regular expression to match `mul(x,y)` where x and y are integers
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    // Iterate through all matches and compute the total sum
    re.captures_iter(input)
        .filter_map(|caps| {
            let x = caps[1].parse::<i32>().ok();
            let y = caps[2].parse::<i32>().ok();
            x.and_then(|x| y.map(|y| x * y))
        })
        .sum()
}

fn process_file(filename: &str) -> io::Result<i32> {
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
