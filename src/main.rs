// ↳ main.rs
use std::{
    collections::{HashMap, HashSet},
    env,
};

use lattice::Lattice;

pub mod lattice;

fn parse_input(input: &str) -> Vec<Vec<String>> {
    return input
        .lines()
        .map(|line| {
            let chars = line.chars();
            return chars.map(|c| c.to_string()).collect();
        })
        .collect();
}

fn create_regions(m: HashMap<Lattice, String>) -> Vec<(String, HashSet<Lattice>)>{
    /*
    def create_regions(m):
        def fill(m: defaultdict, region: set, p: lattice):
            region.add(p)
            for q in p.across():
                if m[q] == m[p] and q not in region:
                    fill(m, region, q)

        left = set(m)
        regions = []
        while left:
            p = next(iter(left))
            region = set()
            fill(m, region, p)
            left -= region
            regions.append((m[p], region))
        return regions
    */

    fn fill(m: &HashMap<Lattice, String>, region: &mut HashSet<Lattice>, p: Lattice, counter: i32) {
        //println!("fill {:?} as region {:?} with {:?}", m, region, p);
        if counter > 100 {
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

    let mut counter = 0;

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

        if counter > 100 {
            panic!("o no")
        }

        counter += 1;
    }

    regions


}

fn total_price(data: Vec<Vec<String>>) -> usize {
    let m = Lattice::lattice_map(&data, |x| x.to_string());

    println!("{:?}", m);

    let regions = create_regions(m);
    
    let mut total = 0;
    for region in regions {
        println!("{:?}", region);
        total += price(&region.1);
    }

    total
}

fn price(region: &HashSet<Lattice>) -> usize {
    let area = region.len();
    area
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

    println!("Result: {}", res);
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
