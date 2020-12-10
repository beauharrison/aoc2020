use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
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

    fn can_contain_shiny_gold(&self) -> bool {
        self.can_contain.iter().filter(|cc| cc.0 == "shiny gold").count() != 0
    }
}

fn parse_rules(input: String) -> HashMap<String, Rule> {
    let lines = input.lines();

    let outer_re = Regex::new(r"^([a-z ]+?) bags contain (.*)\.$").unwrap();
    let inner_re = Regex::new(r"(?:([0-9]+) ([a-z ]*)) bag").unwrap();

    lines
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

                // println!("{}", can_contain_string);

                Rule::new(outer_caps[1].to_string(), can_contain)
            })
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .map(|r| (r.bag_colour.clone(), r))
        .collect::<HashMap<_, _>>()
}

fn rule_resolves_to_shiny_gold<'a>(rule: &'a Rule, rules: &'a HashMap<String, Rule>, known_to_resolve: &mut HashSet<&'a String>, depth: i32) -> bool {
    if known_to_resolve.contains(&rule.bag_colour) {
        true
    } else if rule.can_contain_shiny_gold() {
        known_to_resolve.insert(&rule.bag_colour);
        true
    } else {
        rule.can_contain.iter().any(|cc| {
            known_to_resolve.contains(&cc.0) || rules.contains_key(&cc.0) && rule_resolves_to_shiny_gold(&rules[&cc.0], rules, known_to_resolve, depth + 1)
        })
    }
}

fn part_1(rules: &HashMap<String, Rule>) {
    let mut count = 0;

    let mut known_to_resolve = HashSet::<&String>::new();

    for rule in rules.values() {
        if rule_resolves_to_shiny_gold(rule, rules, &mut known_to_resolve, 0) {
            count += 1;
        }
    }

    println!("part 1 count: {}", count);
}

fn part_2(rules: &HashMap<String, Rule>) {
    let mut count = 0;
    let mut to_check_queue = VecDeque::<&String>::new();

    let a = "shiny gold".to_string();
    to_check_queue.push_back(&a);

    while to_check_queue.len() > 0 {
        let to_check_colour = to_check_queue.pop_front().unwrap();
        
        if let Some(check_rule) = rules.get(to_check_colour) {
            for can_contain in check_rule.can_contain.iter() {
                count += can_contain.1;
                println!("adding {} for {}, parent {} (total: {})", can_contain.1, can_contain.0, check_rule.bag_colour, count);
                to_check_queue.push_back(&can_contain.0);
            }
        }
    }

    println!("part 2 count: {}", count);
}

fn main() {
    let input = fs::read_to_string("..\\input\\day7.txt").expect("poop");
    let rules = parse_rules(input);

    part_1(&rules);
    part_2(&rules);
}
