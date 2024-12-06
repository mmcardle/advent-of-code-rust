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

fn parse_input(input: &str) -> Vec<Vec<str>> {
    // parse 2 integers on each line

    
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let input = parse_input(content.as_str());    

    println!("{:?}", input);

}