use std::fs;
use std::env;
use std::str::FromStr;
use aoc::lattice::Lattice;

use crate::lattice;


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

#[derive(Debug, Clone, Copy)]
struct Robot {
  p: Lattice,
  v: Lattice,
}

impl Robot {
  pub fn new(p: Lattice, v: Lattice) -> Self {
    Self { p, v }
  }

  pub fn move_robot(&mut self, usize: usize, width: i64, height: i64) {
    //println!("Moving robot by {} at velocity {:?}", usize, self.v);
    //println!("Before: {:?}", self.p);
    for _ in 0..usize {
      self.p = self.p + self.v;
    }

    let x = self.p.x();
    let y = self.p.y();

    if x >= width {
      self.p.set_x(x.rem_euclid(width));
    }

    if y >= height {
      self.p.setY(y.rem_euclid(height));
    }

    if x < 0 {
      self.p.set_x(x.rem_euclid(width));
    }
    if y < 0 {
      self.p.setY(y.rem_euclid(height));
    }

    //println!("After: {:?}", self.p);
  }
}

fn count_robots_at_position(robots: &Vec<Robot>, x: i64, y: i64) -> usize {
  robots.iter().filter(|robot| robot.p.x() == x && robot.p.y() == y).count()
}

fn print_grid_with_robots(robots: &Vec<Robot>, width: i64, height: i64) {
  let grid = make_grid(width, height, robots);

  println!();
  print!("col  ");
  for (r, _) in grid[0].iter().enumerate() {
    print!(" {}", format_str_as_yellow(&r.to_string()));
  }
  println!("");
  for (r, row) in grid.iter().enumerate() {
    print!("row {}", format_str_as_green(&r.to_string()));
    for cell in row {
      print!(" {}", cell);
    }
    println!();
  }
  println!();

}

fn make_grid_no_count(width: i64, height: i64, robots: &Vec<Robot>) -> Vec<Vec<char>> {
  let mut grid = vec![vec!['.'; width as usize]; height as usize];

  for robot in robots.iter() {
      let count = count_robots_at_position(&robots, robot.p.x(), robot.p.y());
      //println!("Robot at {:?} has {} robots, debug= {} {} ", robot.p, count, robot.p.y() as usize, robot.p.x() as usize);
      let y= usize::try_from(robot.p.y()).unwrap();
      let x= usize::try_from(robot.p.x()).unwrap();
      grid[y][x] = if count > 0 { '#' } else { '.' };
    }
  grid
}

fn make_grid(width: i64, height: i64, robots: &Vec<Robot>) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for robot in robots.iter() {
        let count = count_robots_at_position(&robots, robot.p.x(), robot.p.y());
        //println!("Robot at {:?} has {} robots, debug= {} {} ", robot.p, count, robot.p.y() as usize, robot.p.x() as usize);
        let y= usize::try_from(robot.p.y()).unwrap();
        let x= usize::try_from(robot.p.x()).unwrap();
        grid[y][x] = count.to_string().chars().next().unwrap();
      }
    grid
}


fn parse_input(input: &str) -> Vec<Robot>{

  let raw_parts = input.lines().map(|line| {
    let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    parts
  }).filter(|x| x.len() > 0).map(|parts| {
    parts
  });

  let mut robots = Vec::new();

  for part in raw_parts {
    let position = part[0].strip_prefix("p=").unwrap();
    let velocity = part[1].strip_prefix("v=").unwrap();
    let pos = position.split(",")
        .map(i64::from_str)
        .collect::<Result<Vec<i64>, _>>()
        .expect("Failed to parse position");

    let vels = velocity.split(",")
        .map(i64::from_str)
        .collect::<Result<Vec<i64>, _>>()
        .expect("Failed to parse velocity");
    println!("{:?} {:?}", pos, vels);

    let robot = Robot::new(
      Lattice::new(pos[0], pos[1]),
      Lattice::new(vels[0], vels[1])
    );
    robots.push(robot);

  }

  println!("{:?}", robots);

  robots

}

fn check_middle_16_square_all_have_robots(grid: &Vec<Vec<char>>) -> bool {
  let middle_x = grid[0].len() / 2;
  let middle_y = grid.len() / 2;

  let middle_squares = vec![
    grid[middle_y - 1][middle_x - 1],
    grid[middle_y - 1][middle_x],
    grid[middle_y - 1][middle_x + 1],
    grid[middle_y - 1][middle_x + 2],
    grid[middle_y][middle_x - 1],
    grid[middle_y][middle_x],
    grid[middle_y][middle_x + 1],
    grid[middle_y][middle_x + 2],
    grid[middle_y + 1][middle_x - 1],
    grid[middle_y + 1][middle_x],
    grid[middle_y + 1][middle_x + 1],
    grid[middle_y + 1][middle_x + 2],
    grid[middle_y + 2][middle_x - 1],
    grid[middle_y + 2][middle_x],
    grid[middle_y + 2][middle_x + 1],
    grid[middle_y + 2][middle_x + 2],
  ];

  let found = middle_squares.iter().all(|&c| c == '#');

  found
}

pub fn main() {

    // 8257 WINNER

    let content = read_file(get_filename_from_args().as_str());

    let mut robots = parse_input(content.as_str());   

    let width: i64 = 101;
    let height: i64 = 103;
    let initial_iterations = 0;
    let iterations: i32 = 75000;

    print_grid_with_robots(&robots, width, height);

    for i in initial_iterations..(iterations + initial_iterations) {
      for robot in robots.iter_mut() {
        robot.move_robot(1, width, height);
      }
      //println!("After {} second(s)", i + 1);
      //print_grid_with_robots(&robots, width, height);

      let grid = make_grid_no_count(width, height, &robots);
      //let found = check_any_subgrid_of_size_has_robots_at_subgrid_of_size_X(&grid, 3);
      let found = check_middle_16_square_all_have_robots(&grid);

      if found {
        print_grid_with_robots(&robots, width, height);
        println!("Found symmetrical grid after {} iterations", i + 1);
        break;
      }
    }

    let robots_in_top_left = robots.iter().filter(
      |robot| robot.p.x() < width / 2 && robot.p.y() < height / 2
    ).count();
    let robots_in_top_right = robots.iter().filter(
      |robot| robot.p.x() > width / 2 && robot.p.y() < height / 2
    ).count();
    let robots_in_bottom_left = robots.iter().filter(
      |robot| robot.p.x() < width / 2 && robot.p.y() > height / 2
    ).count();
    let robots_in_bottom_right = robots.iter().filter(
      |robot| robot.p.x() > width / 2 && robot.p.y() > height / 2
    ).count();

    println!("Robots in top left: {}", robots_in_top_left);
    println!("Robots in top right: {}", robots_in_top_right);
    println!("Robots in bottom left: {}", robots_in_bottom_left);
    println!("Robots in bottom right: {}", robots_in_bottom_right);

    let total_robots = robots_in_top_left + robots_in_top_right + robots_in_bottom_left + robots_in_bottom_right;

    let sum_robot_positions = robots_in_top_left * robots_in_top_right * robots_in_bottom_left * robots_in_bottom_right;

    println!("Total robots: {}", total_robots);
    println!("Total robots: {}", sum_robot_positions);
    
    ()

}