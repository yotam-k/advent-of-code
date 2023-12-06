use std::fs;

use aho_corasick::AhoCorasick;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let input_split: Vec<&str> = input.split("\n").collect();

    match part1(&input_split) {
        Ok(x) => println!("Part 1 ran successfully. Result is {}", x),
        Err(e) => println!("Error running part 1: {}", e),
    }

    match part2(&input_split) {
        Ok(x) => println!("Part 2 ran successfully. Result is {}", x),
        Err(e) => println!("Error running part 2: {}", e),
    }

    Ok(())
}

fn part1(trimmed_input: &Vec<&str>) -> std::io::Result<u32> {
    let mut sum = 0;

    for line in trimmed_input {
        let trim: String = line.chars().filter(|c| !c.is_alphabetic()).collect();
        let first_num: u32 = trim.chars().next().unwrap().to_digit(10).unwrap();
        let last_num: u32 = trim.chars().last().unwrap().to_digit(10).unwrap();
        let result = (first_num * 10) + last_num;
        sum += result;
    }

    Ok(sum)
}

fn part2(trimmed_input: &Vec<&str>) -> std::io::Result<u32> {
    let nums = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    let mut sum = 0;
    let aho_cora = AhoCorasick::new(nums).unwrap();

    for line in trimmed_input {
        let matches = aho_cora.find_overlapping_iter(line).collect::<Vec<_>>();
        let first_num = matches.iter().nth(0).unwrap().pattern().as_u32() / 2 + 1;
        let last_num = matches.iter().last().unwrap().pattern().as_u32() / 2 + 1;
        sum += (first_num * 10) + last_num;
    }

    Ok(sum)
}
