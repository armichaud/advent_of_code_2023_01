use std::fs::read_to_string;
use std::collections::HashMap;

fn solve(filename: &str, convert_digit_words: bool) -> i32 {
    let digits: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);
    read_to_string(filename).unwrap()
        .lines()
        .map( |line| {
            let mut line = line.to_string();
            if convert_digit_words {
                for (word, digit) in digits.iter() {
                    line = line.replace(word, digit);
                }
            }
            let digits = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            let mut n = digits.first().unwrap().to_string();
            n.push_str(digits.last().unwrap().to_string().as_str());
            n.parse::<i32>().unwrap()
        }).sum()
}

fn main() {
    assert_eq!(solve("example_1.txt", false), 142);
    assert_eq!(solve("input.txt", false), 54331);
    assert_eq!(solve("example_2.txt", true), 281);
    assert_eq!(solve("input.txt", true), 54518);
}
