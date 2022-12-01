use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<i32>>
}

impl Grid {
    fn create(size: usize) -> Grid {
        let mut rows = vec![];
        for _i in 0..size {
            let mut values = vec![];
            for _j in 0..size {
                values.push(0);
            }
            rows.push(values);
        }
        Grid {
            rows
        }
    }
}

impl Vent {
    fn create(x1: u32, y1: u32, x2: u32, y2: u32) -> Vent {
        Vent {
            from: Point {
                x: x1,
                y: y1,
            },
            to: Point {
                x: x2,
                y: y2,
            },
        }
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Vent {
    from: Point,
    to: Point,
}


fn main() {
    let result1 = with_file(String::from("day5_test_input"), 10);
    println!("Result1: {}", result1);
    let result2 = with_file(String::from("day5_real_input"), 1000);
    println!("Result2: {}", result2);
}

fn with_file(file_path: String, size: usize) -> usize {
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file).lines();
    let test_data = buf.collect::<Result<Vec<_>, _>>().expect("error reading file");
    let result = solve(test_data, size);
    result
}

fn solve(test_data: Vec<String>, size: usize) -> usize {

    println!("All {:?}", test_data);

    let mut vents: Vec<Vent> = vec![];
    
    for line in test_data {
        let line_split = line.split(" -> ").map(|x| x.parse::<String>().unwrap()).collect::<Vec<String>>();
        let line_split_0 = line_split[0].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let line_split_1 = line_split[1].split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        vents.push(Vent::create(line_split_0[0], line_split_0[1], line_split_1[0], line_split_1[1]));
    }

    let mut grid = Grid::create(size);

    for vent in &vents {
        //println!("{:?}", vent);
        if vent.from.x == vent.to.x && vent.from.y == vent.to.y {
            panic!("crash and burn");
        }
        if vent.from.x == vent.to.x {
            if vent.from.y < vent.to.y {
                println!("X normal {:?}", vent);
                for i in vent.from.y..vent.to.y + 1 {
                    grid.rows[i as usize][vent.from.x as usize] += 1;
                }
            }
            if vent.from.y > vent.to.y {
                println!("X reversed {:?}", vent);
                for i in vent.to.y..vent.from.y + 1 {
                    grid.rows[i as usize][vent.from.x as usize] += 1;
                }
            }
        }
        if vent.from.y == vent.to.y {
            if vent.from.x < vent.to.x {
                println!("Y normal {:?}", vent);
                for i in vent.from.x..vent.to.x + 1 {
                    grid.rows[vent.from.y as usize][i as usize] += 1;
                }
            }
            if vent.from.x > vent.to.x {
                println!("Y reversed {:?}", vent);
                for i in vent.to.x..vent.from.x + 1 {
                    grid.rows[vent.from.y as usize][i as usize] += 1;
                }
            }
        }
    }

    for row in &grid.rows {
        for value in row {
            print!("{}", value);
        }
        println!("");
    }
    let counter = grid.rows.iter().map(|x| x.iter().filter(|y| **y > 1).count()).sum::<usize>();
    println!("{}", counter);

    counter

}