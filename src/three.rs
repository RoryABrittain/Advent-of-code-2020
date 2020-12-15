use std::fs;

pub fn run() {

    let raw_data = fs::read_to_string("three_data.txt")
                .expect("Can't find file");

    let split_data: Vec<&str> = raw_data.split("\r\n").collect();
    
    
    println!("{}", number_of_trees(3, 1, &split_data));
    
    // It doesn't matter but this is a slow way of doing it.
    // I should loop through all of the lines once and count all
    // of the numbers simuiltainously.
    println!("{}", number_of_trees(1, 1, &split_data)
                   * number_of_trees(3, 1, &split_data)
                   * number_of_trees(5, 1, &split_data)
                   * number_of_trees(7, 1, &split_data)
                   * number_of_trees(1, 2, &split_data));
    

    


}

fn is_tree(position: usize, line: &str) -> bool {
    &line[position..(position + 1)] == "#"
}

fn number_of_trees(xstep: usize, ystep: usize, data: &Vec<&str>) -> u64 {
    
    let mut x = 0;
    let mut y = 0;
    
    let xmax = data[0].len();
    let ymax = data.len();
    
    let mut number = 0;
    
    loop {

        if is_tree(x, data[y]) {
            number = number + 1;
        }
            
        x = x + xstep;
            
        if x >= xmax {
            x = x - xmax;
        }

        
        y = y + ystep;
        
        if y >= ymax {
            break
        }
    }
    
    number
}