use std::{env, fs};
use regex::Regex;

fn load_puzzle() -> String {
    let puzzle_folder = "puzzle/";
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {&args[1]} else {"test1.txt"};
    let path = format!("{}{}", puzzle_folder, filename);

    fs::read_to_string(&path)
        .expect("Failed to read puzzle file")
}

fn parse_just_mul(puzzle: &str) -> i32 {
    let re = Regex::new(r"(?m)mul\((?<left>[0-9]{0,3}),(?<right>[0-9]{0,3})\)").unwrap();

    re.captures_iter(&puzzle)
        .map(|caps| {
            let left = i32::from_str_radix(caps.name("left").unwrap().as_str(), 10)
                .unwrap_or_else(|_| 0);
            let right = i32::from_str_radix(caps.name("right").unwrap().as_str(), 10)
                .unwrap_or_else(|_| 0);

            left * right
        })
        .sum::<i32>()
}

fn parse_do_dont(puzzle: &str) -> i32 {
    let re = Regex::new(r"(?m)mul\((?<left>[0-9]{0,3}),(?<right>[0-9]{0,3})\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let mut active = true;

    re.captures_iter(&puzzle)
        .map(|caps| {
            let mut left = 0;
            let mut right = 0;

            if active && caps.name("left").is_some() && caps.name("right").is_some() {
                left = i32::from_str_radix(caps.name("left").unwrap().as_str(), 10)
                    .unwrap_or_else(|_| 0);
                right = i32::from_str_radix(caps.name("right").unwrap().as_str(), 10)
                    .unwrap_or_else(|_| 0);
            } else if caps.name("do").is_some() {
                active = true;
            } else if caps.name("dont").is_some() {
                active = false;
            }

            left * right
        })
        .sum::<i32>()
}
fn main() {
    let puzzle = load_puzzle();

    println!("Solution with just mul(): {}", parse_just_mul(&puzzle));
    println!("Solution with do() and don't(): {}", parse_do_dont(&puzzle));
}
