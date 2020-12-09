use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Rule {
    bag_colour: String,
    can_contain: Vec<(String, i32)>
}

impl Rule {
    fn new(bag_colour: String, can_contain: Vec<(String, i32)>) -> Rule {
        Rule {
            bag_colour: bag_colour,
            can_contain: can_contain
        }
    }
}

fn parse_rules(input: String) {
    let lines = input.lines();

    let outer_re = Regex::new(r"^([a-z ]+?) bags contain (.*)\.$").unwrap();
    let inner_re = Regex::new(r"(?:([0-9]+) ([a-z ]*)) bag").unwrap();

    let a = lines
        .map(|line| {
            outer_re.captures(line).map(|outer_caps| {
                let can_contain_string = outer_caps[2].as_ref();

                let can_contain = inner_re.captures_iter(can_contain_string)
                    .map(|inner_caps| {
                        (
                            inner_caps[2].to_string(),
                            inner_caps[1].parse::<i32>().unwrap()
                        )                        
                    })
                    .collect::<Vec<_>>();

                println!("{}", can_contain_string);

                Rule::new(outer_caps[1].to_string(), can_contain)
            })
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap());

    for aa in a {
        println!("{:?}", aa);
    }

}

fn main() {
    let input = fs::read_to_string("..\\input\\day7.txt").expect("poop");
    parse_rules(input);
}
