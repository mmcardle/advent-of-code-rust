use core::panic;
use std::fs;
use std::env;
use std::vec;

use aoc::lattice::Lattice;
use aoc::lattice::{E, N, S, W};


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

    contents
}

fn parse_input(input: &str) -> (Vec<Lattice>, Vec<Vec<char>>, Vec<char>) {

  let mut lattice: Vec<Lattice> = Vec::new();
  let mut commands: Vec<char> = Vec::new();

  let mut grid: Vec<Vec<char>> = Vec::new();
  for (i, line) in input.lines().enumerate() {
    let mut row = Vec::new();
    if line.starts_with("#") {
      for (j, c) in line.chars().into_iter().enumerate() {
        if c == '#' {
          row.push(c);
          row.push(c);
        } else if c == '@' {
          row.push(c);
          row.push('.');
        } else if c == 'O' {
          row.push('[');
          row.push(']');
        }else {
          row.push('.');
          row.push('.');
        }
        lattice.push(Lattice::new(i as i64, j as i64));
      }
      grid.push(row);
    } else {
      for c in line.chars() {
        commands.push(c);
      }
    }
  }

  (lattice, grid, commands)

    
}

fn print_grid(data: &Vec<Vec<char>>) {
    print!("col   ");
    for (j, _) in data[0].iter().enumerate() {
        print!("  {}  ", format_str_as_green(&j.to_string()));
    }
    println!();
    for (i, d) in data.iter().enumerate() {
        print!("row  {}", format_str_as_green(&i.to_string()));
        for c in d.iter() {
          if c == &'@' {
            print!("  {}  ", format_str_as_red(&c.to_string()));
          } else {
            print!("  {}  ", c);
          }
        }
        println!();
      }
}

fn find_char_in_grid(grid: &Vec<Vec<char>>, c: char) -> Lattice {
  for (i, row) in grid.iter().enumerate() {
    for (j, col) in row.iter().enumerate() {
      if *col == c {
        return Lattice::new(j as i64, i as i64);
      }
    }
  }
  Lattice::new(-1, -1)
}

fn process_whats_in_the_way(robot: &mut Lattice, grid: &mut Vec<Vec<char>>, c: char, direction: Lattice) {
  if c == '.' {
    println!("Nothing in the way");
    grid[usize::try_from(robot.y()).unwrap()][usize::try_from(robot.x()).unwrap()] = '.';
    *robot = *robot + direction;
    grid[usize::try_from(robot.y()).unwrap()][usize::try_from(robot.x()).unwrap()] = '@';
  } else if c == '#' {
    println!("Wall in the way");
  } else if c == 'O' {
    println!("Box in the way");
    let mut step = robot.clone() + direction;
    
    while grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] == 'O' {
      step = step + direction;
    }

    //println!("Step START {:?}", step);

    let is_space  = grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] == '.';
    
    if is_space {

      while grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] != 'O' {
        grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] = 'O';
        step = step - direction;
      }
      
      //println!("Step END {:?}", step);
      grid[usize::try_from(robot.y()).unwrap()][usize::try_from(robot.x()).unwrap()] = '.';
      grid[usize::try_from(robot.y()).unwrap()][usize::try_from(robot.x()).unwrap()] = '.';
      grid[usize::try_from(robot.y() + direction.y()).unwrap()][usize::try_from(robot.x() + direction.x()).unwrap()] = '@';
      *robot = *robot + direction;
      
      //println!("Robot at the END {:?}", robot);
    } else {
      //println!("No space to move");
    }
  } else if c == ']' || c == '[' {

    let mut step = robot.clone() + direction;

    
    while grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] != '.' {
      step = step + direction;
    }

    println!("Step START {:?}", step);
    
    let is_space  = grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] == '.';
    
    if is_space {
      println!("SPACVE TO PUSH START {:?}", step);

      while grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] != '@' {
        let current = grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()];
        let filler = if direction == W { '[' } else { ']' };
        if current == '.' {
          grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] = filler;
        }
        if current == ']' {
          grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] = '[';
        }
        if  current == '[' {
          grid[usize::try_from(step.y()).unwrap()][usize::try_from(step.x()).unwrap()] = ']';
        }
        step = step - direction;
        println!("Step {:?}", step);
      }

      grid[usize::try_from(robot.y()).unwrap()][usize::try_from(robot.x()).unwrap()] = '.';
      grid[usize::try_from(robot.y() + direction.y()).unwrap()][usize::try_from(robot.x() + direction.x()).unwrap()] = '@';
      *robot = *robot + direction;
    }

  }

}

fn process_command_for_robot(robot: &mut Lattice, grid: &mut Vec<Vec<char>>, command: char) {
  println!("\nRobot {:?} go {}", robot, command);
  let north = robot.north();
  let south = robot.south();
  let east = robot.east();
  let west = robot.west();

  match command {
    '^' => {
      let y = usize::try_from(north.y()).unwrap();
      let x = usize::try_from(north.x()).unwrap();
      let what_up_in_grid = grid[y][x];
      println!("Whats north {:?}", what_up_in_grid);
      process_whats_in_the_way(robot, grid, what_up_in_grid, N);
    },
    'v' => {
      let y = usize::try_from(south.y()).unwrap();
      let x = usize::try_from(south.x()).unwrap();
      let what_down_in_grid = grid[y][x];
      println!("Whats south {:?}", what_down_in_grid);
      process_whats_in_the_way(robot, grid, what_down_in_grid, S);
    },
    '<' => {
      let y = usize::try_from(west.y()).unwrap();
      let x = usize::try_from(west.x()).unwrap();
      println!("X Y {:?} {:?}", x, y);
      let what_left_in_grid = grid[y][x];
      println!("Whats west {:?}", what_left_in_grid);
      process_whats_in_the_way(robot, grid, what_left_in_grid, W);
    },
    '>' => {
      let y = usize::try_from(east.y()).unwrap();
      let x = usize::try_from(east.x()).unwrap();
      let what_right_in_grid = grid[y][x];
      println!("Whats east {:?}", what_right_in_grid);
      process_whats_in_the_way(robot, grid, what_right_in_grid, E);
    },
    _ => {
      println!("Unknown command: {}", command);
    }
  }
}

pub fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let (lattice, mut grid, commands) = parse_input(content.as_str());    

    println!("{:?}", lattice);
    println!("Commands {:?}", commands);

    let mut robot = find_char_in_grid(&grid, '@');

    println!("{:?}", robot);

    print_grid(&grid);

    //process_command_for_robot(&mut robot, &mut grid, '^');
    
    print_grid(&grid);
    for c in commands {
      process_command_for_robot(&mut robot, &mut grid, c);
      print_grid(&grid);
    }

    println!("FINAL");
    print_grid(&grid);

    // The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map plus its distance 
    // from the left edge of the map. (This process does not stop at wall tiles; measure all the way to the edges of the map.)
    // So, the box shown below has a distance of 1 from the to

    let gps_box = grid.iter().enumerate().fold(0, |acc, (i, row)| {
      acc + row.iter().enumerate().fold(0, |acc2, (j, col)| {
        if *col == 'O' {
          acc2 + (i * 100 + j)
        } else {
          acc2
        }
      })
    });

    println!("GPS Box: {}", gps_box);


}