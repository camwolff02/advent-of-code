use itertools::{izip, sorted};
use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_puzzle(puzzle: &str) -> (Vec<i32>, Vec<i32>) {

    let mut left = vec![];
    let mut right = vec![];

    for s in puzzle.lines() {
        // let a;
        let mut split = s.split_whitespace();

        if let Some(a) = split.next()   {
            if let Ok(l) = a.parse() {
                left.push(l);
            }
        }

        if let Some(b) = split.next()   {
            if let Ok(r) = b.parse() {
                right.push(r);
            }
        }
    }

    (left, right)
}

// add up each number on the left, after multiplying it by its frequency 
// in the right list
fn similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for &num in right {
        frequencies
            .entry(num)
            .and_modify(|frequency: &mut i32| {
                *frequency += 1
            })
            .or_insert(1);
    }

    left
        .iter()
        .map(|&num| {
            frequencies.entry(num).or_insert(0);
            num * frequencies[&num]
        })
        .sum()
}

fn l1_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let sorted_left: Vec<&i32> = sorted(left).collect();
    let sorted_right: Vec<&i32> = sorted(right).collect();

    izip!(&sorted_left, &sorted_right)
    .map(|(&l, &r)| {
        i32::abs(l - r)
    })
    .sum::<i32>()
}

fn main() {
    // Loading the input file
    let puzzle_folder = "puzzle/";
    let args: Vec<String> = env::args().collect();
    let filename = if !args.is_empty() {&args[1]} else {"test1.txt"};
    let path = format!("{}{}", puzzle_folder, filename);
    let puzzle = fs::read_to_string(&path)
        .expect(&format!("Failed to read puzzle file: \"{}\"", &path));

    // Solving the puzzle
    let (left, right) = parse_puzzle(&puzzle);
    println!("left: {:?}\nright: {:?}", left, right)
    println!("difference: {:?}", l1_distance(&left, &right));
    println!("similarity: {:?}", similarity(&left, &right));
}
