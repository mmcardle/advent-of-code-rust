use core::hash;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;


fn get_filename_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a filename as an argument");
    }
    args[1].clone()
}


fn read_file(filename: &str) -> String {

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    contents
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
    // parse 2 integers on each line

    let mut parsed_input = Vec::new();

    let mut start_positions = Vec::new();
    
    for (i, line) in input.lines().enumerate() {
        let mut parsed_line = Vec::new();
        for (j, number) in line.chars().enumerate() {
            let digit = number.to_string().parse::<usize>().unwrap();
            parsed_line.push(digit);
            if digit == 0 {
                start_positions.push((i, j));
            }
        }
        parsed_input.push(parsed_line);
    }

    (parsed_input, start_positions)
}


fn get_possible_moves(grid: &Vec<Vec<usize>>, position: (usize, usize), trail: &Vec<(usize, usize)> ) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    let i = position.0;
    let j = position.1;

    if i > 0 && !trail.contains(&(i - 1, j)) {
        moves.push((i - 1, j));
    }

    if i < grid.len() - 1 && !trail.contains(&(i + 1, j)) {
        moves.push((i + 1, j));
    }

    if j > 0 && !trail.contains(&(i, j -1)) {
        moves.push((i, j - 1));
    }

    if j < grid[0].len() - 1 && !trail.contains(&(i, j + 1)) {
        moves.push((i, j + 1));
    }

    moves
}

fn format_str_as_red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

fn format_str_as_green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

fn print_path_through_grid(grid: &Vec<Vec<usize>>, path: Vec<(usize, usize)>) {
    for (i, line) in grid.iter().enumerate() {
        for (j, number) in line.iter().enumerate() {
            if path.contains(&(i, j)) {
                print!("{}", format_str_as_red(number.to_string().as_str()));
            } else {
                print!("{}", number);
            }
        }
        println!("");
    }
}

fn find_trail(grid: &Vec<Vec<usize>>, start_position: (usize, usize), mut trail: Vec<(usize, usize)>, all_trails: &mut Vec<Vec<(usize, usize)>>) {
    let mut current_position = start_position;

    trail.push(current_position);

    let current_position_value = grid[current_position.0][current_position.1];

    if current_position_value == 9 {
        println!("Found trail {:?}", trail);
        all_trails.push(trail.clone());
        //print_path_through_grid(grid, trail.clone());
        return;
    }

    let possible_moves = get_possible_moves(grid, current_position, &trail);
    //println!("{:?} => {:?}", current_position, possible_moves);

    for position in possible_moves {
        let position_value = grid[position.0][position.1];
         if position_value == current_position_value + 1 {
            //println!("Found next position {:?} : {}", position, position_value);
            current_position = position;
            find_trail(grid, current_position, trail.clone(), all_trails);
        }
    }

}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let (grid, start_positions) = parse_input(content.as_str());    

    println!("   0  1  2  3  4  5  6  7");
    for (i, line) in grid.iter().enumerate() {
        println!("{} {:?}", i, line);
    }

    let mut all_trails = Vec::new();

    for position in start_positions {
        println!("\n\n{:?}", position);
        find_trail(&grid, position, Vec::new(), &mut all_trails);
    }

    

    let mut part1_trails: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut part2_trails: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for trail in all_trails.iter() {
        part1_trails.insert(trail[0], HashSet::new());
    }

    for trail in all_trails.iter() {
        let start_position = trail[0];
        let last_position = trail[trail.len() - 1];
        // check if last position is already in hashmap
        let mut set = part1_trails.get_mut(&start_position).unwrap();
        set.insert(last_position);

        part2_trails.insert(start_position, trail.clone());
    }

    let total = part1_trails.iter().fold(0, |acc, (key, hashset)| {
        acc + hashset.len()
    });
    for (key, hashset) in part1_trails.iter() {
        println!("{:?} => {:?} => {}", key, hashset, hashset.len());
    }

    let total_part2 = 0;

    for (key, trail) in part2_trails.iter() {
        println!("Part 2 {:?} => {:?}", key, trail);
    }
    
    println!("Total paths: {}", total);
    println!("Total paths part 2: {}", total_part2);
    println!("Found all trails {:?}", all_trails.len());
    /*
    println!("   0  1  2  3  4  5  6  7");
    for (i, line) in grid.iter().enumerate() {
        println!("{} {:?}", i, line);
    }*/

}