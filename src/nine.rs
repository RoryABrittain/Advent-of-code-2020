use std::fs;
    

pub fn run() {

    
    let raw_data = fs::read_to_string("nine_data.txt")
                .expect("Can't find file");
    let lines: Vec<&str> = raw_data.split("\r\n").collect();
    let numbers: Vec<i64> = lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    
    let length = 25;
    
    let mut invalid_number = 0;
    
    for (i, &n) in numbers.iter().enumerate() {
        if i >= length {
        
            if !is_sum(n, &numbers[(i - length)..i]) {
                invalid_number = n;                       
            }
        }
    }
    
    println!("{}", invalid_number);
    
    let mut start_index = 0;
    
    let start;
    let end;
    
    
    'outer: loop {
        
        let mut sum = 0;
        let mut end_index = start_index;
        
        while sum < invalid_number {
            sum = sum + numbers[end_index];
            
            if sum == invalid_number {
                start = start_index;
                end = end_index;
                break 'outer;
            }
            
            end_index = end_index + 1;
            
        }
        
        start_index = start_index + 1;
    }
    
    
    println!("{}", numbers[start..=end].iter().min().unwrap() + numbers[start..=end].iter().max().unwrap());


}

fn is_sum(n: i64, numbers: &[i64]) -> bool {
    let differences: Vec<i64> = numbers.iter().map(|x| n - x).collect();
    let mut output = false;
    for i in &differences {
        if 2 * i != n && numbers.contains(i) {
            output = true;
            break;
        }
    }
    
    output
}