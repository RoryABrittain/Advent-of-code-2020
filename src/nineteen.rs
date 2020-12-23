use regex::Captures;
use regex::Regex;
use std::fs;

pub fn run() {
    /////////////////////////////////////////////////////////////
    // tests
    /////////////////////////////////////////////////////////////

    let test_data1 = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: a
5: b

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    let test_data2 = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: a
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: b
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////

    assert_eq!(number_matches(&test_data1), 2);
    

    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////

    assert_eq!(number_matches(&test_data2), 3);
    
    // Changing 8: 42 to 8: 42 | 42 8 means that 8 now matches any
    // number (more than one) of 42
    let test_data2_update = test_data2.replace("8: 42", "8: (42)+");
    
    
    // Changing 11: 42 31 to 11: 42 31 | 42 11 31 means 11 matches
    // any number (more than one) of 42 and then the same number of
    // 31. I think this is impossible with a regular expression so
    // you have to capture it and check separatly.
    // In our data there is a fairly small number of repitiions so I have just added them manualy.
    let test_data2_update = test_data2_update.replace("11: 42 31", "11: 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31");
    
    assert_eq!(number_matches(&test_data2_update), 12);
    
    
    
    /////////////////////////////////////////////////////////////
    // Real data
    /////////////////////////////////////////////////////////////

    // Manually removed " from input file
    let raw_data = fs::read_to_string("nineteen_data.txt").expect("Can't find file");



    /////////////////////////////////////////////////////////////
    // First part
    /////////////////////////////////////////////////////////////

    println!("{}", number_matches(&raw_data));


    /////////////////////////////////////////////////////////////
    // Second part
    /////////////////////////////////////////////////////////////
    
    let raw_data_update = raw_data.replace("8: 42", "8: (42)+");
    

    // let raw_data_update = raw_data_update.replace("11: 42 31", "11: (?P<ft>42+) (?P<to>31+)");
    let raw_data_update = raw_data_update.replace("11: 42 31", "11: 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31");
    
    println!("{}", number_matches(&raw_data_update));

}

fn number_matches(input: &str) -> u32 {
    let split: Vec<_> = input.split("\n\n").collect();
    
    let rules: Vec<_> = split[0].split("\n").collect();
    let messages: Vec<_> = split[1].split("\n").collect();

 
    let mut split_rules: Vec<Vec<_>> = rules.iter().map(|x| x.split(": ").collect()).collect();
    
    
    let mut main_rule = "0".to_string();
    
    for rule in &split_rules {
        if rule[0] == "0" {
            main_rule = " ".to_string() + rule[1] + " ";
        }
    }
    
    
    split_rules.retain(|x| x[0] != "0");
    

    let rules_list: Vec<(_, String)> = split_rules.iter().map(|x| (
        Regex::new(&(r"(\D)(".to_owned() + x[0] + r")(\D)")).unwrap(),
        "((".to_string() + &x[1].replace(" | ", ")|(") + "))"
    )).collect();
    
    while main_rule.contains(|x: char| ('0'..='9').contains(&x)) {
        for rule in &rules_list {
            let temp = rule.0.replace_all(&main_rule, |caps: &Captures| {
                        format!(
                            "{}{}{}",
                            &caps[1], rule.1, &caps[3]
                        )
                    });
                    
            main_rule = temp.to_string();
            
        }
    }

    
    let temp = main_rule.replace(" ", "");
    
    main_rule = temp;
    
    
    while main_rule.contains("(a)") {
        let temp = main_rule.replace("(a)", "a");
    
        main_rule = temp;
    }
    
    while main_rule.contains("(b)") {
        let temp = main_rule.replace("(b)", "b");
    
        main_rule = temp;
    }
    
    
    let re = Regex::new(&("^".to_string() + &main_rule + "$")).unwrap();
    
    let mut result = 0;
    
    for message in &messages {
        if re.is_match(message) {
            result += 1;
        }
    }
    
    result
}