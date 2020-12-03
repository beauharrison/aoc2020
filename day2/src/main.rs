use std::{fmt::Display, fs};
use regex::Regex;

struct PasswordRecord {
    char: char,
    min: usize,
    max: usize,
    password: String
}

impl PasswordRecord {
    fn new(record_text: String) -> PasswordRecord {
        
        let re = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();

        if let Some(captures) = re.captures(&record_text) {
            return PasswordRecord {
                min: captures[1].parse::<usize>().unwrap(),
                max: captures[2].parse::<usize>().unwrap(),
                char: captures[3].parse::<char>().unwrap(),
                password: captures[4].to_string()
            };
        }

        panic!("Could not parse password record");
    }
}

impl Display for PasswordRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} {}: {}", self.min, self.max, self.char, self.password)
    }
}

fn part_1<T>(password_record: T) where T : Iterator<Item = PasswordRecord> {
    let mut valid_count = 0;

    for record in password_record {
        let rule_char_count = record.password.chars().filter(|c| *c == record.char).count();
        
        if rule_char_count >= record.min && rule_char_count <= record.max {
            valid_count += 1;
        }
    }

    println!("Part 1: count {}", valid_count);
}

fn part_2<T>(password_record: T) where T : Iterator<Item = PasswordRecord> {
    let mut valid_count = 0;

    for record in password_record {
        let password_chars = record.password.chars().collect::<Vec<char>>();

        if (password_chars[record.min-1] == record.char) ^ (password_chars[record.max-1] == record.char) {
            valid_count += 1;
        }
    }

    println!("Part 2: count {}", valid_count);
}

fn main() {
    let input = fs::read_to_string("..\\input\\day2.txt").expect("poop");

    let password_record_iter = input.lines()
        .map(|l| PasswordRecord::new(l.to_string()));

    
    part_1(password_record_iter.clone());
    part_2(password_record_iter.clone());
}
