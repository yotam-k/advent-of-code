use regex::{self, Captures, Regex};
use std::{cmp, fs, ops::Index};

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;
fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let input_split: Vec<&str> = input.split("\n").collect();

    part1(&input_split);
    part2(&input_split);
    Ok(())
}

fn part1(input_split: &Vec<&str>) {
    let mut total = 0;

    for line in input_split {
        let game_splits = line.split(":").collect::<Vec<&str>>();

        // Game ID is index 0 of the above split
        let game_id: u32 = game_splits
            .index(0)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();

        let re = Regex::new("( red| green| blue)").unwrap();

        // all of the game rounds are index 1
        let filtered_rounds = re.replace_all(game_splits.index(1), |capture: &Captures| {
            match &capture[0] {
                " red" => "R",
                " green" => "G",
                " blue" => "B",
                _ => panic!("We shouldn't get here"),
            }
        });

        let game_rounds = filtered_rounds
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let game_rounds = game_rounds.split(";").collect::<Vec<&str>>();
        let mut should_add = true;

        'outer: for round in game_rounds {
            let dice = round.split(",").collect::<Vec<&str>>();
            for die in dice {
                let num: u32 = die
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let color = die
                    .chars()
                    .filter(|c| !c.is_ascii_digit())
                    .collect::<String>();

                match color.as_str() {
                    "R" => should_add = num <= RED,
                    "G" => should_add = num <= GREEN,
                    "B" => should_add = num <= BLUE,
                    _ => panic!("Shouldn't get here"),
                }

                if !should_add {
                    break 'outer;
                }
            }
        }

        if should_add {
            total += game_id;
        }

        println!(
            "Game ID = {}, should add = {}, total = {}",
            game_id, should_add, total
        );
    }

    println!("Part 1 total is {}", total);
}

fn part2(input_split: &Vec<&str>) {
    let mut total = 0;

    for line in input_split {
        let game_splits = line.split(":").collect::<Vec<&str>>();

        // Game ID is index 0 of the above split
        let game_id: u32 = game_splits
            .index(0)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();

        let re = Regex::new("( red| green| blue)").unwrap();

        // all of the game rounds are index 1
        let filtered_rounds = re.replace_all(game_splits.index(1), |capture: &Captures| {
            match &capture[0] {
                " red" => "R",
                " green" => "G",
                " blue" => "B",
                _ => panic!("We shouldn't get here"),
            }
        });

        let game_rounds = filtered_rounds
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let game_rounds = game_rounds.split(";").collect::<Vec<&str>>();
        let (mut red_min, mut blue_min, mut green_min) = (0, 0, 0);

        for round in game_rounds {
            let dice = round.split(",").collect::<Vec<&str>>();
            for die in dice {
                let num: u32 = die
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let color = die
                    .chars()
                    .filter(|c| !c.is_ascii_digit())
                    .collect::<String>();

                match color.as_str() {
                    "R" => red_min = cmp::max(red_min, num),
                    "G" => green_min = cmp::max(green_min, num),
                    "B" => blue_min = cmp::max(blue_min, num),
                    _ => panic!("Shouldn't get here"),
                }
            }
        }

        total += red_min * green_min * blue_min;

        println!("Game ID = {}, total = {}", game_id, total);
    }

    println!("Part 2 total is {}", total);
}
