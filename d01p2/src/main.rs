use std::env;
use std::fs;

fn main() {
    let spelled_numbers = [
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
    let unspelled_numbers = [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9"
    ];

    // first argument is the input file
    let input_filename = env::args().nth(1)
        .expect("First argument should be path to file"); 
    let input = fs::read_to_string(input_filename)
        .expect("Should have been able to read the file");

    // sum first and last digit
    let sum: usize = input.lines()
        .map(|line| {
            println!("{}", line);

            let mut line = String::from(line);
            // replace first spelled digit
            'first_spelled_digit_search: for window_right_side in 0..line.len() {
                for i in 0..9 {
                    let window = &line[0..window_right_side];
                    match window.find(spelled_numbers[i]) {
                        Some(p) => {
                            println!("{} {}", spelled_numbers[i], p);
                            line = line.replacen(spelled_numbers[i], unspelled_numbers[i], 1);
                            println!("{}", line);
                            break 'first_spelled_digit_search;
                        }
                        None => ()
                    }
                }
            }

            println!("{}\n", line);

            // concat first and last digits
            let mut first_last_digits = String::new();
            first_last_digits.push(
                // first digit
                line.chars().find(|c| c.is_digit(10))
                    .expect("every line should cointains 1 number at least")
            );
            first_last_digits.push(
                // last digit
                line.chars().rev().find(|c| c.is_digit(10))
                    .expect("every line should cointains 1 number at least")
            );

            first_last_digits.parse::<usize>().unwrap()
        })
        .sum();
    
    println!("output: {:?}", sum);
}
