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

fn parse_input(input: &str) -> Vec<Vec<char>> {
    
    let mut mapping = Vec::new();   

    for line in input.lines() {
        println!("{}", line);
        let parts: Vec<char> = line.chars().collect();
        mapping.push(parts);
    }
    mapping
}

fn process_input(input: Vec<Vec<char>>) {

    // horizontal
    let mut counter = 0;
    for line in &input {
        let length_of_line = line.len();
        for i in 0..length_of_line-3 {
            let slice_of_line = &line[i..i+4];
            if slice_of_line.len() == 4 {
                if slice_of_line[0] == 'X' && slice_of_line[1] == 'M' && slice_of_line[2] == 'A' && slice_of_line[3] == 'S' {
                    println!("Horizontal: Found XMAS at index: {}", i);
                    counter += 1;
                }
                //backwards
                if slice_of_line[3] == 'X' && slice_of_line[2] == 'M' && slice_of_line[1] == 'A' && slice_of_line[0] == 'S' {
                    println!("Horizontal: Found XMAS at index: {}", i);
                    counter += 1;
                }
            }
        }
    }

    // vertical
    let number_of_lines = input.len();
    for i in 0..number_of_lines-3 {
        let slice_of_lines = &input[i..i+4];
        for j in 0..4 {
            let slice_of_line = &slice_of_lines[j];
            for k in 0..slice_of_line.len()-3 {
                let slice_of_slice = &slice_of_line[k..k+4];
                if slice_of_slice.len() == 4 {
                    if slice_of_slice[0] == 'X' && slice_of_slice[1] == 'M' && slice_of_slice[2] == 'A' && slice_of_slice[3] == 'S' {
                        println!("Vertical: Found XMAS at index: {}", k);
                        //counter += 1;
                    }
                    //backwards
                    if slice_of_slice[3] == 'X' && slice_of_slice[2] == 'M' && slice_of_slice[1] == 'A' && slice_of_slice[0] == 'S' {
                        println!("Vertical: Found XMAS at index: {}", k);
                        //counter += 1;
                    }
                }
            }
        }
    }

    // diagonal
    for i in 0..number_of_lines-3 {
        let slice_of_lines = &input[i..i+4];
        for j in 0..4 {
            let slice_of_line = &slice_of_lines[j];
            for k in 0..slice_of_line.len()-3 {
                let slice_of_slice = &slice_of_line[k..k+4];
                if slice_of_slice.len() == 4 {
                    if slice_of_slice[0] == 'X' && slice_of_slice[1] == 'M' && slice_of_slice[2] == 'A' && slice_of_slice[3] == 'S' {
                        println!("Diagonal: Found XMAS at index: {} {} {}", i, j, k);
                        //counter += 1;
                    }
                    //backwards
                    if slice_of_slice[3] == 'X' && slice_of_slice[2] == 'M' && slice_of_slice[1] == 'A' && slice_of_slice[0] == 'S' {
                        println!("Diagonal: Found XMAS at index: {} {} {}", i, j, k);
                        //counter += 1;
                    }
                }
            }
        }
    }

    println!("Total: {}", counter);

}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let input_array = parse_input(content.as_str());    

    println!("{:?}", input_array);

    process_input(input_array);

}