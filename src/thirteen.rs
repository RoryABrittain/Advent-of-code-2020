use std::fs;
    

pub fn run() {

    let test_data = "939\r\n7,13,x,x,59,x,31,19";
    
    assert_eq!(first_bus(&test_data), 295);
    
    let raw_data = fs::read_to_string("thirteen_data.txt")
                .expect("Can't find file");
    println!("{}", first_bus(&raw_data));
    
    assert_eq!(time_for_all_busses(&test_data), 1068781);
    assert_eq!(time_for_all_busses(&" \r\n17,x,13,19"), 3417);
    assert_eq!(time_for_all_busses(&" \r\n67,7,59,61"), 754018);
    assert_eq!(time_for_all_busses(&" \r\n67,x,7,59,61"), 779210);
    assert_eq!(time_for_all_busses(&" \r\n67,7,x,59,61"), 1261476);
    assert_eq!(time_for_all_busses(&" \r\n1789,37,47,1889"), 1202161486);
    
    println!("{}", time_for_all_busses(&raw_data));


}

fn first_bus(input: &str) -> u32 {

    let lines: Vec<&str> = input.split("\r\n").collect();
    
    let start_time = lines[0].parse::<u32>().unwrap();
    
    let mut busses = lines[1].split(",").collect::<Vec<&str>>();
    busses.retain(|&x| x != "x");
    let bus_numbers = busses.iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    
    let mut time = start_time;
    
    let bus;
    
    'outer: loop {
        for i in &bus_numbers {
            if time % i == 0 {
                bus = i;
                break 'outer;
            }
        }
        
        time += 1;
        
    }
    
    bus * (time - start_time)
}


fn time_for_all_busses(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\r\n").collect();
    let split_second_line = lines[1].split(",").collect::<Vec<&str>>();
    
    let mut busses = Vec::new();
    
    for (i, x) in split_second_line.iter().enumerate() {
        if x != &"x" {
            busses.push((i, x.parse::<usize>().unwrap()));
        }
    }
    
    let mut busses_iter = busses.iter();
    

    let mut step = busses_iter.next().unwrap().1;
    
    let mut i = 0;
    
    for bus in busses_iter {
        
        loop {
            if (i + bus.0) % bus.1 == 0 {
                
                break;
            }
            
            i += step;
        }
        
        step *= bus.1;
    }
    i
}