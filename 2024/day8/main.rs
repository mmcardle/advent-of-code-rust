use std::collections::HashSet;
use std::fs;
use std::env;
use std::collections::HashMap;
use itertools::Itertools; // 0.8.2

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

    println!("Found text:\n{}", contents);
    contents
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    // parse 2 integers on each line

    let mut result = Vec::new();
    for line in input.lines() {
        let mut line_vec = Vec::new();
        for word in line.split("") {
            if word != "" {
                line_vec.push(word);
            }
        }
        result.push(line_vec);
    }
    result
}

const DIRECTION_UP: &str = "^";
const DIRECTION_DOWN: &str = "v";
const DIRECTION_LEFT: &str = "<";
const DIRECTION_RIGHT: &str = ">";
const OBSTACLE: &str = "0";
const EX: &str = "X";
const BLOCK: &str = "#";

fn format_str_as_red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

fn format_str_as_green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

fn format_str_as_yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
}

fn print_mapping(mapping: &Vec<Vec<&str>>) {
    // print header
    println!("Printing Mapping:");
    print!("  ");
    for i in 0..mapping.len() {
        print!("{: >3}", i);
    }
    println!();
    let mut counter = 0;
    for row in mapping {
        print!("{: >3}", counter);
        for &item in row {
            // if # print in red
            if item == BLOCK {
                print!(" {} ", format_str_as_red(item));
            } else if item == "." {
                print!(" {} ", item);
            } else if item == EX {
                print!(" {} ", format_str_as_yellow(item));
            } else {
                print!(" {} ", format_str_as_green(item));
            }
        }
        counter += 1;
        println!();
    }
}

fn find_all_digits_and_numbers<'a>(mapping: &'a Vec<Vec<&'a str>>) -> HashMap<&'a str, Vec<(isize, isize)>> {
    let mut result: HashMap<&str, Vec<(isize, isize)>> = HashMap::new();
    for (y, row) in mapping.iter().enumerate() {
        for (x, &item) in row.iter().enumerate() {
            if item != BLOCK && item != "." && item != "" {
                if result.contains_key(item) {
                    let current = result.get_mut(item).unwrap();
                    current.push(((x as isize), y as isize));
                } else {
                    result.insert(item, vec![((x as isize), y as isize)]);
                }
            }
        }
    }
    result
}

fn step_between(p1: (isize, isize), p2: (isize, isize)) -> (isize, isize) {
    let gap_x = p1.0 - p2.0;
    let gap_y = p1.1 - p2.1;
    (gap_x, gap_y)
}

fn add_gap(p1: (isize, isize), gap: (isize, isize)) -> (isize, isize) {
    (p1.0 + gap.0, p1.1 + gap.1)
}

fn in_bounds(p: (isize, isize), width: usize, height: usize) -> bool {
    p.0 >= 0 && p.0 < width as isize && p.1 >= 0 && p.1 < height as isize
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let mapping = parse_input(content.as_str());    

    print_mapping(&mapping);

    let mut cloned_mapping = mapping.clone();

    let height = mapping.len();
    let width = mapping[0].len();

    println!("Cloned Mapping: {:?} x {:>}", height, width);

    let digits = find_all_digits_and_numbers(&cloned_mapping);

    println!("Digits: {:?}", digits);

    let mut locations: HashSet<(isize, isize)> = HashSet::new();

    let mut num_posutions = 0;

    digits.iter().for_each(|(digit, positions)| {
        println!("Digit: {} :: {:?}", digit, positions);
        num_posutions += positions.len();
        let combinations = positions.iter().combinations(2);
        for combination in combinations {
            let p1 = combination[0];
            let p2 = combination[1];
            let distance = step_between(*p1, *p2);
            println!("   -- Step between {:?} and {:?} is {:?}", p1, p2, distance);
            
            let mut gap_up1 = add_gap(*p1, distance);
            let mut gap_down2 = add_gap(*p2, (-distance.0, -distance.1));
            
            println!("         -- Gap up: {:?} and Gap down: {:?}", gap_up1, gap_down2);
            
            if ( in_bounds(gap_up1, width, height) && gap_up1.0 >= 0 ) && (gap_up1.1 >= 0 && gap_up1 != *p1 && gap_up1 != *p2) {
                println!("         -- Adding Gap up 1: {:?}", gap_up1);
                locations.insert(gap_up1);
            }
            
            if ( in_bounds(gap_down2, width, height) && gap_down2.0 >= 0) && (gap_down2.1 >= 0 && gap_down2 != *p1 && gap_down2 != *p2) {
                println!("         -- Adding Gap down 2: {:?}", gap_down2);
                locations.insert(gap_down2);
            }

            while in_bounds(gap_up1, width, height) {
                gap_up1 = add_gap(gap_up1, distance);
                if (gap_up1.0 >= 0 && gap_up1.0 < width as isize) && (gap_up1.1 >= 0 && gap_up1.1 < height as isize && gap_up1 != *p1 && gap_up1 != *p2) {
                    println!("         -- Adding Gap up 1: {:?}", gap_up1);
                    locations.insert(gap_up1);
                }
            }

            while in_bounds(gap_down2, width, height) {
                gap_down2 = add_gap(gap_down2, (-distance.0, -distance.1));
                if (gap_down2.0 >= 0 && gap_down2.0 < width as isize) && (gap_down2.1 >= 0 && gap_down2.1 < height as isize && gap_down2 != *p1 && gap_down2 != *p2) {
                    println!("         -- Adding Gap down 2: {:?}", gap_down2);
                    locations.insert(gap_down2);
                }
            }
            
            /*
            println!("         -- Gap up: {:?} and Gap down: {:?} Gap up 2 {:?} Gap down 2 {:?}", gap_up1, gap_down1, gap_up2, gap_down2);
            let gap_down1 = add_gap(*p1, (-distance.0, -distance.1));
            let gap_up2 = add_gap(*p2, distance);
            if (gap_down1.0 >= 0 && gap_down1.0 < width as isize) && (gap_down1.1 >= 0 && gap_down1.1 < height as isize && gap_down1 != *p1 && gap_down1 != *p2) {
                println!("         -- Adding Gap down 1: {:?}", gap_down1);
                //locations.insert(gap_down1);
            }
            if (gap_up2.0 >= 0 && gap_up2.0 < width as isize) && (gap_up2.1 >= 0 && gap_up2.1 < height as isize && gap_up2 != *p1 && gap_up2 != *p2) {
                println!("         -- Adding Gap up 2: {:?}", gap_up2);
                //locations.insert(gap_up2);
            }*/
        }
    });

    println!("Locations: {:?} {} + {} = {}", locations, locations.len(), num_posutions, locations.len() + num_posutions);

    

    locations.iter().for_each(|location| {
        cloned_mapping[location.1 as usize][location.0 as usize] = EX;
    });
    
    print_mapping(&mapping);
    print_mapping(&cloned_mapping);

    let count_of_all_non_dots = cloned_mapping.iter().map(|row| row.iter().filter(|&item| *item != ".").count()).sum::<usize>();

    println!("Count of all non dots: {}", count_of_all_non_dots);

}