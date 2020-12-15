 use std::fs;
 use regex::Regex;

pub fn run() {

    let raw_data = fs::read_to_string("four_data.txt")
                .expect("Can't find file");

    let split_data: Vec<&str> = raw_data.split("\r\n\r\n").collect();
    
    let mut number_with_fields = 0;
    let mut number_with_valid_fields = 0;
    
    for i in &split_data {
        if has_fields(i) {
            number_with_fields = number_with_fields + 1;
        }
        if valid_fields(i) {
            number_with_valid_fields = number_with_valid_fields + 1;
        }
    }
    
    println!("Number of passports with correct fields: {}", number_with_fields);
    println!("Number of passports with valid fields: {}", number_with_valid_fields);


}

fn has_fields(passport: &str) -> bool {
    passport.matches("byr").count() == 1
    && passport.matches("iyr").count() == 1
    && passport.matches("eyr").count() == 1
    && passport.matches("hgt").count() == 1
    && passport.matches("hcl").count() == 1
    && passport.matches("ecl").count() == 1
    && passport.matches("pid").count() == 1
}

fn valid_fields(passport: &str) -> bool {
    let birth = Regex::new(r"byr:(\d{4})").unwrap();
    let issue = Regex::new(r"iyr:(\d{4})").unwrap();
    let expiration = Regex::new(r"eyr:(\d{4})").unwrap();
    let height = Regex::new(r"hgt:((?P<cm>\d{3})cm)|((?P<in>\d{2})in)").unwrap();
    let hair = Regex::new(r"hcl:#[0-9a-f]{6}([ \r\n]|$)").unwrap();
    let eye = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)([ \r\n]|$)").unwrap();
    let number = Regex::new(r"pid:\d{9}([ \r\n]|$)").unwrap();
    
    true && match birth.captures(passport) {
        Some(x) => (1920 <= x[1].parse::<u32>().unwrap() && x[1].parse::<u32>().unwrap() <= 2002),
        None => false,
    }
    && match issue.captures(passport) {
        Some(x) => (2010 <= x[1].parse::<u32>().unwrap() && x[1].parse::<u32>().unwrap() <= 2020),
        None => false,
    }
    && match expiration.captures(passport) {
        Some(x) => (2020 <= x[1].parse::<u32>().unwrap() && x[1].parse::<u32>().unwrap() <= 2030),
        None => false,
    }
    && match height.captures(passport) {
        Some(x) => {
           true && match x.name("cm") {
                Some(y) => 150 <= y.as_str().parse::<u32>().unwrap() && y.as_str().parse::<u32>().unwrap() <= 193,
                None => false,
            }
            || match x.name("in") {
                Some(y) => 59 <= y.as_str().parse::<u32>().unwrap() && y.as_str().parse::<u32>().unwrap() <= 76,
                None => false,
            }
        },
        None => false,
    }
    && hair.is_match(passport)
    && eye.is_match(passport)
    && number.is_match(passport)
}