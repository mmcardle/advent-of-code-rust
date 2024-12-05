use std::fs;
use std::env;
use std::str::FromStr;


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
        println!("{:?}", page);

        let mut valid = true;
        
        for i in 0..page.len() -1 {
            
            let pair = (page[i], page[i+1]);

            // find pair n rules
            let mut found = false;
            the_rules.iter().for_each(|rule| {
                if rule == &pair {
                    found = true;
                }
            });

            valid = valid && found;

            if !found {
            } else {
                // println!("    Valid pair: {:?}", pair);
            }
        }

        if valid {
            let middlevalue = page.len() / 2;
            let middle = page[middlevalue];
            total += middle;
            println!("Valid page: {:?} +{}", page, middle);
        } else {

            // find the first permutation of this page that passes all the rules

            let middlevalue = page.len() / 2;
            let middle = page[middlevalue];
            invalid_total += middle;
            println!("InValid page: {:?} +{}", page, middle);
        }

    });

    println!("Total: {}", total);
    println!("Invalid Total: {}", invalid_total);

    ()
}

fn main() {
    let content = read_file(get_filename_from_args().as_str());

    let input = parse_input(content.as_str());    

    println!("{:?}", input);

}