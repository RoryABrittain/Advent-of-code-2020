use std::fs;
    

pub fn run() {

    
    let raw_data = fs::read_to_string("ten_data.txt")
                .expect("Can't find file");
    let lines: Vec<&str> = raw_data.split("\r\n").collect();
    let mut numbers: Vec<i64> = lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    
    numbers.push(0); // power socket
    
    numbers.sort();
    
    // println!("{:?}", numbers);
    
    let mut differences: Vec<i64> = Vec::new();
    
    for (i, n) in numbers.iter().enumerate() {
        if i > 0 {
            differences.push(n - &numbers[i - 1]);
        }
    }
    
    differences.push(3); // final adaptor
    
    // println!("{:?}", differences);
    println!("{}\n", differences.iter().filter(|&x| *x == 1).count() * differences.iter().filter(|&x| *x == 3).count());
    
    
    let mut i = 0;
    
    let mut number_of_ways = 1;
    
    
    while i < numbers.len() {
        let mut temp: Vec<i64> = Vec::new();
        
        temp.push(numbers[i]);
        
        while i + 1 < numbers.len() && numbers[i + 1] - numbers[i] != 3 {
            i = i + 1;
            temp.push(numbers[i]);
        }


        // The data has no steps of 2 and at most 5 numbers in a row
        // separated by one. I just manualy worked out the number of
        // different ways for up to 5 becaseu it is not trivial what
        // the formula is.
        number_of_ways = number_of_ways * ([0, 1, 1, 2, 4, 7][temp.len()] as usize);
        i = i + 1;
    }
    
    println!("{}", number_of_ways);



}

