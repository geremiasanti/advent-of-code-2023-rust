use std::env;
use std::fs;
use regex::{Captures, Regex};

fn main() {
    // first argument is the input file
    let input_filename = env::args().nth(1)
        .expect("First argument should be path to file"); 
    let input = fs::read_to_string(input_filename)
        .expect("Should have been able to read the file");
    println!("{}", input);

    // replace spelled digits with digits chars
    let spelled_digits_regex = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let input = spelled_digits_regex.replace_all(&input, |captures: &Captures| {
        match &captures[0] {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!("unknown capture")
        }
    });
    println!("{}", input);

    // sum first and last digit
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

            println!("{}", first_last_digits);

            first_last_digits.parse::<usize>().unwrap()
        })
        .sum();
    
    println!("output: {:?}", sum);
}
