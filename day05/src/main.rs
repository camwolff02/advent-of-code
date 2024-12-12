use std::collections::{HashMap, HashSet};

// the rules are a hash map mapping a number to the values that CANNOT follow it
fn construct_rules(page_ordering_rules: &str) -> HashMap<i32, HashSet<i32>> {
    let mut rules = HashMap::new();

    for rule in page_ordering_rules.lines() {
        let (before_raw, after_raw) = rule.split_once('|').unwrap();
        let before = i32::from_str_radix(before_raw.trim(), 10).unwrap();
        let after = i32::from_str_radix(after_raw.trim(), 10).unwrap();

        if !rules.contains_key(&after) {
            // rules.insert(after, HashSet::from([after]))
            rules.insert(after, HashSet::new());
        }     

        rules.get_mut(&after).unwrap().insert(before);
    } 

    rules
}

fn follows_rules(items: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut banlist = HashSet::new();

    for item in items {
        if banlist.contains(&item) {
            return false
        }

        match rules.get(&item) {
            Some(banned_items) => {
                for banned_item in banned_items.iter() {
                    banlist.insert(banned_item);
                }
            },
            None => ()
        }
    }

    true
}

fn get_middle_elements(update_numbers: &str, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut middle_elements = Vec::new();

    for line in update_numbers.lines() {
        let items: Vec<i32> = line.split(',')
            .map(|str_item| i32::from_str_radix(str_item, 10))
            .flatten()
            .collect();

        if follows_rules(&items, rules) {
            middle_elements.push(items[items.len() / 2]);
        }
    }

    middle_elements
}


fn main() {
    let puzzle = puzzle_files::load_puzzle_file();

    let split_puzzle: Vec<&str> = puzzle.splitn(2, "\n\n").collect();
    if split_puzzle.len() != 2 {
        panic!("Puzzle file not formatted correctly!");
    }

    let page_ordering_rules = split_puzzle[0].trim();
    let update_numbers = split_puzzle[1].trim();

    // in order to construct our rules, we can use a graph representation
    let rules = construct_rules(page_ordering_rules);
    let middle_elements = get_middle_elements(update_numbers, &rules);

    println!("Solution: {}", middle_elements.iter().sum::<i32>());
}
