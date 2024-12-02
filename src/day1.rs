use std::io::{self};
use crate::utils::read_lines;

pub fn day1(filename: &str) -> io::Result<()> {

    let mut totals = Vec::new();
    totals.push(0);

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match line?.parse::<i32>() {
                Ok(n) => {
                    //println!("{}", n);
                    let l = totals.len();
                    totals[l - 1] = totals[l -1] + n;
                },
                Err(_e) => {
                    totals.push(0);
                    //println!("{}", e);
                },
            }
        }
    }

    totals.sort();
    let tot: Vec<i32> = totals.into_iter().rev().collect();
    
    let top3: &[i32] = &tot[0..3];
    let sum: i32 = top3.iter().sum();
    
    println!("Top3 {:?}", sum);
    
    Ok(())
}

