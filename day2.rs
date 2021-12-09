use std::io::BufRead;
use std::fs::File;
use std::io::{self, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("day2_test_input").expect("no such file");
    let buf = BufReader::new(file).lines();
    let test_data = buf.collect::<Result<Vec<_>, _>>().expect("error reading file");

    println!("{}", process(test_data));

    let file = File::open("day2_input").expect("no such file");
    let buf = BufReader::new(file).lines();
    let real_data = buf.collect::<Result<Vec<_>, _>>().expect("error reading file");

    println!("{}", process(real_data));

    Ok(())
}

fn process(data: Vec<String>) -> i32 {

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for l in data {
        let mut split = l.split(" ");
        let command = split.next().unwrap();
        let amount: i32 = split.next().unwrap().parse::<i32>().unwrap();
        println!("{} {:?}", command, amount);

        match command {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            },
            "down" => {
                aim += amount;
            },
            "up" => {
                aim -= amount;
            },
            _ => println!("Bad Command"),
        }

    }

    println!("Result {} {}", horizontal, depth);
    
    return horizontal * depth;
}