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
                    subset.split(",").fold(game, |mut game, cube_group| {
                        let mut cube_group = cube_group.to_string();
                        cube_group.remove(0);
                        let (amount, color) = cube_group
                            .split_once(" ")
                            .expect("cube group should be formatted as \"<amount> <color>\"");
                        dbg!(&cube_group, amount, color);
                        match color {
                            "red" => {
                                let amount = amount.parse::<usize>().expect(
                                    "cube group should be formatted as \"<amount> <color>\"",
                                );
                                if amount > game.max_red {
                                    game.max_red = amount;
                                }
                            }
                            "green" => {
                                let amount = amount.parse::<usize>().expect(
                                    "cube group should be formatted as \"<amount> <color>\"",
                                );
                                if amount > game.max_green {
                                    game.max_green = amount;
                                }
                            }
                            "blue" => {
                                let amount = amount.parse::<usize>().expect(
                                    "cube group should be formatted as \"<amount> <color>\"",
                                );
                                if amount > game.max_blue {
                                    game.max_blue = amount;
                                }
                            }
                            _ => panic!("colors should be red, green and blue only"),
                        };
                        game
                    })
                },
            );
            dbg!(game);

            0
        })
        .sum();

    println!("output: {:?}", id_sum);
}
