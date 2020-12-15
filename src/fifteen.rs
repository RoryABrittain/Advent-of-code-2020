use std::collections::HashMap;

pub fn run() {
    
    assert_eq!(series(vec![0, 3, 6], 2020), 436);
    
    println!("{}", series(vec![5, 1, 9, 18, 13, 8, 0], 2020));
    
    assert_eq!(series(vec![0, 3, 6], 30000000), 175594);
    
    println!("{}", series(vec![5, 1, 9, 18, 13, 8, 0], 30000000));
    



}

fn series(mut initial: Vec<u64>, nth: u64) -> u64 {
    let mut last_indices = HashMap::new();
    
    let mut n = initial.pop().unwrap();
    
    for (i, &x) in initial.iter().enumerate() {
        last_indices.insert(x, i as u64);
    }
    
    let mut next;
    
    for index in (initial.len() as u64)..(nth - 1) {
        // if index % 100_000 == 0 {println!("{}", index);}
    
        match last_indices.get(&n) {
            Some(x) => next = index - x,
            None => next = 0,
        }
        
        last_indices.insert(n, index);
        n = next;
        
    }
    
    n
}
