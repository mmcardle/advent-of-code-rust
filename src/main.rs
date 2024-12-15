use core::panic;
// ↳ main.rs
use std::{
    collections::{HashMap, HashSet},
    env
};
pub mod lattice;

use lattice::{Lattice, E, N, S, W};

fn parse_input(input: &str) -> Vec<Vec<String>> {
    return input
        .lines()
        .map(|line| {
            let chars = line.chars();
            return chars.map(|c| c.to_string()).collect();
        })
        .collect();
}

fn create_regions(m: HashMap<Lattice, String>) -> Vec<(String, HashSet<Lattice>)> {
    fn fill(m: &HashMap<Lattice, String>, region: &mut HashSet<Lattice>, p: Lattice, counter: i32) {
        //println!("fill {:?} as region {:?} with {:?}", m, region, p);
        if counter > 1000000 {
            panic!("o no recorsion maxed out")
        }
        region.insert(p);
        for q in p.across(false) {
            let p_val = m.get(&p).unwrap();
            let default_string = String::new();
            let q_val = m.get(&q).unwrap_or(&default_string);
            if p_val.eq(q_val) && !region.contains(&q) {
                //println!("{:?} == {:?} -- YES", p_val, q_val);
                fill(m, region, q, counter + 1);
            } else {
                if q_val.eq(&default_string) {
                    //println!("{:?} == {:?} -- NO", p_val, q_val);
                } else {
                    //println!("{:?} != {:?}", p_val, q_val);
                }
            }
        }
    }

    let mut left: HashSet<Lattice> = m.keys().cloned().collect();
    let mut regions: Vec<(String, HashSet<Lattice>)> = Vec::new();

    while !left.is_empty() {
        let p = left.iter().next().cloned().unwrap();
        let was_in = left.remove(&p);

        if !was_in {
            panic!("was not in ")
        }

        let val = &m[&p];
        let mut region: HashSet<Lattice> = HashSet::new();
        fill(&m, &mut region, p, 0);

        // Remove all in regions from left
        left = left.difference(&region).cloned().collect();

        regions.push((val.clone(), region));
    }

    regions
}

fn price_part1(region: &HashSet<Lattice>) -> usize {
    let area = region.len();
    let perimeter = permieter(region);
    area * perimeter
}


fn permieter(region: &HashSet<Lattice>) -> usize {
    let mut perimeter = 0;
    for p in region.iter() {
        for q in p.across(false) {
            if !region.contains(&q) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn outer_perimeter(region: &HashSet<Lattice>) -> HashSet<(Lattice, Lattice)> {
    /*
    def outer_perimeter(region: set):
        outer = set()
        for p in region:
            for q in p.across():
                if q not in region:
                    outer.add((q, p - q))  # p-q is the direction the cell touches the region
        return outer
     */

     let mut outer: HashSet<(Lattice, Lattice)> = HashSet::new();

     region.iter().for_each(|p| {
        //println!("FINDING ACROSS p == {:?}", p);
        for q in p.across(false) {
            if !region.contains(&q) {
                //println!("ADDING q == {:?}", q);
                //outer.insert((q, p.sub(q)));
                outer.insert((q, p.direction(&q)));
            }
        } 
     });

     //println!("Outer Perimeter length {:?}", outer.len());

     outer
}

fn price_part2(region: &HashSet<Lattice>) -> usize {
    /*
        perimeter = 0
        outer = outer_perimeter(region)
        while outer:
            p, touch_d = outer.pop()
            directions = (N, S) if touch_d in (W, E) else (W, E)
            for d in directions:
                q = p
                while True:
                    q += d
                    if (q, touch_d) in outer:
                        outer.remove((q, touch_d))
                    else:
                        break
            perimeter += 1
        return perimeter * len(region)
    */
    let mut perimeter = 0;
    let mut outer = outer_perimeter(region);
    while !outer.is_empty() {
        //println!("Outer Length is now {:?}", outer.len());
        let (p, touch_d ) = outer.iter().next().cloned().unwrap();
        outer.remove(&(p, touch_d));
        let directions = if touch_d == N || touch_d == S { Vec::from([W, E]) } else { Vec::from([N, S]) };
        for dir in directions.iter() {
            let mut q = p.clone();
            loop {
                q = q + *dir;
                if outer.contains(&(q, touch_d)) {
                    outer.remove(&(q, touch_d));
                } else {
                    break;
                }
            }
        }
        perimeter += 1;
    }
    perimeter * region.len()
}



fn total_price(data: Vec<Vec<String>>) -> (usize, usize) {
    let m = Lattice::lattice_map(&data, |x| x.to_string());

    let regions = create_regions(m);

    let mut total = 0;
    let mut total_part2 = 0;
    for region in regions {
        let _part1 = price_part1(&region.1);
        let _part2 = price_part2(&region.1);
        println!("Region {:?} total={} total part 2={}", region.0, _part1, _part2);
        total += _part1;
        total_part2 += _part2;
    }

    (total, total_part2)
}

fn get_filename_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a filename as an argument");
    }
    args[1].clone()
}

fn read_file(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn main() {
    let filename = get_filename_from_args();
    let contents = read_file(&filename);

    let data = parse_input(&contents);

    print_grid(&data);

    let res = total_price(data);

    println!("Result: {:?}", res);
}

fn print_grid(data: &Vec<Vec<String>>) {
    print!("col  ");
    for (j, _) in data[0].iter().enumerate() {
        print!(" {} ", j);
    }
    println!();
    for (i, d) in data.iter().enumerate() {
        print!("row {}", i);
        for c in d.iter() {
            print!(" {} ", c);
        }
        println!();
    }
}
