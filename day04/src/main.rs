fn create_puzzle_grid(puzzle: &str) -> Vec<Vec<char>> {
    puzzle.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn how_many_times(puzzle: &Vec<Vec<char>>, search_trigger: char, search_query: &str, contains: fn(&Vec<Vec<char>>, usize, usize, &str) -> i32) -> i32 {
    let mut num_times = 0;

    for row in 0..puzzle.len() {
        for col in 0..puzzle[row].len() {
            if puzzle[row][col] == search_trigger {
                num_times += contains(&puzzle, row, col, search_query);
            }
        }
    }

    num_times
}

fn find_word_in_all_directions(puzzle: &Vec<Vec<char>>, row: usize, col: usize, search_query: &str) -> i32 {
    let mut count = 0;
    let directions   = [
        (0, 1),     // right
        (1, 1),     // down right
        (1, 0),     // down
        (1, -1),    // down left
        (0, -1),    // left
        (-1, -1),   // up left
        (-1, 0),    // up
        (-1, 1),    // up right
    ];

    for (horiz_step, vert_step) in directions {
        if find_word_in_one_direction(puzzle, row, col, search_query, horiz_step, vert_step) {
            count += 1;
        }
    }

    count
}

fn find_word_in_x(puzzle: &Vec<Vec<char>>, row: usize, col: usize, search_query: &str)  -> i32 {
    let mut words_found = 0;
    let directions   = [
        (1, 1),     // down right
        (1, -1),    // down left
        (-1, -1),   // up left
        (-1, 1),    // up right
    ];   

    // make sure we have room for an x
    if row <= 0 || row >= puzzle.len()-1 || col <= 0 || col >= puzzle[row].len()-1 {
        0
    } else {
        for (horiz_step, vert_step) in directions {
            if find_word_in_one_direction(puzzle, row.checked_add_signed(-horiz_step).unwrap(), col.checked_add_signed(-vert_step).unwrap(), search_query, horiz_step, vert_step) {
                words_found += 1;
            }
        }

        if words_found == 2 {1} else {0}
    }
     
}

fn find_word_in_one_direction(puzzle: &Vec<Vec<char>>, row: usize, col: usize, word: &str, horiz_step: isize, vert_step: isize) -> bool {
    let mut row_idx = Some(row);
    let mut col_idx = Some(col);

    for letter in word.chars() {
        if row_idx.is_some_and(|r| r < puzzle.len()) && 
           col_idx.is_some_and(|c| c < puzzle[row_idx.unwrap()].len()) &&
           puzzle[row_idx.unwrap()][col_idx.unwrap()] == letter {

            row_idx = row_idx.unwrap().checked_add_signed(horiz_step);
            col_idx = col_idx.unwrap().checked_add_signed(vert_step);

        } else {
            return false;
        }
    }
   
    true
}

fn main() {
    let puzzle_raw = puzzle_files::load_puzzle_file();
    let puzzle =  create_puzzle_grid(&puzzle_raw);

    let word = "XMAS";
    let trigger = 'X';
    println!("# of times {} appears: {}", word, how_many_times(&puzzle, trigger, word, find_word_in_all_directions));

    let word = "MAS";
    let trigger = 'A';
    println!("# of times {} appears in an X: {}", word, how_many_times(&puzzle, trigger, word, find_word_in_x));
}
