use std::fs;
use std::env;
use std::str::FromStr;
use itertools::Itertools; // 0.8.2


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

fn check_pair_follows_rules(pair: (u32, u32), rules: &Vec<(u32, u32)>) -> bool {
    let mut found = false;
    rules.iter().for_each(|rule| {
        if rule == &pair {
            found = true;
        }
    });
    found
}

fn parse_input(input: &str) -> () {
    // parse 2 integers on each line

    let split: Vec<&str> = input.split("\n\n").collect();

    if split.len() < 2 {
        panic!("Input does not contain both rules and messages");
    }
    let rules_text = split[0];
    let rules: Vec<&str> = rules_text.split("\n").collect();
    println!("{:?}", rules);
    
    let mut the_rules: Vec<(u32, u32)> = Vec::new();
    
    rules.iter().for_each(|rule| {
        let rule_split: Vec<&str> = rule.split("|").collect();
        let rule_i = rule_split[0];
        let rule_j = rule_split[1];
        
        let i: u32 = FromStr::from_str(rule_i).unwrap();
        let j: u32 = FromStr::from_str(rule_j).unwrap();
        
        the_rules.push((i, j));
        
        println!("Rule: {} {}", i, j);
    });
    
    let messages_text = split[1];
    let messages: Vec<&str> = messages_text.split("\n").collect();
    println!("{:?}", messages);

    let mut pages: Vec<Vec<u32>> = Vec::new();
    
    messages.iter().for_each(|page_string| {
        println!("Message: {}", page_string);
        let mut page: Vec<u32> = Vec::new();
        
        page_string.split(",").for_each(|m| {
            let i = FromStr::from_str(m).unwrap();
            page.push(i);
        });
        pages.push(page);
    }); 

    println!("{:?}", pages);

    let mut total = 0;
    let mut invalid_total = 0;

    pages.iter_mut().for_each(|page| {
        println!("\nPAGE {:?}", page);

        let valid = check_page_follows_rules(page, &the_rules);

        if valid {
            let middlevalue = page.len() / 2;
            let middle = page[middlevalue];
            total += middle;
            println!("Valid page: {:?} +{}", page, middle);
        } else {
            
            println!("Page started as: {:?}", page);

            let mut now_valid = false;
            let mut counter = 0;

            while !now_valid {
                for i in 0..page.len() -1 {
                    let pair = (page[i], page[i+1]);
                    let found = check_pair_follows_rules(pair, &the_rules);
                    if !found {
                        // swap values
                        let temp = page[i];
                        page[i] = page[i+1];
                        page[i+1] = temp;
                        println!("Swapped {} and {}", page[i], page[i+1]);
                        println!("   Page is now: {:?}", page);
                    }
                }

                now_valid = check_page_follows_rules(page, &the_rules);
                counter += 1;
                if counter > 100 {
                    break;
                }
            }

            let end_page_valid = check_page_follows_rules(page, &the_rules);

            let middlevalue = page.len() / 2;
            let middle = page[middlevalue];
            invalid_total += middle;

            println!("Page ended as: {:?} which is {}", page, end_page_valid);

            /*
            // Memory intensive
            for perm in page.iter().permutations(page.len()).unique() {
                let mut perm_vec: Vec<u32> = perm.into_iter().copied().collect();
                let perm_is_valid = check_page_follows_rules(&mut perm_vec, &the_rules);
                if perm_is_valid {
                    println!("PERM VALID: {:?}", perm_vec);
                    invalid_total += perm_vec[perm_vec.len() / 2];
                    break;
                }
            }
            */



            println!("InValid page: {:?}", page);
        }

        println!("\n");

    });

    println!("Total: {}", total);
    println!("Invalid Total: {}", invalid_total);

    ()
}

fn check_page_follows_rules(page: &mut Vec<u32>, the_rules: &Vec<(u32, u32)>) -> bool {
    let mut valid = true;
    for i in 0..page.len() -1 {
    
        let pair = (page[i], page[i+1]);
        let found = check_pair_follows_rules(pair, &the_rules);
        valid = valid && found;

    }
    valid
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let input = parse_input(content.as_str());    

    println!("{:?}", input);

}