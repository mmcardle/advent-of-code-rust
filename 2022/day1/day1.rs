use std::io::BufRead;
use std::fs::File;
use std::io::{self, BufReader};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    //let mut totals = [0];
    let mut totals = Vec::new();
    totals.push(0);

    for line in reader.lines() {

        // or, to be safe, match the `Err`
        match line?.parse::<i32>() {
            Ok(n) => {
                println!("{}", n);
                let l = totals.len();
                totals[l - 1] = totals[l -1] + n;
            },
            Err(e) => {
                totals.push(0);
                println!("{}", e);
            },
        }

    }

    println!("{:?}", totals);
    let maxValue = totals.iter().max().unwrap();
    println!("Max {}", maxValue);
    
    totals.sort();
    let tot: Vec<i32> = totals.into_iter().rev().collect();
    
    println!("Sorted {:?}", tot);
    
    let top3 = tot[0] + tot[1] + tot[2];
    
    println!("Top3 {:?}", top3);
    
    Ok(())
}

