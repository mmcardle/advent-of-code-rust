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
    let disky: Vec<(usize, i64, bool)> = first_line.split("").filter(|c| {
        c.len() == 1
    }).enumerate().map(|(index, char) | {
        let i = char.parse::<i64>().unwrap();
        for _ in 0..i {
            if index % 2 == 0 {
                betterdisk.push((index/2, i, true));
            } else {
                betterdisk.push((0, 0, false));
            }
        }
        (index, i, i % 2 != 0)
    }).collect();
    betterdisk
}

fn print_disk_map(disk_map: Vec<(usize, i64, bool)>) -> String {
    let mut s: String = String::from("");
    for i in 0..disk_map.len() {
        let item = disk_map[i];
        if item.2 {
            s.push_str(&format!("{}", item.0));
            /*for _ in 0..disk_map[i].1 {
                s.push_str(&format!("{}", i/2));
            }*/
        } else {
            s.push_str(&format!("."));
            /*for _ in 0..disk_map[i].1 {
                s.push_str(&format!("."));
            }*/
        }
    }
    s
}


fn move_single_sector_to_end(mut disk_map: String) -> String {

    const RADIX: u32 = 10;
    let last_digit = disk_map.rfind(char::is_alphanumeric).unwrap();
    let first_dot =  disk_map.find(".").unwrap();

    if last_digit <= first_dot {

        // Disk is good

        println!("{}", &disk_map);

        let iter = disk_map.chars().enumerate();
        let mut total = 0;

        iter.for_each(|f| {
            let disk_id = f.0;
            let size = f.1;
            if size != '.' {
                let size_int = size.to_digit(RADIX).unwrap();
                let new_total = total + (disk_id * (size_int as usize));
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
    //let mut disk = disk.clone();
    disk.swap(last_disk_index, first_space_index);

    //disk[last_disk_index].1 -= 1;
    //disk[first_space_index].1 -= 1;
    //disk.insert(first_space_index, (disk[last_disk_index].0, 1, true));

    //println!("Disk Map New: {:?}", disk);

    //println!("{}", print_disk_map(disk.clone()));

    return (false, disk);

}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let mut disk_map = parse_input(content.as_str());
    println!("Disk Map: {:?}", disk_map);
    
    //let mut formatted_disk_map = print_disk_map(disk_map.clone());
    //println!("{}", formatted_disk_map);

    let mut counter = 0;

    println!("{}", print_disk_map(disk_map.clone()));

    loop {
        let (formatted, new_disk_map) = format_disk(disk_map.clone());
        disk_map = new_disk_map.clone();
        if formatted  {
            println!("{}", print_disk_map(new_disk_map.clone()));
            break;
        }
        counter += 1;
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

    /*

    loop {
        formatted_disk_map = move_single_sector_to_end(formatted_disk_map.clone());
        if formatted_disk_map == "" {
            break;
        }
        //println!("{}", formatted_disk_map);
    }
    */

}