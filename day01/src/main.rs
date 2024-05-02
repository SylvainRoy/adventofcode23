use regex::Regex;
use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("./data/input.txt").expect("Failed to read input file");
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    // Part 1
    let mut result1: u64 = 0;
    for line in &lines {
        let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let first_digit = digits[0].to_digit(10).unwrap() as u64;
        let last_digit = digits[digits.len() - 1].to_digit(10).unwrap() as u64;
        let number = first_digit * 10 + last_digit;
        result1 += number;
    }
    println!("Part1: {}", result1);

    // Part 2
    let pattern = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let mut result2: u64 = 0;
    for line in &lines {
        let mut digits: String = String::new();
        for i in 0..line.len() {
            let nextchar = match pattern.captures(&line[i..]) {
                None => "",
                Some(capture) => match &capture[0] {
                    "one" | "1" => "1",
                    "two" | "2" => "2",
                    "three" | "3" => "3",
                    "four" | "4" => "4",
                    "five" | "5" => "5",
                    "six" | "6" => "6",
                    "seven" | "7" => "7",
                    "eight" | "8" => "8",
                    "nine" | "9" => "9",
                    _ => unreachable!(),
                },
            };
            digits += nextchar;
        }
        let first_digit = digits.chars().next().unwrap() as u64 - '0' as u64;
        let last_digit = digits.chars().last().unwrap() as u64 - '0' as u64;
        let number = first_digit * 10 + last_digit;
        result2 += number;
    }
    println!("Part2: {}", result2);
}
