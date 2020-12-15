use std::fs;
    

pub fn run() {

    
    let raw_data = fs::read_to_string("twelve_data.txt")
                .expect("Can't find file");
    let lines: Vec<&str> = raw_data.split("\r\n").collect();
    
    let mut position = (0, 0);
    let mut direction = 1;
    
    
    
    for line in &lines {
        let chars: Vec<char> = line.chars().collect();
        let mut instruction = chars[0];
        let number_string: String = chars[1..].iter().collect();
        let number = number_string.parse::<i32>().unwrap();
        
        if instruction == 'F' {
            instruction = match direction{
                0 => 'N',
                1 => 'E',
                2 => 'S',
                3 => 'W',
                _ => panic!("Wrong direction,")
            };
        }
        
        match instruction {
            'N' => position.1 += number,
            'E' => position.0 += number,
            'S' => position.1 -= number,
            'W' => position.0 -= number,
            
            'L' => direction = (((direction - number / 90) % 4) + 4) % 4,
            
            'R' => direction = (((direction + number / 90) % 4) + 4) % 4,
            
            _ => panic!("Wrong instruction,"),
        }
    }
    
    println!("{}", position.0.abs() + position.1.abs());
    
    
    
    let mut position = (0, 0);
    let mut waypoint = (10, 1);
    
    for line in &lines {
        let chars: Vec<char> = line.chars().collect();
        let instruction = chars[0];
        let number_string: String = chars[1..].iter().collect();
        let number = number_string.parse::<i32>().unwrap();
        
        
        match instruction {
            'N' => waypoint.1 += number,
            'E' => waypoint.0 += number,
            'S' => waypoint.1 -= number,
            'W' => waypoint.0 -= number,
            
            'F' => position = (position.0 + number * waypoint.0, position.1 + number * waypoint.1),
            
            'L' => match number {
                90 => waypoint = (-waypoint.1, waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (waypoint.1, -waypoint.0),
                _ => panic!("Wrong angle,"),
            },
            
            'R' => match number {
                90 => waypoint = (waypoint.1, -waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (-waypoint.1, waypoint.0),
                _ => panic!("Wrong angle,"),
            },
            
            _ => panic!("Wrong instruction,"),
        }
    }
    
    println!("{}", position.0.abs() + position.1.abs());
}
