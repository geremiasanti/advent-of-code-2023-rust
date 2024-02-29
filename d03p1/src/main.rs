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

    fn clear(&mut self) {
        self.buffer.clear();
        self.is_empty = true;
        self.is_part = false;
    }
}

pub trait CanBeSymbol {
    fn is_symbol(&self) -> bool;
}

impl CanBeSymbol for char {
    fn is_symbol(&self) -> bool {
        *self != '.' && !self.is_digit(10)
    }
}
impl CanBeSymbol for u8 {
    fn is_symbol(&self) -> bool {
        *self != b'.' && !self.is_ascii_digit()
    }
}

fn main() {
    // first argument is the input file
    let input_filename = env::args()
        .nth(1)
        .expect("First argument should be path to file");
    let input = fs::read_to_string(input_filename).expect("Should have been able to read the file");

    if !input.is_ascii() {
        panic!("String should be ascii only");
    }

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
        .map(|((prev_line, line), next_line)| (prev_line.as_bytes(), line, next_line.as_bytes()))
        .map(|(prev_line_bytes, line, next_line_bytes)| {
            dbg!(line);

            let mut part_buffer = PartBuffer::new();
            for (i, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    part_buffer.push(c);

                    let near_symbol = prev_line_bytes[i - 1..i + 1].into_iter();
                    dbg!(near_symbol);
                    /*
                    if !part_buffer.is_part && near_symbol {
                        part_buffer.is_part = true;
                    }
                    */
                }
                if c == '.' && !part_buffer.is_empty {
                    dbg!(&part_buffer);
                    part_buffer.clear();
                }
            }

            0
        })
        .sum();
    dbg!(parts_sum);
}
