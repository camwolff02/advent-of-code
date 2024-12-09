use std::{env, fs};

use itertools::Itertools;

fn load_puzzle() -> String {
    let puzzle_folder = "puzzle/";
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {&args[1]} else {"test1.txt"};
    let path = format!("{}{}", puzzle_folder, filename);

    fs::read_to_string(&path)
        .expect("Failed to read puzzle file")
}

fn parse_puzzle(puzzle: &str) -> Vec<Vec<i32>> {
    puzzle.lines()
        .map(|line| line.split_whitespace()
            .filter_map(|next| next.parse().ok())
            .collect())
        .collect()
}

fn is_safe(report: &Vec<i32>, delta: u16) -> bool {
    _is_safe_with_dampener(report, delta, true)
}

fn is_safe_with_dampener(report: &Vec<i32>, delta: u16) -> bool {
    _is_safe_with_dampener(report, delta, false)
}

fn _is_safe_with_dampener(report: &Vec<i32>, delta: u16, dampened: bool) -> bool {
    if report.len() < 2 {true} 
    else {
        // dir is positive if report is ascending, and negative vice versa
        let mut ascending: Option<bool> = None; 

        for i in 0..report.len()-1 {
            let diff = report[i+1] - report[i]; 
            let _delta: i32 = delta.into();

            match ascending {
                Some(true) => 
                    if diff <= 0 || diff > _delta {
                        return _handle_dampener(report, delta, dampened, i)
                    },
                Some(false) => 
                    if diff >= 0 || -diff > _delta {
                        return _handle_dampener(report, delta, dampened, i)
                    },
                None => {
                    ascending = Some(diff > 0);

                    if diff == 0 || i32::abs(diff) > _delta {
                        return _handle_dampener(report, delta, dampened, i)
                    }
                }
            }
        }

        true
    }
}



// if we can be safe by removing either of the problem numers, we are safe
fn _handle_dampener(report: &Vec<i32>, delta: u16, dampened: bool, idx: usize) -> bool {
    if dampened {false}
    else {
        let min = if idx == 0 {0} else {idx-1};
        let max = idx+1;
        let mut sol = false;

        for i in min..max+1 {
            let mut _report = report.clone();  
            _report.remove(i);
            sol |= _is_safe_with_dampener(&_report, delta, true);
        }

        sol
    }
}

fn main() {
    let puzzle = load_puzzle();
    let reports = parse_puzzle(&puzzle);

    println!("reports w/o dampener:");
    print_reports_solution(&reports, is_safe);

    println!("\nreports w/ dampener:");
    print_reports_solution(&reports, is_safe_with_dampener);
}

fn print_reports_solution(reports: &Vec<Vec<i32>>, calc_safe: fn(&Vec<i32>, u16) -> bool) {
    let mut safe = 0;

    for report in reports {
        // print!("{:?}", report);
        if calc_safe(&report, 3) {
            safe += 1; 
            // println!(": safe");
        } else {
            // println!(": not safe");
        }
    }
    println!("solution: {}", safe);
}
