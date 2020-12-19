// use regex::Regex;
// use std::fs;

// #[derive(Debug)]
// struct Field {
// name: String,
// range1: (u32, u32),
// range2: (u32, u32),
// possible: Vec<usize>,
// position: Option<usize>,
// }

pub fn run() {
    /////////////////////////////////////////////////////////////
    // tests
    /////////////////////////////////////////////////////////////

    let test_data_txt = ".#.
..#
###";

    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////

    let mut test_grid3d = parse_input3d(&test_data_txt);

    for _i in 0..6 {
        test_grid3d = update3d(test_grid3d);
    }

    assert_eq!(count3d(&test_grid3d), 112);

    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////

    let mut test_grid4d = parse_input4d(&test_data_txt);



    for _i in 0..6 {
        test_grid4d = update4d(test_grid4d);
    }

    assert_eq!(count4d(&test_grid4d), 848);

    /////////////////////////////////////////////////////////////
    // Real data
    /////////////////////////////////////////////////////////////

    let raw_data = "####...#
......##
####..##
##......
..##.##.
#.##...#
....##.#
.##.#.#.";

    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////

    let mut grid3d = parse_input3d(&raw_data);

    for _i in 0..6 {
        grid3d = update3d(grid3d);
    }


    println!("{}", count3d(&grid3d));

    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////

    let mut grid4d = parse_input4d(&raw_data);

    for _i in 0..6 {
        grid4d = update4d(grid4d);
    }

    println!("{}", count4d(&grid4d));
}

fn parse_input3d(input: &str) -> [[[u8; 24]; 24]; 15] {
    let mut grid: [[[u8; 24]; 24]; 15] = [[[0; 24]; 24]; 15];

    for (j, row) in input.split('\n').enumerate() {
        for (i, cell) in row.chars().enumerate() {
            if cell == '#' {
                grid[7][j + 7][i + 7] = 1;
            }
        }
    }
    grid
}

fn parse_input4d(input: &str) -> [[[[u8; 24]; 24]; 15]; 15] {
    let mut grid: [[[[u8; 24]; 24]; 15]; 15] = [[[[0; 24]; 24]; 15]; 15];

    for (j, row) in input.split('\n').enumerate() {
        for (i, cell) in row.chars().enumerate() {
            if cell == '#' {
                grid[7][7][j + 7][i + 7] = 1;
            }
        }
    }
    grid
}

fn update3d(grid: [[[u8; 24]; 24]; 15]) -> [[[u8; 24]; 24]; 15] {
    let mut new_grid = grid;


        for k in 1..14 {
            for j in 1..23 {
                for i in 1..23 {
                    let mut neighbours = 0;


                        for l in 0..3 {
                            for m in 0..3 {
                                for n in 0..3 {
                                    if !(l == 1 && m == 1 && n == 1) {
                                        neighbours +=
                                            grid[k + l - 1][j + m - 1][i + n - 1];
                                    }
                                }
                            }
                        }
                    

                    if grid[k][j][i] == 1 {
                        if !(neighbours == 2 || neighbours == 3) {
                            new_grid[k][j][i] = 0;
                        }
                    } else {
                        if neighbours == 3 {
                            new_grid[k][j][i] = 1;
                        }
                    }
                }
            }
        }
    

    new_grid
}

fn count3d(grid: &[[[u8; 24]; 24]; 15]) -> u32 {
    let mut total = 0;

    for k in 0..15 {
        for j in 0..24 {
            for i in 0..24 {
                total += grid[k][j][i] as u32;
            }
        }
    }

    total
}

fn update4d(grid: [[[[u8; 24]; 24]; 15]; 15]) -> [[[[u8; 24]; 24]; 15]; 15] {
    let mut new_grid = grid;

    for p in 1..14 {
        for k in 1..14 {
            for j in 1..23 {
                for i in 1..23 {
                    let mut neighbours = 0;

                    for q in 0..3 {
                        for l in 0..3 {
                            for m in 0..3 {
                                for n in 0..3 {
                                    if !(q == 1 && l == 1 && m == 1 && n == 1) {
                                        neighbours +=
                                            grid[p + q - 1][k + l - 1][j + m - 1][i + n - 1];
                                    }
                                }
                            }
                        }
                    }

                    if grid[p][k][j][i] == 1 {
                        if !(neighbours == 2 || neighbours == 3) {
                            new_grid[p][k][j][i] = 0;
                        }
                    } else {
                        if neighbours == 3 {
                            new_grid[p][k][j][i] = 1;
                        }
                    }
                }
            }
        }
    }

    new_grid
}

fn count4d(grid: &[[[[u8; 24]; 24]; 15]; 15]) -> u32 {
    let mut total = 0;

    for l in 0..15 {
        for k in 0..15 {
            for j in 0..24 {
                for i in 0..24 {
                    total += grid[l][k][j][i] as u32;
                }
            }
        }
    }

    total
}
