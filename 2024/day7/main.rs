use std::fs;
use std::env;
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

    //println!("Found text:\n{}", contents);
    contents
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    
    let mut result = Vec::new();
    for line in input.lines() {
        let mut line_vec = Vec::new();
        for word in line.split_whitespace() {
            line_vec.push(word);
        }
        result.push(line_vec);
    }
    result
}

fn parse_line(line: Vec<&str>) -> (i64, Vec<i64>) {
    let total = line[0].replace(":", "").parse::<i64>().unwrap();
    let numbers: Vec<i64> = line[1..].iter().map(|x| x.parse::<i64>().unwrap()).collect();
    (total, numbers)
}

const MULTIPLY: &str = "MULTIPLY";
const ADD: &str = "ADD";
const CONCAT: &str = "CONCAT";


fn calculate(numbers: &Vec<i64>, operations: Vec<&&str>) -> i64 {

    let mut running_total = numbers[0];
    let mut index = 1;

    operations.iter().for_each(|op| {
        //let previous_total = running_total;
        let next_number = numbers[index];
        if op.contains(MULTIPLY) {
            running_total = running_total * next_number;
        } else if op.contains(ADD) {
            running_total = running_total + next_number;
        } else if op.contains(CONCAT) {
            running_total = (running_total.to_string() + &next_number.to_string()).parse::<i64>().unwrap()
        } else {
            panic!("Unknown operator {}", op)
        }
        //println!("{} {} {} = {}", previous_total, op, next_number, running_total);
        index += 1;
    });

    running_total

}

fn variations_up_to_length<T>(items: &[T], length: usize) -> impl Iterator<Item = Vec<&T>> {
    (1..=length).flat_map(move |n| {
        std::iter::repeat(items.iter())
            .take(n)
            .multi_cartesian_product()
            .filter(move |ops| ops.len() == length)
    })
}

fn process_total_and_numbers(total: i64, numbers: Vec<i64>) -> i64 {

    let numbers_length = numbers.len();

    let items = vec![MULTIPLY, ADD, CONCAT];

    let variations = variations_up_to_length(&items, numbers_length -1 );

    let mut result = 0;
    variations.for_each(|operations|{
        let possible_total = calculate(&numbers, operations);
        if possible_total == total {
            result = total;
        }
    });
    
    result
}


fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let input = parse_input(content.as_str());    
    
    let mut all_total = 0;

    input.iter().for_each(|line| {
        let (total, numbers) = parse_line(line.clone());
        //println!("");
        //println!("{} : {:?}", total, numbers);
        let total = process_total_and_numbers(total, numbers);

        all_total += total;


    });

    println!("ALL Total {}", all_total)

}