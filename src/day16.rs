use std::fs;
use std::env;

use aoc::lattice::{Lattice, N, S, E, W};


fn format_str_as_red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
  }
  
  fn format_str_as_green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
  }
  
  fn format_str_as_yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
  }

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

fn print_grid(grid: &Vec<Vec<char>>,) {
    print!("col    ");
    for (col, _row) in grid[0].iter().enumerate() {
        print!("{:3}", col);
    }
    println!();
    for (i, row) in grid.iter().enumerate() {
        print!("row {:3
        } ", i);
        for (_j, c) in row.iter().enumerate() {
            if c == &'S' {
                print!("{}", format_str_as_red("S"));
            } else if c == &'E' {
                print!("{}", format_str_as_yellow("E"));
            } else if c == &'X' {
                print!("{}", format_str_as_green("X"));
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
  }

fn parse_input(input: &str) {
    // parse 2 integers on each line

    let mut start = Lattice::new(0, 0);
    let mut end = Lattice::new(0, 0);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        println!("{}", line);
        grid.push(Vec::new());
        line.chars().enumerate().for_each(|(col, c)| {
            grid[row].push(c);
            match c {
                '#' => {},
                ',' => {},
                'S' => {
                    start = Lattice::new(col as i64, row as i64);
                },
                'E' => {
                    end = Lattice::new(col as i64, row as i64);
                },
                _ => {}
            }
        });
    }

    println!("Start: {:?}", start);
    println!("End: {:?}", end);
    //println!("Grid: {:?}", grid);

    print_grid(&grid);

    traverse_grid(&grid, start, end);

    
}

fn cost_of_grid_and_turns(depth: usize, turns: usize) -> usize {
    depth + turns * 1000
}

fn traverse_grid(grid: &Vec<Vec<char>>, start: Lattice, end: Lattice) {

    fn dfs(
        grid: &mut Vec<Vec<char>>,
        start: Lattice,
        end: Lattice,
        depth: usize,
        turns: usize,
        current_direction: Lattice,
        valid_grids: &mut Vec<(Vec<Vec<char>>, usize, usize)>
    ) {

        if depth == 100000 {
            return
        }

        start.across(false).iter().rev().for_each(|next_step| {
            
            let char_at_next_step = grid[next_step.y() as usize][next_step.x() as usize];
            //println!("   Next step to {:?} = {}", next_step, char_at_next_step);
            
            if char_at_next_step == '.' {
                
                let new_direction = start.direction(&next_step);

                /*if new_direction == N {
                    println!("   Heading North");
                } else if new_direction == S {
                    println!("   Heading South");
                } else if new_direction == E {
                    println!("   Heading East");
                } else if new_direction == W {
                    println!("   Heading West");
                }*/

                let same_direction = new_direction == current_direction;
                //println!("   Next step to {:?} = {:?}", next_step, new_direction);
                //println!("   Same dicrection current={:?} == new={:?} = {:?} ", current_direction, new_direction, new_direction == current_direction);
                if same_direction {
                    //*turns += 1;
                    //println!("HEADING SAME DIRECTION from {:?} to {:?}", current_direction, new_direction);
                } else {
                    //println!("TURNING IT UP {:?} from {:?} to {:?}", start, current_direction, new_direction);
                }

                let mut cloned_grid = grid.clone();
                cloned_grid[start.y() as usize][start.x() as usize] = 'X';
                cloned_grid[next_step.y() as usize][next_step.x() as usize] = 'S';
                //print_grid(&cloned_grid);
                
                let current_cost_of_grid_and_turns = cost_of_grid_and_turns(depth, turns);

                for valid_grid in valid_grids.iter() {
                    let cost_of_grid_and_turns = cost_of_grid_and_turns(valid_grid.1, valid_grid.2);
                    if cost_of_grid_and_turns < current_cost_of_grid_and_turns {
                        // Short circuit
                        return
                    }
                }
                if same_direction {
                    dfs(&mut cloned_grid, *next_step, end, depth + 1, turns , new_direction, valid_grids);
                } else {
                    dfs(&mut cloned_grid, *next_step, end, depth + 1, turns + 1, new_direction, valid_grids);
                }
            }
            
            if char_at_next_step == 'E' {
                //print_grid(&grid);
                let mut cloned_grid = grid.clone();
                cloned_grid[start.y() as usize][start.x() as usize] = 'X';
                cloned_grid[next_step.y() as usize][next_step.x() as usize] = 'S';
                println!("Grid found at depth {} turns={}", depth, turns);
                print_grid(&cloned_grid);
                valid_grids.push((cloned_grid, depth + 1, turns));
            }
        });

    }

    let mut valid_grids: Vec<(Vec<Vec<char>>, usize, usize)> = vec![];
    let current_direction = E;

    dfs(&mut grid.clone(), start, end, 0, 0, current_direction, &mut valid_grids);

    let mut totals: Vec<usize> = vec![];

    for grid_result in valid_grids {
        let grid = grid_result.0;
        let depth = grid_result.1;
        let turns = grid_result.2;
        totals.push(cost_of_grid_and_turns(depth, turns));
        print_grid(&grid);
    }

    println!("Totals: {:?}", totals);

    let min_total = totals.iter().min().unwrap();

    println!("Min total: {}", min_total);

}

pub fn main() {

    // TOO high Grid found at depth 767 turns=242 === 242767

    let content = read_file(get_filename_from_args().as_str());

    let input = parse_input(content.as_str());    

    println!("{:?}", input);

}