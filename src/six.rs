 use std::fs;

pub fn run() {

    let raw_data = fs::read_to_string("six_data.txt")
                .expect("Can't find file");

    let split_data: Vec<&str> = raw_data.split("\r\n\r\n").collect();
    
    
    let mut total1 = 0;
    
    for i in &split_data {
        let mut letters: Vec<char> = i.chars().collect();
        letters.retain(|&x| x != '\n' && x != '\r');
        letters.sort_unstable();
        letters.dedup();
        
        total1 = total1 + letters.len();
    }
    
    println!("{}", total1);

    
    let mut total2 = 0;
    
    for &i in &split_data {
        
        let lines: Vec<&str> = i.split("\r\n").collect();
        
        let mut iterator = lines.iter();
        
        let mut first_line: Vec<char>  = iterator.next().unwrap().chars().collect(); 
        
        first_line.sort_unstable();
        
        for j in iterator {
            let mut line: Vec<char>  = j.chars().collect();
            line.sort_unstable();
            
            first_line = intersection_sorted_vec(first_line, line);
        }
        
        
        total2 = total2 + first_line.len();
    }
    
    println!("{}", total2);
    
    
    


}

fn intersection_sorted_vec(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    let mut intersection: Vec<char> = Vec::new();
    let mut b_iter = b.iter();
    if let Some(mut current_b) = b_iter.next() {
        for current_a in a {
            while current_b < &current_a {
                current_b = match b_iter.next() {
                    Some(current_b) => current_b,
                    None => return intersection,
                };
            }
            if &current_a == current_b {
                intersection.push(current_a);
            }
        }
    }
    intersection
}


