use std::env;
use std::fs;

#[derive(Debug)]
struct PartBuffer {
    buffer: String,
    is_empty: bool,
    is_part: bool,
}

impl PartBuffer {
    fn new() -> PartBuffer {
        PartBuffer {
            buffer: String::new(),
            is_empty: true,
            is_part: false,
        }
    }

    fn push(&mut self, c: char) {
        self.buffer.push(c);
        if self.is_empty {
            self.is_empty = false;
        }
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.is_empty = true;
        self.is_part = false;
    }
}

fn main() {
    // first argument is the input file
    let input_filename = env::args()
        .nth(1)
        .expect("First argument should be path to file");
    let input = fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let line_len = input
        .lines()
        .next()
        .expect("Input should have at least one line")
        .chars()
        .count();
    let mut empty_line = String::from_utf8(vec![b'.'; line_len]).unwrap();
    empty_line.push('\n');
    let input_extended = empty_line.clone() + &input + &empty_line;

    let parts_sum: usize = input_extended
        .lines()
        .zip(input_extended.lines().skip(1))
        .zip(input_extended.lines().skip(2))
        .map(|((_prev_line, line), _next_line)| {
            dbg!(line);

            let mut part_buffer = PartBuffer::new();
            for (_i, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    part_buffer.push(c);
                }
                if c == '.' && !part_buffer.is_empty {
                    dbg!(&part_buffer);
                    part_buffer.reset();
                }
            }

            0
        })
        .sum();
    dbg!(parts_sum);
}
