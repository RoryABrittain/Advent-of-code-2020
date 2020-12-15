use std::fs;

pub fn run() {
    let raw_data = fs::read_to_string("one_data.txt")
                .expect("Can't find file");

    let split_data:Vec<&str> = raw_data.split("\r\n").collect();
    
    let mut integer_data: Vec<i32> = Vec::new();
    
    for i in &split_data {
        integer_data.push(i.parse().unwrap());
    }
    
    for i in &integer_data {
        for j in & integer_data {
            if i + j == 2020 {
                println!("{} {} {}", i, j, i * j);
            }
            
            for k in &integer_data {
                if i + j + k == 2020 {
                    println!("{} {} {} {}", i, j, k, i * j * k);
                }
            }
        }
    }

}