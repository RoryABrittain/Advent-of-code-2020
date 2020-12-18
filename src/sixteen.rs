use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Field {
    name: String,
    range1: (u32, u32),
    range2: (u32, u32),
    possible: Vec<usize>,
    position: Option<usize>,
}

pub fn run() {

    /////////////////////////////////////////////////////////////
    // tests
    /////////////////////////////////////////////////////////////
    
    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////
    
    let test_data_txt = "class: 1-3 or 5-7\r\nrow: 6-11 or 33-44\r\nseat: 13-40 or 45-50\r\n\r\nyour ticket:\r\n7,1,14\r\n\r\nnearby tickets:\r\n7,3,47\r\n40,4,50\r\n55,2,20\r\n38,6,12".to_string();

    
    let (test_fields, _test_my_ticket, test_nearby_tickets) = parse_input(&test_data_txt);
    
    assert_eq!(ticket_scanning_error_rate(&test_fields, &test_nearby_tickets), 71);
    
    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////
    
    let test_data_txt2 = "class: 0-1 or 4-19\r\nrow: 0-5 or 8-19\r\nseat: 0-13 or 16-19\r\n\r\nyour ticket:\r\n11,12,13\r\n\r\nnearby tickets:\r\n3,9,18\r\n15,1,5\r\n5,14,9".to_string();
    
    let (test_fields2, _test_my_ticket2, test_nearby_tickets2) = parse_input(&test_data_txt2);

    let test_valid_tickets2 = test_nearby_tickets2.iter().filter(|x| x.iter().all(|&y| valid_number(&test_fields, y))).collect::<Vec<_>>();
    

    let mut test_fields2 = test_fields2;
    
    for ticket in &test_valid_tickets2 {
        for (i, &n) in ticket.iter().enumerate() {
            for field in &mut test_fields2 {
                if !match_field(field, n) {
                    field.possible.retain(|&x| x != i);
                }
            }
        }
    }
    


    
    /////////////////////////////////////////////////////////////
    // Real data
    /////////////////////////////////////////////////////////////
    
    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////
    
    let raw_data = fs::read_to_string("sixteen_data.txt").expect("Can't find file");
    
    let (fields, my_ticket, nearby_tickets) = parse_input(&raw_data);
    
    println!("{}", ticket_scanning_error_rate(&fields, &nearby_tickets));
    
    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////
    
    let valid_tickets = nearby_tickets.iter().filter(|x| x.iter().all(|&y| valid_number(&fields, y))).collect::<Vec<_>>();
        

    let mut fields = fields;
    
    for ticket in &valid_tickets {
        for (i, &n) in ticket.iter().enumerate() {
            for field in &mut fields {
                if !match_field(field, n) {
                    field.possible.retain(|&x| x != i);
                }
            }
        }
    }
    


    
    let mut position = None;
    
    loop {
        for field in &mut fields {
            if field.possible.len() == 1 {
                position = Some(field.possible[0]);
                field.position = position;
                break;
                
            }
        }
        
        if fields.iter().all(|x| x.position != None) {
            break;
        }
        
        match position {
            Some(x) => for field in &mut fields {
                field.possible.retain(|&y| y != x);
            },
            None => (),
        }
        
        
    }
    
    
    let mut multiply_departures = 1;
    
    let departure_re = Regex::new(r"^departure").unwrap();
    
    for field in &fields {
        if departure_re.is_match(&field.name) {
            multiply_departures *= my_ticket[field.position.unwrap()];
        }
    }
    
    println!("{}", multiply_departures);


}

fn match_field(field: &Field, n: u32) -> bool {
    (field.range1.0 <= n && n <= field.range1.1) || (field.range2.0 <= n && n <= field.range2.1)
}

fn valid_number(fields: &Vec<Field>, n: u32) -> bool {
    fields.iter().any(|x| match_field(x, n))
}

fn parse_input(input: &String) -> (Vec<Field>, Vec<u64>, Vec<Vec<u32>>) {
    let split_input_re = Regex::new(r"\r\n\r\nyour ticket:\r\n|\r\n\r\nnearby tickets:\r\n")
        .unwrap();
    let mut split_input = split_input_re.split(input);
    
    let fields_text = split_input.next().unwrap();
    let my_ticket_txt = split_input.next().unwrap();
    let nearby_tickets_txt = split_input.next().unwrap();
    
    let my_ticket = my_ticket_txt.split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
            
    let num_fields = my_ticket.len();
    
    let mut fields: Vec<Field> = Vec::new();
    
    for i in fields_text.split("\r\n") {
        let parsed = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap().captures(i).unwrap();
        
        fields.push(Field {
            name: parsed[1].to_string(),
            range1: (parsed[2].parse().unwrap(), parsed[3].parse().unwrap()),
            range2: (parsed[4].parse().unwrap(), parsed[5].parse().unwrap()),
            possible : (0..num_fields).collect(),
            position: None,
        });
        
    }
    
    
    (
        fields,
        my_ticket,
        nearby_tickets_txt.split("\r\n")
        .map(|x| x.split(',')
            .map(|y| y.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
    )
}

fn ticket_scanning_error_rate(fields: &Vec<Field>, nearby_tickets: &Vec<Vec<u32>>) -> u32 {
    let mut error_rate = 0;
    
    for i in nearby_tickets {
        for &j in i {
            if !valid_number(fields, j) {
                error_rate += j;
            }
        }
    }
    
    error_rate
}
