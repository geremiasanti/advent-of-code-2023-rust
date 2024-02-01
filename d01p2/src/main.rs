use std::env;
use std::fs;

fn main() {
    let digits = [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    // first argument is the input file
    let input_filename = env::args().nth(1)
        .expect("First argument should be path to file"); 
    let input = fs::read_to_string(input_filename)
        .expect("Should have been able to read the file");

    // sum first and last digit
    let sum: usize = input.lines()
        .map(|line| {
            println!("\"{}\"", line);

            let mut first_last_digits = String::new();

            'first_digit: for window_right_side in 0..line.len() {
                let window = &line[..window_right_side];
                for (i, digit) in digits.iter().enumerate() {
                    if window.contains(digit) {
                        let first_digit = char::from_digit((i % 9 + 1) as u32, 10)
                            .expect("Should have found a single digit number");
                        first_last_digits.push(first_digit);
                        break 'first_digit;
                    }
                }
            }
            'last_digit: for window_left_side in (0..line.len()).rev() {
                let window = &line[window_left_side..line.len()];
                for (i, digit) in digits.iter().enumerate() {
                    if window.contains(digit) {
                        let last_digit = char::from_digit((i % 9 + 1) as u32, 10)
                            .expect("Should have found a single digit number");
                        first_last_digits.push(last_digit);
                        break 'last_digit;
                    }
                }
            }

            println!("first and last: {}", first_last_digits);

            first_last_digits.parse::<usize>()
                .expect("found digits not parsable to usize")
        })
        .sum();
    
    println!("output: {:?}", sum);
}
