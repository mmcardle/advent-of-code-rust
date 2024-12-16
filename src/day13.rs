use std::fs;
use std::env;
use std::os::linux::raw;
use std::usize;

use itertools::Itertools;

use crate::lattice;

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

#[derive(Debug)]
struct Button {
  x: usize,
  y: usize,
}

impl Button {
  pub fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }
}

#[derive(Debug)]
struct Target {
  x: usize,
  y: usize,
}


impl Target {
  pub fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }
}

#[derive(Debug)]
struct Competition {
  a: Button,
  b: Button,
  target: Target,
}

impl Competition {
  pub fn new(a: Button, b: Button, p: Target) -> Self {
    Self { a, b, target: p}
  }

  pub fn run(&self) {
    println!("Running {:?}", self);
    panic!("not implemented")
    
  }
}

fn to_i64(value: &&str) -> i64 {
  value.parse::<i64>().expect("Unable to parse string to i64")
}

fn parse_input(input: &str) {

  let raw_parts = input.lines().map(|line| {
    let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    parts
  }).filter(|x| x.len() > 0).map(|parts| {
    parts
  });

  // split into chunks of 3

  let binding = raw_parts.chunks(3);
  let mut xxx: Vec<_> = binding.into_iter().collect();

  let mut competitions: Vec<Competition> = Vec::new();

  xxx.iter_mut().for_each(|f |{
      let button1_str = &f.next().unwrap()[2..];
      let button2_str = &f.next().unwrap()[2..];
      let target_str =   &f.next().unwrap()[1..];

      let button1_x = &button1_str[0].strip_prefix("X+").unwrap().strip_suffix(",").unwrap();
      let button1_y = &button1_str[1].strip_prefix("Y+").unwrap();

      let button2_x = &button2_str[0].strip_prefix("X+").unwrap().strip_suffix(",").unwrap();
      let button2_y = &button2_str[1].strip_prefix("Y+").unwrap();

      let target_x  = &target_str[0].strip_prefix("X=").unwrap().strip_suffix(",").unwrap();
      let target_y  = &target_str[1].strip_prefix("Y=").unwrap();

      let button1_x = to_i64(&button1_x);
      let button1_y = to_i64(&button1_y);

      let button2_x = to_i64(&button2_x);
      let button2_y = to_i64(&button2_y);

      let target_x = to_i64(&target_x);
      let target_y = to_i64(&target_y);

      let button1 = Button::new(button1_x as usize, button1_y as usize);
      let button2 = Button::new(button2_x as usize, button2_y as usize);
      let target = Target::new(target_x as usize, target_y as usize);
      let c = Competition::new(button1, button2, target);

      println!("{:?}", c);

      competitions.push(c);

  });


  println!("{:?}", competitions);

  competitions.iter().for_each(|competition|{
    competition.run();
  });
  /*
  raw_parts.for_each(|f|{
    println!("XX {:?}", f)
  });
  */



    
    
}

pub fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let input = parse_input(content.as_str());    

    println!("{:?}", input);

    let lat = lattice::Lattice::new(1, 2);

    println!("{:?}", lat);

}