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
    //println!("Found text:\n{}", contents);
    contents
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
        //If none of the other rules apply, the stone is replaced by a new stone; 
        // the old stone's number multiplied by 2024 is engraved on the new stone.
        let stone_int = stone.parse::<i64>().unwrap();
        let new_stone = stone_int * 2024;
        return Vec::from([Box::leak(new_stone.to_string().into_boxed_str()) as &str])
    }
}


fn blink_times(blinks: usize, stones: Vec<&str>, cache: &mut HashMap<String, usize>) -> usize {
    let mut total = 0;
    let cache_key_suffix = stones.join(",");
    let cache_prefix= format!("{:?}::", blinks);
    let cache_key = format!("{}{}", cache_prefix, cache_key_suffix);

    if blinks == 0 {
        return stones.len();
    }

    if cache.contains_key(cache_key.as_str()) {
        return *cache.get(cache_key.as_str()).unwrap();
    }

    for stone in &stones {
        let new_stones = process_stone(stone);
        let this_total = blink_times(blinks - 1, new_stones , cache);
        total += this_total;
    }

    cache.insert(cache_key.clone(), total);
    return total;
}


fn main() {
    let content = read_file(get_filename_from_args().as_str());
    let stones = parse_input(content.as_str());    
    let total = blink_times(300, Vec::from(stones), &mut HashMap::new());
    println!("Total {}", total);
}