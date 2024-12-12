use std::collections::HashSet;
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

    contents
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    // parse 2 integers on each line

    return input.lines().map(|line| {
        let chars = line.chars();
        return chars.collect();
    }).collect();  
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!(" {} ", cell);
        }
        println!();
    }
}

fn format_str_as_green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}


fn prcoess_grid(grid: Vec<Vec<char>>) {

    let unique_chars_in_grid = grid.iter().flatten().copied().collect::<HashSet<char>>();
    println!("Unique chars in grid: {:?}", unique_chars_in_grid);

    unique_chars_in_grid.iter().for_each(|c| {
        process_grid_char(grid.clone(), *c);
    });

}

fn adjacent_points(grid: Vec<Vec<char>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let (i, j) = point;
    if i > 0 {
        points.push((i - 1, j));
    }
    if i < grid.len() - 1 {
        points.push((i + 1, j));
    }
    if j > 0 {
        points.push((i, j - 1));
    }
    if j < grid[0].len() - 1 {
        points.push((i, j + 1));
    }
    return points;
}

fn process_grid_char(grid: Vec<Vec<char>>, c: char) -> usize {
    println!("\nProcessing grid for char: {}", c);
    let mut points= Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == c {
                points.push((i, j));
                print!(" {} ", format_str_as_green(cell.to_string().as_str()));
            } else {
                print!(" . ");
            }
        }
        println!();
    }
    
    println!("Points: {:?}", points);
    points.iter().for_each(|p| {
        let mut fences = 4;
        let adj_points = adjacent_points(grid.clone(), *p);
        println!("Adjacent points for {:?}: {:?}", p, adj_points);
        if adj_points.iter().any(|adj_p| {
            let (i, j) = adj_p;
            grid[*i][*j] == c
        }) {
            fences -= 1;
        } 
        println!("{:?} has {} fences", p, fences);
    });


    return 0;
}


fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let grid = parse_input(content.as_str());    

    print_grid(&grid);

    prcoess_grid(grid);

}