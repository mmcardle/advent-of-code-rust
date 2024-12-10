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

fn parse_input(input: &str) -> Vec<(usize, i64, bool)>{

    let mut betterdisk = Vec::new();

    let first_line = input.lines().next().unwrap();
    first_line.split("").filter(|c| {
        c.len() == 1
    }).enumerate().for_each(|(index, char) | {
        let i = char.parse::<i64>().unwrap();
        for _ in 0..i {
            if index % 2 == 0 {
                betterdisk.push((index/2, i, true));
            } else {
                betterdisk.push((0, 0, false));
            }
        }
    });
    betterdisk
}

fn print_disk_map(disk_map: Vec<(usize, i64, bool)>) -> String {
    let mut s: String = String::from("");
    for i in 0..disk_map.len() {
        let item = disk_map[i];
        if item.2 {
            s.push_str(&format!("{}", item.0));
        } else {
            s.push_str(&format!("."));
        }
    }
    s
}


fn format_disk(mut disk: Vec<(usize, i64, bool)>) -> (bool, Vec<(usize, i64, bool)>) {
        
    let last_disk_index = disk.iter().rposition(|(_, _, even)| {
        *even 
    }).unwrap();
    //println!("Last disk item {} = {:?}", last_disk_index,  disk[last_disk_index]);

    let first_space_index = disk.iter().position(|(_, _, even)| {
        *even == false
    }).unwrap();
    //println!("First space to fill {} = {:?}", first_space_index , disk[first_space_index]);

    if (last_disk_index < first_space_index) {
        return (true, disk);
    }

    // swap the two
    disk.swap(last_disk_index, first_space_index);
    //println!("Disk Map New: {:?}", disk);

    return (false, disk);

}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let mut disk_map = parse_input(content.as_str());
    println!("Disk Map: {:?}", disk_map);
    
    println!("{}", print_disk_map(disk_map.clone()));

    loop {
        let (formatted, new_disk_map) = format_disk(disk_map.clone());
        disk_map = new_disk_map.clone();
        if formatted  {
            println!("{}", print_disk_map(new_disk_map.clone()));
            break;
        }
    }

    println!("{:?}", disk_map.clone());

    let mut total = 0;

    for i in 0..disk_map.len() {
        let item = disk_map[i];
        if item.2 == true {
            println!("{} :: {} + {} * {} = {}  {:?}", i, total, item.0, i, total + i * item.0, item);
            total += i * item.0;
        }
    }

    println!("LEN {}", disk_map.len());
    println!("Total {}", total);

}