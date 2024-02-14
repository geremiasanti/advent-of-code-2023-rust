use std::env;
use std::fs;

fn main() {
    // first argument is the input file
    let input_filename = env::args()
        .nth(1)
        .expect("First argument should be path to file");
    let input = fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let id_sum: usize = input
        .lines()
        .map(|game| {
            let (game_title, subsets) = game
                .split_once(':')
                .expect("Each game should have title and plays, separated by colon");
            dbg!(game_title, subsets);

            let (_, game_id) = game_title
                .split_once(" ")
                .expect("Game title should be formatted as \"Game <number>\"");
            dbg!(game_id);

            //let (max_red, max_blue, max_green) =
            _ = subsets.split(";").map(|subset| {
                dbg!(subset);
                subset.split(",")
            });

            0
        })
        .sum();

    println!("output: {:?}", id_sum);
}
