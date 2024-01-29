use std::env;
use std::fs;

fn main() {
    // first argument is the input file
    let input_filename = env::args().nth(1)
        .expect("First argument should be path to file"); 

    let input = fs::read_to_string(input_filename)
        .expect("Should have been able to read the file");

    let sum: usize = input.lines()
        .map(|line| {
            let mut first_last_digits = String::new();
            first_last_digits.push(
                // first digit
                line.chars().find(|c| c.is_digit(10)).unwrap()
            );
            first_last_digits.push(
                // last digit
                line.chars().rev().find(|c| c.is_digit(10)).unwrap()
            );

            first_last_digits.parse::<usize>().unwrap()
        })
        .sum();
    
    println!("output: {:?}", sum);
}
