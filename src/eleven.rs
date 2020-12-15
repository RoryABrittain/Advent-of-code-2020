use std::fs;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Space {
    Empty,
    Occupied,
    Floor,
}
    

pub fn run() {

    
    let raw_data = fs::read_to_string("eleven_data.txt")
                .expect("Can't find file");
    let lines: Vec<&str> = raw_data.split("\r\n").collect();
    
    let initial_layout: Vec<Vec<Space>> = lines.iter().map(|x| x.chars().collect::<Vec<char>>().iter().map(|y| match y {
        'L' => Space::Empty,
        '#' => Space::Occupied,
        '.' => Space::Floor,
        _ => panic!("Invalid input"),
    }).collect()).collect();
    
    /////////////////////////////////////////////////////
    // Part 1
    ////////////////////////////////////////////////////
    
    let mut layout = initial_layout.clone();
    
    loop {
        
        let temp = layout.clone();
        layout = update(&layout);
        
        if layout == temp {
            break
        }
    }
    
    println!("{}", count_occupied(&layout));
    
    /////////////////////////////////////////////////////
    // Part 2
    ////////////////////////////////////////////////////
    
    let mut layout = initial_layout.clone();
    
    loop {
        // print_layout(&layout);
        // println!("");
        
        let temp = layout.clone();
        layout = update2(&layout);
        
        if layout == temp {
            break
        }
    }
    
    println!("{}", count_occupied(&layout));


    
}

fn adjacent((x, y): (usize, usize), width: usize, height: usize) -> Vec<(usize,usize)> {
    
    let mut output = Vec::new();
    
    if x > 0 {
        output.push((x - 1, y));
        
        if y > 0 {
            output.push((x - 1, y - 1));
        }
        
        if y < height - 1 {
            output.push((x - 1, y + 1));
        }
    }
    
    if x < width - 1 {
        output.push((x + 1, y));
        
        if y > 0 {
            output.push((x + 1, y - 1));
        }
        
        if y < height - 1 {
            output.push((x + 1, y + 1));
        }
    }
    
    if y > 0 {
        output.push((x, y - 1));
    }
        
    if y < height - 1 {
        output.push((x, y + 1));
    }
    
    output
}

fn update(initial: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    
    let mut output = Vec::new();
    
    let width = initial[0].len();
    let height = initial.len();
    
    for y in 0..height {
        let mut row = Vec::new();
        
        for x in 0..width {
            match initial[y][x] {
                Space::Empty => {
                    let mut temp = adjacent((x, y), width, height);
                    temp.retain(|x| match &initial[x.1][x.0] {
                        Space::Occupied => true,
                        _ => false,
                    });
                    if temp.len() > 0 {
                        row.push(Space::Empty);
                    } else {
                        row.push(Space::Occupied);
                    }
                },
                Space::Occupied => {
                    let mut temp = adjacent((x, y), width, height);
                    temp.retain(|x| match &initial[x.1][x.0] {
                        Space::Occupied => true,
                        _ => false,
                    });
                    if temp.len() >= 4 {
                        row.push(Space::Empty);
                    } else {
                        row.push(Space::Occupied);
                    }
                },
                Space::Floor => row.push(Space::Floor),
            }
        }
        
        output.push(row);
    }
    
    output
}

fn print_layout(layout: &Vec<Vec<Space>>) {
    for row in layout {
        let row_chars = row.iter().map(|x| match x {
            Space::Empty => 'L',
            Space::Occupied => '#',
            Space::Floor => '.',
        }).collect::<Vec<char>>();
        let row_string: String = row_chars.iter().collect();
        println!("{}", row_string);
    }
}

fn count_occupied(layout: &Vec<Vec<Space>>) -> u64 {
    
    let mut count = 0;
    
    for row in layout {
        for i in row {
            match i {
                Space::Empty => (),
                Space::Occupied => count += 1,
                Space::Floor => (),
            }
        }
    }
    count
}

fn update2(initial: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    
    let mut output = Vec::new();
    
    let width = initial[0].len();
    let height = initial.len();
    
    for y in 0..height {
        let mut row = Vec::new();
        
        for x in 0..width {
            match initial[y][x] {
                Space::Empty => {
                    let mut temp = visible(initial, (x, y), width, height);
                    temp.retain(|x| match &initial[x.1][x.0] {
                        Space::Occupied => true,
                        _ => false,
                    });
                    if temp.len() > 0 {
                        row.push(Space::Empty);
                    } else {
                        row.push(Space::Occupied);
                    }
                },
                Space::Occupied => {
                    let mut temp = visible(initial, (x, y), width, height);
                    temp.retain(|x| match &initial[x.1][x.0] {
                        Space::Occupied => true,
                        _ => false,
                    });
                    if temp.len() >= 5 {
                        row.push(Space::Empty);
                    } else {
                        row.push(Space::Occupied);
                    }
                },
                Space::Floor => row.push(Space::Floor),
            }
        }
        
        output.push(row);
    }
    
    output
}

fn visible(layout: &Vec<Vec<Space>>, (x, y): (usize, usize), width: usize, height: usize) -> Vec<(usize,usize)> {
    
    let mut output = Vec::new();
    
    let mut i;
    let mut j;
    
    // -1 -1 direction
    if x > 0 && y > 0 {
        i = x - 1;
        j = y - 1;
        
        loop {
            if layout[j][i] != Space::Floor {
                output.push((i, j));
                break;
            }
            if i > 0 && j > 0 {
                j -= 1;
                i -= 1;
            } else {
                break;
            }
        }
    }
    
    // -1 0 direction
    if x > 0 {
        i = x - 1;
        j = y;
        
        loop {
            if layout[j][i] != Space::Floor {
                output.push((i, j));
                break;
            }
            if i > 0 {
                i -= 1;
            } else {
                break;
            }
        }
    }
    
    // -1 +1 direction
    if x > 0 {
        i = x - 1;
        j = y + 1;
        
        while j < height {
            if layout[j][i] != Space::Floor {
                output.push((i, j));
                break;
            }
            if i > 0 {
                i -= 1;
                j += 1;
            } else {
                break;
            }
        }
    }
    
    // 0 -1 direction
    if y > 0 {
        i = x;
        j = y - 1;
        
        loop {
            if layout[j][i] != Space::Floor {
                output.push((i, j));
                break;
            }
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
    }
    
    // 0 +1 direction
    i = x;
    j = y + 1;
    
    while j < height {
        if layout[j][i] != Space::Floor {
            output.push((i, j));
            break;
        }
        j += 1;
    }
    
    // +1 -1 direction
    if y > 0 {
        i = x + 1;
        j = y - 1;
        
        while i < width {
            if layout[j][i] != Space::Floor {
                output.push((i, j));
                break;
            }
            if j > 0 {
                j -= 1;
                i += 1;
            } else {
                break;
            }
        }
    }
    
    // +1 0 direction
    i = x + 1;
    j = y;
    
    while i < width {
        if layout[j][i] != Space::Floor {
            output.push((i, j));
            break;
        }
        i += 1;
    }
    
    // +1 +1 direction
    i = x + 1;
    j = y + 1;
    
    while i < width && j < height {
        if layout[j][i] != Space::Floor {
            output.push((i, j));
            break;
        }
        i += 1;
        j += 1;
    }
    
    
    
    output
}