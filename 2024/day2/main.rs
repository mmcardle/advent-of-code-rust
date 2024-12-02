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

  println!("Found text:\n{}", contents);
  contents
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32, i32)> {
  // parse 2 integers on each line
  let mut numbers = Vec::new();
  for line in input.lines() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      let a = parts[0].parse::<i32>().unwrap();
      let b = parts[1].parse::<i32>().unwrap();
      let c = parts[2].parse::<i32>().unwrap();
      let d = parts[3].parse::<i32>().unwrap();
      let e = parts[4].parse::<i32>().unwrap();
      numbers.push((a, b, c, d, e));

  }
  return numbers;
}

fn main() {
  let content = read_file(get_filename_from_args().as_str());

  let numbers = parse_input(content.as_str());    

  println!("{:?}", numbers);

}