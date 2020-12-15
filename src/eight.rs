use std::fs;
    

pub fn run() {

    
    let raw_data = fs::read_to_string("eight_data.txt")
                .expect("Can't find file");
    let lines: Vec<&str> = raw_data.split("\r\n").collect();
    
    
    let split_lines: Vec<Vec<&str>> = lines.iter().map(|x| x.split(" ").collect()).collect();
    
    
    let instructions: Vec<(&str, i32)> = split_lines.iter().map(|x| (x[0], x[1].parse::<i32>().unwrap())).collect();
    

    let mut accumulator = 0;
    
    let mut index = 0;
    
    let mut indexes: Vec<usize> = Vec::new();
    
    loop {
        
        if !indexes.contains(&index) {
            indexes.push(index);
        } else {
            println!("{}", accumulator);
            break;
        }
        
        match instructions[index] {
            ("acc", x) => {
                accumulator = accumulator + x;
                index = index + 1
            },
            ("jmp", x) => index = ((index as i32) + x) as usize,
            ("nop", _x) => index = index + 1,
            _ => break,
        }
    }
    
    let halt = instructions.len();
    
    for (i, instruction) in instructions.iter().enumerate() {

        
        if instruction.0 == "acc" {
            continue;
        }
        
        let mut new_instructions = instructions.clone();
        new_instructions[i] = match instruction {
            ("jmp", x) => ("nop", *x),
            ("nop", x) => ("jmp", *x),
            _ => ("aa", 0), // This should never happen.
        };
        
        index = 0;
        accumulator = 0;
        indexes = Vec::new();
        
        loop {
        
            if index == halt {
                println!("{}", accumulator);
                break;
            } else if !indexes.contains(&index) {
                indexes.push(index);
            } else {
                break;
            }
        
            match new_instructions[index] {
                ("acc", x) => {
                    accumulator = accumulator + x;
                    index = index + 1
                },
                ("jmp", x) => index = ((index as i32) + x) as usize,
                ("nop", _x) => index = index + 1,
                _ => break,
            }
        }
    }



}