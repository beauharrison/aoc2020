use std::{collections::{HashSet, hash_map::RandomState}, fs};
use std::iter::FromIterator;

fn part_1(input: &String) {
    let mut total_count = 0;
    let mut group_answers = HashSet::<char>::new();

    let mut lines_iter = input.lines();
    
    for line in input.lines() {
        if line == "" {
            total_count += group_answers.len();
            group_answers.clear();
        } else {
            for c in line.chars() {
                group_answers.insert(c);
            }
        }
    }
    
    total_count += group_answers.len();

    println!("part 1 count: {}", total_count);
}

fn part_2(input: &String) {
    let mut total_count = 0;
    let mut group_set = Option::<HashSet<char>>::None;

    for line in input.lines() {
        println!("{:?}", group_set);
        if line == "" {
            if let Some(gs) = group_set {
                total_count += gs.len();
            }

            group_set = None;
        } else {
            if let Some(gs) = group_set {
                let current_set = HashSet::<char>::from_iter(line.chars());
                let union = gs.intersection(&current_set);
                group_set = Some(union.map(|c| *c).collect::<HashSet<char>>());
            } else {
                group_set = Some(HashSet::<char>::from_iter(line.chars()));
            }            
        }
    }
    
    if let Some(gs) = group_set {
        total_count += gs.len();
    }

    println!("part 2 count: {}", total_count);
}

fn main() {   
    let input = fs::read_to_string("..\\input\\day6.txt").expect("poop");

    part_1(&input);
    part_2(&input);
}
