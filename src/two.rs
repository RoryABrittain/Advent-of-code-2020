use std::fs;

pub fn run() {

    let raw_data = fs::read_to_string("two_data.txt")
                .expect("Can't find file");

    let split_data: Vec<&str> = raw_data.split("\r\n").collect();
    
    let mut number_correct1 = 0;
    
    for i in &split_data {
        if line_correct1(i) {
            number_correct1 = number_correct1 + 1;
        }
    }
    
    println!("{}", number_correct1);
    
    let mut number_correct2 = 0;
    
    for i in &split_data {
        if line_correct2(i) {
            number_correct2 = number_correct2 + 1;
        }
    }
    
    println!("{}", number_correct2);


}

fn line_correct1(line: &str) -> bool {
    let split_line: Vec<&str> = line.split(|c: char| !(c.is_alphanumeric())).filter(|s| !s.is_empty()).collect();
    
    let count = split_line[3].matches(split_line[2]).count();
    
    split_line[0].parse::<usize>().unwrap() <= count && count <= split_line[1].parse::<usize>().unwrap()
}

fn line_correct2(line: &str) -> bool {
    let split_line: Vec<&str> = line.split(|c: char| !(c.is_alphanumeric())).filter(|s| !s.is_empty()).collect();
    
    let index1 = split_line[0].parse::<usize>().unwrap() - 1;
    let index2 = split_line[1].parse::<usize>().unwrap() - 1;
    
    let position1: &str = &split_line[3][index1..(index1 + 1)];
    let position2: &str = &split_line[3][index2..(index2 + 1)];
    
    (position1 == split_line[2] || position2 == split_line[2]) && position1 != position2
}