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

                Rule::new(outer_caps[1].to_string(), can_contain)
            })
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .map(|r| (r.bag_colour.clone(), r))
        .collect::<HashMap<_, _>>()
}

fn rule_resolves_to_shiny_gold<'a>(rule: &'a Rule, rules: &'a HashMap<String, Rule>, known_to_resolve: &mut HashSet<&'a String>) -> bool {
    if known_to_resolve.contains(&rule.bag_colour) {
        true
    } else if rule.can_contain_shiny_gold() {
        known_to_resolve.insert(&rule.bag_colour);
        true
    } else {
        rule.can_contain.iter().any(|cc| {
            known_to_resolve.contains(&cc.0) || rules.contains_key(&cc.0) && rule_resolves_to_shiny_gold(&rules[&cc.0], rules, known_to_resolve)
        })
    }
}

fn rule_resolves_to_contribute<'a>(rule: &'a Rule, rules: &'a HashMap<String, Rule>, known_contribution: &mut HashMap<&'a String, i32>) -> i32 {
    if let Some(contribution) = known_contribution.get(&rule.bag_colour) {
        *contribution
    } else {
        let contribution = rule.can_contain.iter()
            .map(|cc| {
                let r = rules.get(&cc.0).unwrap();
                let contribution = rule_resolves_to_contribute(r, rules, known_contribution);
                cc.1 + contribution * cc.1
            })
            .sum();
        
        known_contribution.insert(&rule.bag_colour, contribution);

        contribution
    }
}

fn part_1(rules: &HashMap<String, Rule>) {
    let mut count = 0;

    let mut known_to_resolve = HashSet::<&String>::new();

    for rule in rules.values() {
        if rule_resolves_to_shiny_gold(rule, rules, &mut known_to_resolve) {
            count += 1;
        }
    }

    println!("part 1 count: {}", count);
}

fn part_2(rules: &HashMap<String, Rule>) {
    let mut known_contribution = HashMap::<&String, i32>::new();

    let shiny_gold = "shiny gold".to_string();
    let shiny_gold_rule = rules.get(&shiny_gold).unwrap();
    
    let total = rule_resolves_to_contribute(shiny_gold_rule, rules, &mut known_contribution);


    println!("part 2 count: {}", total);
}

fn main() {
    let input = fs::read_to_string("..\\input\\day7.txt").expect("poop");
    let rules = parse_rules(input);

    part_1(&rules);
    part_2(&rules);
}
