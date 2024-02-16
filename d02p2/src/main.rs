use std::env;
use std::fs;

#[derive(Debug)]
struct Game {
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

fn main() {
    // first argument is the input file
    let input_filename = env::args()
        .nth(1)
        .expect("First argument should be path to file");
    let input = fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let power_sum: usize = input
        .lines()
        .map(|game_str| {
            let (_, subsets) = game_str
                .split_once(':')
                .expect("Each game should have title and plays, separated by colon");

            //let (max_red, max_blue, max_green) =
            let minimum_cubes_set = subsets.split(";").fold(
                Game {
                    max_red: 0,
                    max_green: 0,
                    max_blue: 0,
                },
                |game, subset| {
                    subset.split(",").fold(game, |mut game, cube_group| {
                        let mut cube_group = cube_group.to_string();
                        cube_group.remove(0);
                        let (amount, color) = cube_group
                            .split_once(" ")
                            .expect("cube group should be formatted as \"<amount> <color>\"");
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
            dbg!(&minimum_cubes_set);
            dbg!(
                minimum_cubes_set.max_red
                    * minimum_cubes_set.max_green
                    * minimum_cubes_set.max_blue
            )
        })
        .sum();

    println!("output: {:?}", power_sum);
}
