use std::{env, fs};

pub fn load_puzzle() -> String {
    let puzzle_folder = "puzzle/";
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {&args[1]} else {"test1.txt"};
    let path = format!("{}{}", puzzle_folder, filename);

    fs::read_to_string(&path)
        .expect("Failed to read puzzle file")
}
