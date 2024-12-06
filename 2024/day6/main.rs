use core::panic;
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

    //println!("Found text:\n{}", contents);
    contents
}

fn format_str_as_red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

fn format_str_as_green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

const DIRECTION_UP: &str = "^";
const DIRECTION_DOWN: &str = "v";
const DIRECTION_LEFT: &str = "<";
const DIRECTION_RIGHT: &str = ">";
const OBSTACLE: &str = "0";
const EX: &str = "X";
const BLOCK: &str = "#";

fn print_mapping(mapping: &Vec<Vec<&str>>) {
    for row in mapping {
        for &item in row {
            // if # print in red
            if item == BLOCK {
                print!("{} ", format_str_as_red(item));
            } else if item == DIRECTION_UP || item == DIRECTION_DOWN || item == DIRECTION_LEFT || item == DIRECTION_RIGHT {
                print!("{} ", format_str_as_green(item));
            } else {
                print!("{} ", item);
            }
        }
        println!();
    }
}

fn count_x_in_mapping(mapping: &Vec<Vec<&str>>) -> usize {
    let mut counter = 0;
    for row in mapping {
        for &item in row {
            if item == EX {
                counter += 1;
            }
        }
    }
    counter
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    // parse 2 integers on each line

    let mut mapping = Vec::new();
    let lines= input.split("\n");
    for line in lines {
        let items: Vec<&str> = line.split("").collect();
        mapping.push(items);
    }
    mapping
}

fn find_direction_in_matrix(matrix: &Vec<Vec<&str>>) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            if item == DIRECTION_UP || item == DIRECTION_DOWN || item == DIRECTION_LEFT || item == DIRECTION_RIGHT {
                return (i, j);
            }
        }
    }
    panic!("No direction found in matrix");
}

fn turn_right(direction: &str) -> &str {
    match direction {
        DIRECTION_UP => DIRECTION_RIGHT,
        DIRECTION_DOWN => DIRECTION_LEFT,
        DIRECTION_LEFT => DIRECTION_UP,
        DIRECTION_RIGHT => DIRECTION_DOWN,
        _ => panic!("Unknown direction: {}", direction)
    }
}

#[derive(PartialEq)]
enum MAPResult {
    InfiniteLoop,
    OutOfBounds,
    IllegalObstacle
}

fn process_next_step(step_count: usize, direction: &str, x: usize, y: usize, mut mapping: Vec<Vec<&str>>) -> MAPResult {
    // check if out of bounds
    //print_mapping(&mapping);
    if step_count > 10000 {
        println!("Step count: {}", step_count);
        //print_mapping(&mapping);
        return MAPResult::InfiniteLoop;
    }
    let (mut new_x, mut new_y) = (x, y);
    //println!("Processing step at ({}, {}) in direction {}", x, y, direction);
    let mut current_direction = "ERROR";
    match direction {
        DIRECTION_UP => {
            //println!("Going up");
            new_x -= 1;
            current_direction = DIRECTION_UP;
        },
        DIRECTION_DOWN => {
            //println!("Going down");
            new_x += 1;
            current_direction = DIRECTION_DOWN;
        },
        DIRECTION_LEFT => {
            //println!("Going left");
            new_y -= 1;
            current_direction = DIRECTION_LEFT;
        },
        DIRECTION_RIGHT => {
            //println!("Going right");
            new_y += 1;
            current_direction = DIRECTION_RIGHT;
        },
        _ => {
            panic!("Unknown direction: {}", direction);
        }
    }
    if new_x >= mapping.len() || new_y >= mapping[0].len() {
        println!("Out of bounds");
        mapping[x][y] = EX;
        let counter = count_x_in_mapping(&mapping);
        //print_mapping(&mapping);
        println!("Total X's: {}", counter);
        return MAPResult::OutOfBounds;
    }
    let trying_to_step_into = mapping[new_x][new_y];

    if trying_to_step_into == "." || trying_to_step_into == EX {
        //println!("Found empty space");
        mapping[new_x][new_y] = current_direction;
        mapping[x][y] = EX;
        return process_next_step(step_count+1, current_direction, new_x, new_y, mapping);
    } else if trying_to_step_into == BLOCK || trying_to_step_into == OBSTACLE {
        let new_direction = turn_right(&current_direction);
        mapping[x][y] = new_direction;
        //println!("Turning to {}", new_direction);
        return process_next_step(step_count+1, new_direction, x, y, mapping);
    } else {
        //print_mapping(&mapping);
        //panic!("CANT STEP INTO: {}", trying_to_step_into);
        return MAPResult::OutOfBounds;
    }
    //print_mapping(mapping);
    //process_next_step(direction, new_x, new_y, mapping);
}


fn modify_mapping_at_position(x: usize, y: usize, original_mapping: &Vec<Vec<&str>>) -> MAPResult {

    //println!("MODIFY  ({}, {})", x, y);
    let mut new_mapping = original_mapping.clone();

    if x >= new_mapping.len() || y >= new_mapping[0].len() {
        return MAPResult::IllegalObstacle;
    }
    let current_value = new_mapping[x][y];
    
    if current_value == BLOCK || current_value == OBSTACLE || current_value == DIRECTION_UP 
      || current_value == DIRECTION_DOWN || current_value == DIRECTION_LEFT || current_value == DIRECTION_RIGHT {
        return MAPResult::IllegalObstacle;
    }

    new_mapping[x][y] = OBSTACLE;

    //println!("MAPPING WITH OBS AT  ({}, {})", x, y);
    //print_mapping(&new_mapping);

    let (x, y) = find_direction_in_matrix(&new_mapping);
    let direction = new_mapping[x][y];
    return process_next_step(0, direction, x, y, new_mapping);
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());
    let mapping = parse_input(content.as_str());    

    //print_mapping(&mapping);

    //let (x, y) = find_direction_in_matrix(&mapping);
    //let direction = mapping[x][y];
    // Part 1
    //println!("Found direction {} at position ({}, {})", direction, x, y);
    //let step_count = 0;
    //process_next_step(step_count, direction, x, y, mapping);

    let length = mapping.len();
    let width = mapping[0].len();

    let mut map_results = Vec::new();

    for i in 0..length  {
        for j in 0..width {
            let result = modify_mapping_at_position(i, j, &mapping);
            map_results.push(result);
        }
    }

    /*map_results.iter().for_each(|result| {
        match result {
            MAPResult::InfiniteLoop => println!("Infinite loop"),
            MAPResult::OutOfBounds => println!("Out of bounds"),
            MAPResult::IllegalObstacle => println!("Illegal obstacle"),
        }
    });*/

    let number_of_infinite_loops = map_results.iter().filter(|&x| *x == MAPResult::InfiniteLoop).count();

    println!("Number of infinite loops: {}", number_of_infinite_loops);


}