use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    exp_year: Option<String>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            exp_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None
        }
    }
}

fn parse_passports(input: String) -> Vec<Passport> {
    let mut passports = Vec::<Passport>::new();
    let mut next_value_set = Vec::<&str>::new();

    for line in input.lines() {
        if line == "" {
            passports.push(parse_passport(&next_value_set));
            next_value_set.clear();
        } else {
            for value in line.split_whitespace() {
                next_value_set.push(value);
            }
        }
    }

    if next_value_set.len() > 0 {
        passports.push(parse_passport(&next_value_set));
    }

    passports
}

fn parse_passport(value_set: &Vec::<&str>) -> Passport {
    let mut passport = Passport::new();

    for value in value_set {
        let split_value = value.split(":").collect::<Vec<&str>>();

        match split_value[0] {
            "byr" => passport.birth_year = Some(split_value[1].to_string()),
            "iyr" => passport.issue_year = Some(split_value[1].to_string()),
            "eyr" => passport.exp_year = Some(split_value[1].to_string()),
            "hgt" => passport.height = Some(split_value[1].to_string()),
            "hcl" => passport.hair_colour = Some(split_value[1].to_string()),
            "ecl" => passport.eye_colour = Some(split_value[1].to_string()),
            "pid" => passport.passport_id = Some(split_value[1].to_string()),
            "cid" => passport.country_id = Some(split_value[1].to_string()),
            _ => panic!("Unexpected value: {value}")
        }
    }

    passport
}

fn has_all_required_fields(passport: &Passport) -> bool {
    passport.birth_year.is_some() &&
        passport.issue_year.is_some() &&
        passport.exp_year.is_some() &&
        passport.height.is_some() &&
        passport.hair_colour.is_some() &&
        passport.eye_colour.is_some() &&
        passport.passport_id.is_some()
}

fn is_all_data_valid(passport: &Passport) -> bool {
    if !has_all_required_fields(passport) {
        false
    } else {        
        validate_year(passport.birth_year.as_ref().unwrap(), 1920, 2002) &&
            validate_year(passport.issue_year.as_ref().unwrap(), 2010, 2020) &&
            validate_year(passport.exp_year.as_ref().unwrap(), 2020, 2030) &&
            validate_height(passport.height.as_ref().unwrap()) &&
            validate_hair(passport.hair_colour.as_ref().unwrap()) &&
            validate_eyes(passport.eye_colour.as_ref().unwrap()) &&
            validate_pid(passport.passport_id.as_ref().unwrap())
    }
}

fn validate_year(year_string: &String, min: i32, max: i32) -> bool {
    let parsed = year_string.parse::<i32>();

    if let Result::Ok(year) = parsed {
        year >= min && year <= max
    } else {
        false
    }
}

fn validate_height(height: &String) -> bool {
    let re = Regex::new(r"^([0-9]+)([a-z]+)$").unwrap();
    let caps_option = re.captures(height);

    if let Some(caps) = caps_option {
        let value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let value_type = caps.get(2).unwrap().as_str();

        return match value_type {
            "in" => value >= 59 && value <= 76,
            "cm" => value >= 150 && value <= 193,
            _ => {
                false
            }
        }    
    }

    false
}

fn validate_hair(hair: &String) -> bool {
    let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    re.is_match(hair)
}

fn validate_eyes(eyes: &String) -> bool {
    match eyes.as_str() {
       "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
       _ => false
    }
}

fn validate_pid(pid: &String) -> bool {
    pid.len() == 9 && pid.parse::<i32>().is_ok()
}

fn part_1(passports: &Vec<Passport>) {
    let valid_count = passports.iter().filter(|p| has_all_required_fields(p)).count();

    println!("Part 1 count: {}", valid_count);
}

fn part_2(passports: &Vec<Passport>) {
    let valid_count = passports.iter().filter(|p| is_all_data_valid(p)).count();

    println!("Part 2 count: {}", valid_count);
}

fn main() {
    let input = fs::read_to_string("..\\input\\day4.txt").expect("poop");
    let passports = parse_passports(input);

    part_1(&passports);
    part_2(&passports);
}
