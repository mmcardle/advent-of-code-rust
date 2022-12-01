use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;


fn main() {
    let result1 = solve(String::from("day4_test_input"));
    println!("Result1: {}", result1);
    let result2 = solve(String::from("day4_real_input"));
    println!("Result2: {}", result2);
}

fn solve(file_path: String) -> usize {
    
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file).lines();
    let test_data = buf.collect::<Result<Vec<_>, _>>().expect("error reading file");

    let numbers = test_data[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    //println!("Numbers {:?}", numbers);
    let rest = &test_data[2..];
    //println!("Rest {:?}", rest);

    let mut game: Vec<Vec<Vec<i32>>> = vec![];

    rest.chunks(6).for_each(|x| {
        let mut grid: Vec<Vec<i32>> = vec![];
        for row in x.iter() {
            let arow = row.split(" ").filter(
                |x| !x.is_empty()
            ).map(
                |x| x.parse::<i32>().unwrap()
            ).collect::<Vec<i32>>();
            
            if arow.len() > 0 {
                grid.push(arow);
            }
        }
        //println!("GRID {:?}", grid);
        game.push(grid);
    });

    let mut winners: Vec<i32> = vec![];

    for number in numbers {
        for x in 0..game.len() {
            for y in 0..game[x].len() {
                for z in 0..game[x].len() {
                    if game[x][y][z] == number {
                        game[x][y][z] = -1;
                        //println!("{} {} {}", x, y, z);
                        let complete = check_board(&mut game[x]);
                        if complete {
                            let mut score = 0;
                            for row in game[x].iter() {
                                for cell in row.iter() {
                                    if cell != &-1 {
                                        score += cell;
                                    }
                                }
                            }
                            let dd = x as i32 + 1;
                            if !winners.contains(&dd) {
                                winners.push(dd);
                                println!("Board Winner {} = {}", x + 1, winners.len());
                                //println!("Complete {:?} board={} number={} score={}", game[x], x, number, score);
                                println!("Complete board={} number={} boardscore={} toalscore={}", x, number, score, number * score);
                            } else {
                                //println!("DUPLICATED Winner {}", x);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Winners {:?}", winners);
    return 99999;
}

fn check_board(board: &mut Vec<Vec<i32>>) -> bool {
    for row in board.iter_mut() {
        let row_complete = row.iter().all(|x| *x == -1);
        if row_complete {
            return true;
        }
    }
    for row in 0..5 {
        let tcol = [
            board[0][row],
            board[1][row],
            board[2][row],
            board[3][row],
            board[4][row],
        ];
        let col_complete = tcol.iter().all(|x| *x == -1);
        if col_complete {
            return true;
        }
    }
    return false;
}