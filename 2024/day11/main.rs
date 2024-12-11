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

fn format_str_as_red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

fn read_file(filename: &str) -> String {

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    //println!("Found text:\n{}", contents);
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

fn parse_input(input: &str) -> Vec<&str> {
    let mut parsed_input = Vec::new();
    for line in input.lines() {
        line.split_whitespace().for_each(|item| {
            parsed_input.push(item);
        });
    }
    parsed_input
}
fn format_str_as_green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}
fn string_of_length_N(n: usize) -> String {
    let mut s = String::new();
    for _ in 0..n*2 {
        s.push(' ');
    }
    s
}

fn blink_length<'a>(blinks: usize, stones: Vec<&'a str>, cache: &mut HashMap<&'a str, usize>) -> usize {
    let mut total = 0;

    let padding = string_of_length_N(blinks);

    if blinks == 0 {
        //println!("{} Base case for {:?} = {}", padding, stones.clone(), stones.len());
        return stones.len();
    }

    for stone in &stones {
        //println!("{} Round {} Processing stone {:?}", padding, blinks, stone);
        if cache.contains_key(stone) {
            let red_cache = format_str_as_red("Cache hit");
            let cached_value = cache.get(stone).unwrap();
            //println!("{} {} for {:?} is {}", padding, red_cache, stone, cached_value);
            total += *cached_value;
            //return *cached_value;
        } else {
            ////println!("Cache miss for {:?}", stone);
            let new_stones = process_stone(stone);
            ////println!("New stones {:?} from {}", new_stones, stone);
            let this_total = blink_length(blinks - 1, new_stones , cache);
            total += this_total;
            let cache_green = format_str_as_green("Cache insert");
            //println!("{} {} -> {} for stone {:?}", padding, cache_green, this_total, stone);
            //cache.insert(stone, this_total);
            //return this_total
        }
    }

    //println!("{} Total for {:?} = {}", padding , stones.clone(), total);

    total

}



fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let stones = parse_input(content.as_str());    

    
    // part2 
    
    //let debug1 = blink_length(25, Vec::from(["125", "17"]), &mut HashMap::new());
    //println!("Total DEBUG {}", debug1);
    
    let debug2 = blink_length(75, Vec::from(stones), &mut HashMap::new());
    println!("Total DEBUG {}", debug2);

    return;
    
    println!("{:?}", stones);
    
    let blinks = 6;
    let debug2 = blink_length(6, Vec::from(["17"]), &mut HashMap::new());
    println!("Total DEBUG 2 {}", debug2);

    let debug3 = blink_length(6, Vec::from(["125", "17"]), &mut HashMap::new());
    println!("Total DEBUG 2 {}", debug3);

    let mut cache = HashMap::new();
    let total2 = blink_length(blinks, stones.clone(), &mut cache);

    println!("Total pArt 2 {}", total2);
    
    /*
    for _ in 0..blinks {
        stones = blink(stones.clone());
    }
    println!("Result {}", &stones.len());
    */

}