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

fn parse_input(input: &str) -> Vec<Vec<i32>> {
  // parse 2 integers on each line
  let mut numbers = Vec::new();
  for line in input.lines() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      numbers.push(parts.iter().map(|x| x.parse::<i32>().unwrap()).collect());
  }
  return numbers;
}


fn diff_greater_than_3(a: i32, b: i32) -> bool {
  let diff = (a - b).abs();
  if diff > 3 {
    return true;
  }
  return false;
}

fn print_number_list_with_diffs(number_list: Vec<i32>) {
  let numbers_as_pairs = do_numbers_as_pairs(number_list);
  for pair in numbers_as_pairs {
    let diff = diff_greater_than_3(pair.0, pair.1);
    let formatted_diff = if diff {
      format!("**\x1b[1;31m{}\x1b[0m**", (pair.0 - pair.1).abs())
    } else {
      if pair.0 == pair.1 {
        format!("**\x1b[1;31m{}\x1b[0m**", (pair.0 - pair.1).abs())
      } else {
        format!("{}", (pair.0 - pair.1).abs())
      }
    };
    print!("{} {} {} ", pair.0, formatted_diff, pair.1);
  }
  println!()
}

fn do_numbers_as_pairs(number_list: Vec<i32>) -> Vec<(i32, i32)> {
  let mut numbers_as_pairs = Vec::new();
  for i in 0..number_list.len() - 1 {
    numbers_as_pairs.push((number_list[i], number_list[i + 1]));
  }
  return numbers_as_pairs;
}


fn is_valid(number_list: Vec<i32>) -> bool {

  let numbers_as_pairs = do_numbers_as_pairs(number_list.clone());

  //print_number_list_with_diffs(number_list);

  let mut increasing = false;
  let mut decreasing = false;

  for pair in numbers_as_pairs {
    if pair.0 == pair.1 {
      //println!("Same: {:?}", number_list);
      return false;
    }
    if pair.0 > pair.1 {
      increasing = true;
    }
    if pair.0 < pair.1 {
      decreasing = true;
    }
    if increasing && decreasing {
      //println!("Increasing and decreasing: {:?}", number_list);
      return false;
    }
    let gt3 = diff_greater_than_3(pair.0, pair.1);
    if  gt3 {
      //println!("Diff greater than 3: {:?} : {} {}", number_list, pair.0, pair.1);
      return false;
    }
  }
  println!("Valid: {:?}", number_list);
  print_number_list_with_diffs(number_list);
  return true;
}


fn is_valid_with_removal(number_list: Vec<i32>) -> bool {

  for i in 0..number_list.len() {
    let mut number_list_copy = number_list.clone();
    number_list_copy.remove(i);
    let is_valid = is_valid(number_list_copy);
    if is_valid {
      return true;
    }
  }

  return false;
}

fn main() {
  let content = read_file(get_filename_from_args().as_str());

  let numbers = parse_input(content.as_str());    

  let slice = &numbers[..];

  let mut valid_count = 0;
  let mut invalid_count = 0;

  for number_list in slice {
    let is_valid = is_valid_with_removal(number_list.to_vec());
    if is_valid {
      valid_count += 1;
    } else {
      invalid_count += 1;
    }
  }
  println!("Valid count: {} Invalid Count {}", valid_count, invalid_count);
}