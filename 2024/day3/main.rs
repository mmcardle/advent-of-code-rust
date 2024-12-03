use core::num;
use std::fs;
use std::env;
use regex::Regex;



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

fn parse_str_asi32(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // parse 2 integers on each line

    let re = Regex::new(r"mul\((?<one>[0-9]+),(?<two>[0-9]+)\)").unwrap();
    // 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
    let dates: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    println!("{:?}", dates);

    let mut total = 0;

    dates.iter().for_each(|date| {
        let caps = re.captures(date).unwrap();
        let one = caps.name("one").unwrap().as_str();
        let two = caps.name("two").unwrap().as_str();
        let num1 = one.parse::<i32>().unwrap();
        let num2 = two.parse::<i32>().unwrap();
        let sum = num1 * num2;
        println!("{} * {} = {}", num1, num2, sum);
        total += sum;
    });

    println!("Total: {}", total);


    return Ok(());
    
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let numbers = parse_input(content.as_str());    

    println!("{:?}", numbers);

}