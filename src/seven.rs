use std::fs;
use regex::Regex;
 
#[derive(Debug)]
struct BagType<'a> {
    colour: &'a str,
    goes_in: Option<Vec<&'a str>>,
}

#[derive(Debug)]
    struct BagRule<'a> {
        colour: &'a str,
        contains: Option<Vec<(&'a str,u8)>>,
    }
    

pub fn run() {

    let raw_data = fs::read_to_string("seven_data.txt")
                .expect("Can't find file");
    

    ////////////////////////////////////////////////////////
    // First part
    //////////////////////////////////////////////////
    
    let re1 = Regex::new(r"(bags)|(bag)|[0-9\. ]").unwrap();
    let re2 = Regex::new(r"contain").unwrap();
    
    let cleaned_data = re1.replace_all(&raw_data, "");
    let cleaned_data = re2.replace_all(&cleaned_data, ",");
    

    let split_data: Vec<&str> = cleaned_data.split("\r\n").collect();
    
    
   
    let mut bags: Vec<BagType> = Vec::new();

    
    for i in &split_data {
        let line: Vec<&str> = i.split(",").collect();
        
        if line[1] == "noother" {
            bags.push(BagType {
                colour: line[0],
                goes_in: None,
            })
        } else {
            bags.push(BagType {
                colour: line[0],
                goes_in: Some(line[1..].to_vec()),
            })
        }
    }
    
    
    let first = "shinygold";
    
    let mut next_level = can_go_in(&bags, first);
    
    let mut external_bags: Vec<&str> = Vec::new();
    
    external_bags.extend(next_level.iter().cloned());
   
    
    while next_level.len() > 0 {
        
        let mut temp = Vec::new();
    
        for i in &next_level {

            temp.extend(can_go_in(&bags, i).iter().cloned());

        }
        
        next_level = temp;
        
        next_level.sort_unstable();
        next_level.dedup();
        
        external_bags.extend(next_level.iter().cloned());
    }
    
    external_bags.sort_unstable();
    external_bags.dedup();
    
    println!("{}", external_bags.len());
    
    ////////////////////////////////////////////////////////
    // Second part
    //////////////////////////////////////////////////

    let re_bags = Regex::new(r"( bags)|( bag)|\.").unwrap();
    
    let data_no_bags = re_bags.replace_all(&raw_data, "");
    
    
    let data_lines: Vec<&str> = data_no_bags.split("\r\n").collect();

    
    let seperator = Regex::new(r"( contain )|(, )").unwrap();
    
    let extract = Regex::new(r"(\d) ([a-z]+ [a-z]+)").unwrap();
    
    let mut rules: Vec<BagRule> = Vec::new();
       
    for line in &data_lines {
        
        let temp = seperator.split(line).collect::<Vec<&str>>();

        
        let rule = match temp[1] {
            "no other" => BagRule {
                colour: temp[0],
                contains: None,
            },
            _ => BagRule {
                colour: temp[0],
                contains: Some(temp[1..].iter().map(|x| (extract.captures(x).unwrap().get(2).unwrap().as_str(), extract.captures(x).unwrap().get(1).unwrap().as_str().parse::<u8>().unwrap())).collect::<Vec<(&str, u8)>>()),
            },
        };
        
        rules.push(rule);
        
    }
    
    
    
    println!("{}", number_inside(&rules, "shiny gold"));
    



}

// given a colour gives the colours of the bags that it goes in.
fn can_go_in<'a>(bags: &Vec<BagType<'a>>, colour: &str) -> Vec<&'a str> {
    let mut output: Vec<&str> = Vec::new();
    
    for i in bags {
        match &i.goes_in {
            Some(x) => if x.contains(&colour) {
                output.push(i.colour);
            },
            None => (),
        }
    }
    output
}

fn number_inside<'a>(rules: &Vec<BagRule<'a>>, colour: &str) -> u32 {
    
    let mut temp: &BagRule = &BagRule {
        colour: "",
        contains: None,
    };
    
    for rule in rules.iter() {
        if rule.colour == colour {
            temp = rule;
        }
    }
    
    let mut output = 0;
    
    match &temp.contains {
        Some(x) => for i in x {
            output = output + (i.1 as u32) * (1 + number_inside(rules, i.0));
        },
        None => (),
    }
    
    output
}