 use std::fs;

pub fn run() {

    let raw_data = fs::read_to_string("five_data.txt")
                .expect("Can't find file");

    let split_data: Vec<&str> = raw_data.split("\r\n").collect();
    
    let mut ids: Vec<u32> = Vec::new();
    
    
    for i in &split_data {
        let temp = 8 * row(i) + column(i);
        
        ids.push(temp);
    }
    
    println!("{}", ids.iter().max().unwrap());
    
    ids.sort_unstable();
    
    let mut iterator1 = ids.iter();
    let mut iterator2 = ids.iter();
    
    iterator2.next();
    
    for i in iterator2 {
        let temp = iterator1.next().unwrap();
        
        if i - temp == 2 {
            println!("{}", temp + 1);
        }
    }


}

fn row(code: &str) -> u32 {
    u32::from_str_radix(&code[0..7].replace("B", "1").replace("F", "0"), 2).unwrap()
}

fn column(code: &str) -> u32 {
    u32::from_str_radix(&code[7..10].replace("R", "1").replace("L", "0"), 2).unwrap()
}

