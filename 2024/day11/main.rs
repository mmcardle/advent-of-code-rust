use std::collections::HashMap;
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

fn process_stone(stone: &str) -> Vec<&str> {
    if stone == "0" {
        //If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        return Vec::from(["1"])
    } else if stone.len() % 2 == 0 {
        // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. 
        // The left half of the digits are engraved on the new left stone, 
        // and the right half of the digits are engraved on the new right stone. 
        // (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        let (lhs, rhs) = stone.split_at(stone.len() / 2);
        let lhs_str = Box::leak(lhs.parse::<i64>().unwrap().to_string().into_boxed_str()) as &str;
        let rhs_str = Box::leak(rhs.parse::<i64>().unwrap().to_string().into_boxed_str()) as &str;
        return Vec::from([lhs_str, rhs_str])
    } else {
        //If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
        let stone_int = stone.parse::<i64>().unwrap();
        let new_stone = stone_int * 2024;
        return Vec::from([Box::leak(new_stone.to_string().into_boxed_str()) as &str])
    }
}

fn blink(stones: Vec<&str>) -> Vec<&str> {
    let mut new_stones = Vec::new();

    for stone in stones {
        new_stones.extend(process_stone(stone));
    }

    new_stones
}

fn blink_length(blinks: usize, stones: Vec<&str>, cache: &HashMap<&str, usize>) -> usize {
    let mut total = 0;

    if blinks == 0 {
        return total;
    }

    for stone in stones {
        if cache.contains_key(stone) {
            total = total + cache.get(stone).unwrap();
        } else {
            total = total + blink_length(blinks - 1, process_stone(stone), cache);
        }
    }

    println!("Total for {:?} = {}", stones, total);

    total

}

fn parse_input(input: &str) -> Vec<&str> {
    let mut parsed_input = Vec::new();
    for line in input.lines() {
        line.split_whitespace().for_each(|item| {
            parsed_input.push(item);
        });
    }
    parsed_input
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let mut stones = parse_input(content.as_str());    

    println!("{:?}", stones);

    // part2 
    let blinks = 6;

    let total2 = blink_length(blinks, stones.clone(), &HashMap::new());

    println!("Total pArt 2 {}", total2);
    
    for _ in 0..blinks {
        stones = blink(stones.clone());
    }
    println!("Result {}", &stones.len());

}