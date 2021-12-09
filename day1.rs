use std::io::BufRead;
use std::fs::File;
use std::io::{self, BufReader};


fn main() -> io::Result<()> {
    let data = vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ];

    let file = File::open("day1_input").unwrap();
    let data2: Vec<i32> = BufReader::new(file)
        .lines()
        .flatten()                             // gets rid of Err from lines
        .flat_map(|line| line.parse::<i32>())  // ignores Err variant from Result of str.parse
        .collect();

    let result = process(data.clone());
    println!("part1 test result is {}", result);

    let result2 = process(data2.clone());
    println!("part1 real result is {}", result2);

    let result3 = process_part2(data.clone());
    println!("part1 test result is {}", result3);

    let result4 = process_part2(data2.clone());
    println!("part2 result is {}", result4);

    Ok(())
}

fn process_part2(data: Vec<i32> ) -> i32 {
    let mut x = 0;
    let i = data.len() - 3;
    let mut counter = 0;
    while x < i {
        let one = data[x];
        let two = data[x + 1];
        let three = data[x + 2];
        let sum = one + two + three;
        let next_one = data[x + 1];
        let next_two = data[x + 2];
        let next_three = data[x + 3];
        let next_sum = next_one + next_two + next_three;
        if next_sum > sum {
            counter += 1;
        }
        x = x + 1;
    }
    return counter;
}

fn process(data: Vec<i32> ) -> i32 {
    let mut x = 0;
    let i = data.len() - 1;
    let mut counter = 0;
    while x < i {
        let greater = data[x] < data[x + 1];
        if greater {
            counter += 1;
        }
        x = x + 1;
    }
    return counter;
}