use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let test_data1 =
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\r\nmem[8] = 11\r\nmem[7] = 101\r\nmem[8] = 0";

    assert_eq!(memory_total1(&test_data1), 165);

    let raw_data = fs::read_to_string("fourteen_data.txt").expect("Can't find file");

    println!("{}", memory_total1(&raw_data));


    let test_data2 = "mask = 000000000000000000000000000000X1001X\r\nmem[42] = 100\r\nmask = 00000000000000000000000000000000X0XX\r\nmem[26] = 1";

    assert_eq!(memory_total2(&test_data2), 208);
    
    println!("{}", memory_total2(&raw_data));
}

fn memory_total1(input: &str) -> u64 {
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut memory = HashMap::new();

    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    let re = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();

    for line in &lines {
        if &line[0..4] == "mask" {
            mask = &line[7..];
        } else {
            let numbers = re.captures(line).unwrap();
            memory.insert(
                numbers[1].parse::<u64>().unwrap(),
                masked(mask, numbers[2].parse::<u64>().unwrap()),
            );
        }
    }

    let mut total = 0;

    for (_, value) in &memory {
        total += value;
    }

    total
}

fn masked(mask: &str, n: u64) -> u64 {
    let mut output = n;

    for (i, digit) in mask.chars().enumerate() {
        match digit {
            '0' => output = output & !(1 << (35 - i)),
            '1' => output = output | (1 << (35 - i)), // 36 bit mask hardcoded,
            _ => (),
        }
    }
    output
}
fn decode_memory_address(code: &str, address: u64) -> Vec<u64> {
    let mut addresses = vec![address];

    for (i, digit) in code.chars().enumerate() {
        match digit {
            '1' => addresses[0] = addresses[0] | (1 << (35 - i)), // 36 bit code hardcoded,
            _ => (),
        }
    }

    for (i, digit) in code.chars().enumerate() {
        match digit {
            'X' => {
                let temp: Vec<u64> = addresses.iter().map(|x| x ^ (1 << (35 - i))).collect();
                addresses.extend(temp.iter().cloned());
            }
            _ => (),
        }
    }

    addresses
}

fn memory_total2(input: &str) -> u64 {
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut memory = HashMap::new();

    let mut code = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    let re = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();

    for line in &lines {
        if &line[0..4] == "mask" {
            code = &line[7..];
        } else {
            let numbers = re.captures(line).unwrap();
            let locations = decode_memory_address(&code, numbers[1].parse::<u64>().unwrap());
            for i in &locations {
                memory.insert(*i, numbers[2].parse::<u64>().unwrap());
            }
        }
    }

    let mut total = 0;
    

    for (_, value) in &memory {
        total += value;
    }

    total
}
