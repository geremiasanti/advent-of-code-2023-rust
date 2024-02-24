use std::env;
use std::fs;

fn main() {
    // first argument is the input file
    let input_filename = env::args()
        .nth(1)
        .expect("First argument should be path to file");
    let input = fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let parts_sum: usize = input
        .lines()
        .zip(input.lines().skip(1))
        .zip(input.lines().skip(2))
        .map(|((l0, l1), l2)| {
            let l1_parts_sum = 0;
            dbg!(l0, l1, l2);

            let mut current_part_buffer = String::new();
            for ((c0, c1), c2) in l0.chars().zip(l1.chars()).zip(l2.chars()) {
                dbg!(&c0, &c1, &c2);

                if c1.is_digit(10) {
                    current_part_buffer.push(c1);
                } else if !current_part_buffer.is_empty() {
                    current_part_buffer = String::new();
                }
                dbg!(&current_part_buffer);
            }

            dbg!(l1_parts_sum)
        })
        .sum();
}
