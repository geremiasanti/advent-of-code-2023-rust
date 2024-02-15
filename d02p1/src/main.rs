use std::env;
use std::fs;

#[derive(Debug)]
struct Game {
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

impl Game {}

fn main() {
    // first argument is the input file
    let input_filename = env::args()
        .nth(1)
        .expect("First argument should be path to file");
    let input = fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let id_sum: usize = input
        .lines()
        .map(|game_str| {
            let (game_title, subsets) = game_str
                .split_once(':')
                .expect("Each game should have title and plays, separated by colon");

            let (_, game_id) = game_title
                .split_once(" ")
                .expect("Game title should be formatted as \"Game <number>\"");

            dbg!(game_id, subsets);

            //let (max_red, max_blue, max_green) =
            let game = subsets.split(";").fold(
                Game {
                    max_red: 0,
                    max_green: 0,
                    max_blue: 0,
                },
                |game, subset| {
                    dbg!(subset);
                    _ = subset
                        .split(",")
                        .map(|cube_group| dbg!(cube_group))
                        .collect::<Vec<&str>>();
                    game
                },
            );
            dbg!(game);

            0
        })
        .sum();

    println!("output: {:?}", id_sum);
}
