use regex::Captures;
use regex::Regex;
use std::fs;

pub fn run() {
    /////////////////////////////////////////////////////////////
    // tests
    /////////////////////////////////////////////////////////////

    let test_data1 = "1 + 2 * 3 + 4 * 5 + 6";
    let test_data2 = "1 + (2 * 3) + (4 * (5 + 6))";
    let test_data3 = "2 * 3 + (4 * 5)";
    let test_data4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    let test_data5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    let test_data6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////

    assert_eq!(evaluate_in_order(&test_data1), 71);
    assert_eq!(evaluate_in_order(&test_data2), 51);
    assert_eq!(evaluate_in_order(&test_data3), 26);
    assert_eq!(evaluate_in_order(&test_data4), 437);
    assert_eq!(evaluate_in_order(&test_data5), 12240);
    assert_eq!(evaluate_in_order(&test_data6), 13632);

    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////

    assert_eq!(evaluate_sum_first(&test_data1.to_string()), 231);
    assert_eq!(evaluate_sum_first(&test_data2.to_string()), 51);
    assert_eq!(evaluate_sum_first(&test_data3.to_string()), 46);
    assert_eq!(evaluate_sum_first(&test_data4.to_string()), 1445);
    assert_eq!(evaluate_sum_first(&test_data5.to_string()), 669060);
    assert_eq!(evaluate_sum_first(&test_data6.to_string()), 23340);

    /////////////////////////////////////////////////////////////
    // Real data
    /////////////////////////////////////////////////////////////

    let raw_data = fs::read_to_string("eighteen_data.txt").expect("Can't find file");

    let data_split = raw_data.split("\r\n").collect::<Vec<&str>>();

    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////

    let evaluations = data_split
        .iter()
        .map(|x| evaluate_in_order(x))
        .collect::<Vec<i64>>();

    println!("{}", evaluations.iter().sum::<i64>());

    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////
    
    let evaluations = data_split
        .iter()
        .map(|x| evaluate_sum_first(&x.to_string()))
        .collect::<Vec<i64>>();

    println!("{}", evaluations.iter().sum::<i64>());
    
}

fn evaluate_in_order(input: &str) -> i64 {
    let input_no_whitespace = input.replace(" ", "");
    let characters: Vec<char> = input_no_whitespace.chars().collect();
    let mut char_iter = characters.iter();

    let mut result: Vec<i64> = match char_iter.next().unwrap() {
        x @ '0'..='9' => vec![x.to_digit(10).unwrap() as i64],
        '(' => vec![0, 0],
        _ => panic!("Wrong initial character"),
    };

    let mut operation = Vec::new();

    loop {
        match char_iter.next() {
            Some(x) => match x {
                '+' => operation.push('+'),
                '*' => operation.push('*'),
                '(' => {
                    result.push(0);
                    operation.push('+');
                }
                ')' => {
                    let temp = result.pop().unwrap();

                    match operation.pop() {
                        Some('+') => *result.last_mut().unwrap() += temp,
                        Some('*') => *result.last_mut().unwrap() *= temp,
                        Some(_) => panic!("Wrong operation"),
                        None => *result.last_mut().unwrap() += temp,
                    }
                }
                y @ '0'..='9' => match operation.pop() {
                    Some('+') => *result.last_mut().unwrap() += y.to_digit(10).unwrap() as i64,
                    Some('*') => *result.last_mut().unwrap() *= y.to_digit(10).unwrap() as i64,
                    Some(_) => panic!("Wrong operation"),
                    None => *result.last_mut().unwrap() += y.to_digit(10).unwrap() as i64,
                },
                _ => panic!("Wrong character"),
            },
            None => break,
        }
    }

    if result.len() > 1 {
        panic!("Result stack greater than 1 number");
    }

    result[0]
}

// fn evaluate_sum_first(input: &String) -> i64 {
// println!("{}", &input);

// let re_plus = Regex::new(r"(\d+) \+ (\d+)").unwrap();
// let re_times = Regex::new(r"(\d+) \* (\d+)").unwrap();
// let re_parens = Regex::new(r"\((\d+)\)").unwrap();

// let mut line = input.clone();

// println!("{}", &line);

// let mut result;

// loop {
// match line.parse::<i64>() {
// Ok(x) => {
// result = x;
// break;
// }
// Err(_) => {
// while re_plus.is_match(&line) {
// let temp = re_plus.replace_all(&line, |caps: &Captures| {
// format!(
// "{}",
// &caps[2].parse::<i64>().unwrap() + &caps[1].parse::<i64>().unwrap()
// )
// });

// line = temp.to_string();

// println!("{}", &line);
// }

// while re_times.is_match(&line) {
// let temp = re_times.replace_all(&line, |caps: &Captures| {
// format!(
// "{}",
// &caps[2].parse::<i64>().unwrap() * &caps[1].parse::<i64>().unwrap()
// )
// });

// line = temp.to_string();

// println!("{}", &line);
// }

// while re_parens.is_match(&line) {
// let temp = re_parens.replace_all(&line, "$1");

// line = temp.to_string();

// println!("{}", &line);
// }
// }
// }
// }

// println!("{}", &line);

// result
// }

fn evaluate_sum_first(input: &String) -> i64 {
    let re_plus = Regex::new(r"(\d+)\+(\d+)").unwrap();
    // let re_times = Regex::new(r"(\d+) \* (\d+)").unwrap();
    let re_times = Regex::new(r"\((\d+)\*(\d+)\)").unwrap();
    let re_times2 = Regex::new(r"^(\d+)\*(\d+)\*").unwrap();
    let re_times3 = Regex::new(r"\((\d+)\*(\d+)\*").unwrap();
    let re_times4 = Regex::new(r"^(\d+)\*(\d+)$").unwrap();
    let re_parens = Regex::new(r"\((\d+)\)").unwrap();

    let mut line = input.replace(" ", "");

    
    let result;

    loop {
        match line.parse::<i64>() {
            Ok(x) => {
                result = x;
                break;
            }
            Err(_) => {
                while re_plus.is_match(&line) {
                    let temp = re_plus.replace_all(&line, |caps: &Captures| {
                        format!(
                            "{}",
                            &caps[2].parse::<i64>().unwrap() + &caps[1].parse::<i64>().unwrap()
                        )
                    });

                    line = temp.to_string();

                }

                while re_times.is_match(&line) {
                    let temp = re_times.replace_all(&line, |caps: &Captures| {
                        format!(
                            "{}",
                            &caps[2].parse::<i64>().unwrap() * &caps[1].parse::<i64>().unwrap()
                        )
                    });

                    line = temp.to_string();

                }

                while re_times2.is_match(&line) {
                    let temp = re_times2.replace_all(&line, |caps: &Captures| {
                        format!(
                            "{}*",
                            &caps[2].parse::<i64>().unwrap() * &caps[1].parse::<i64>().unwrap()
                        )
                    });

                    line = temp.to_string();

                }

                while re_times3.is_match(&line) {
                    let temp = re_times3.replace_all(&line, |caps: &Captures| {
                        format!(
                            "({}*",
                            &caps[2].parse::<i64>().unwrap() * &caps[1].parse::<i64>().unwrap()
                        )
                    });

                    line = temp.to_string();

                }

                while re_times4.is_match(&line) {
                    let temp = re_times4.replace_all(&line, |caps: &Captures| {
                        format!(
                            "{}",
                            &caps[2].parse::<i64>().unwrap() * &caps[1].parse::<i64>().unwrap()
                        )
                    });

                    line = temp.to_string();

                }

                while re_parens.is_match(&line) {
                    let temp = re_parens.replace_all(&line, "$1");

                    line = temp.to_string();

                }
            }
        }
    }

    result
}
