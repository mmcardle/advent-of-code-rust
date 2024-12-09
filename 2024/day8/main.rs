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

fn parse_input(input: &str) -> Vec<i64>{
    let first_line = input.lines().next().unwrap();
    let char_list: Vec<i64> = first_line.split("").filter(|c| {
        c.len() == 1
    }).map(| char| {
        let i = char.parse::<i64>().unwrap();
        i
    }).collect();
    char_list
}

fn print_disk_map(disk_map: Vec<i64>) -> String {
    let mut s: String = String::from("");
    for i in 0..disk_map.len() {
        if i % 2 == 0 {
            for _ in 0..disk_map[i] {
                s.push_str(&format!("{}", i/2));
            }
        } else {
            for _ in 0..disk_map[i] {
                s.push_str(&format!("."));
            }
        }
    }
    s
}

fn move_single_sector_to_end(mut disk_map: String) -> String {

    const RADIX: u32 = 10;
    let last_digit = disk_map.rfind(char::is_alphanumeric).unwrap();
    let first_dot =  disk_map.find(".").unwrap();

    if last_digit < first_dot {

        // Disk is good

        println!("{}", &disk_map);

        let mut iter = disk_map.chars().enumerate();
        let mut total = 0;

        iter.for_each(|f| {
            let disk_id = f.0;
            let size = f.1;
            if size != '.' {
                let size_int = size.to_digit(RADIX).unwrap();
                let new_total = total + disk_id * (size_int as usize);
                println!("{} + {} * {} = {}", total, disk_id, size_int, new_total);
                total = new_total;
            }
        });

        println!("Total {}", total);

        return String::from("")
    }

    let replace_with   = disk_map[last_digit..last_digit + 1].to_string();

    //println!("Swap index {} with index {} as a {}", last_digit, first_dot, replace_with);

    disk_map.replace_range(last_digit..last_digit+1, ".");
    disk_map.replace_range(first_dot..first_dot+1, &replace_with);

    disk_map
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let disk_map = parse_input(content.as_str());    
    let mut formatted_disk_map = print_disk_map(disk_map);
    println!("{}", formatted_disk_map);

    let maxy: i64 = i64::MAX;
     while true {
        formatted_disk_map = move_single_sector_to_end(formatted_disk_map.clone());
        if formatted_disk_map == "" {
            break;
        }
        //println!("{}", formatted_disk_map);
    }

}