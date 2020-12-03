use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

const TARGET_VALUE: i32 = 2020;

fn part_1(values_hash_set: &HashSet<i32>) {
    for value in values_hash_set {
        let diff = TARGET_VALUE - *value;
        
        if values_hash_set.contains(&diff) {
            println!("part 1:");
            println!("\tvalue 1: {}", *value);
            println!("\tvalue 2: {}", diff);
            println!("\tans: {}", value * diff);
            break;
        }
    }
}

fn part_2(values_hash_set: &HashSet<i32>) {
    'top: for value_1 in values_hash_set {
        let value_1_diff = TARGET_VALUE - *value_1;

        for value_2 in values_hash_set {
            if *value_2 <= value_1_diff {
                let value_2_diff = value_1_diff - *value_2;
                
                if values_hash_set.contains(&value_2_diff) {
                    println!("part 2:");
                    println!("\tvalue 1: {}", *value_1);
                    println!("\tvalue 2: {}", *value_2);
                    println!("\tvalue 3: {}", value_2_diff);
                    println!("\tans: {}", value_1 * value_2 * value_2_diff);
                    break 'top;
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("..\\input\\day1.txt").expect("poop");

    let input_integers_iter = input.lines()
        .map(|l| l.parse::<i32>())
        .filter(|p| p.is_ok())
        .map(|p| p.unwrap());

    let values_hash_set = HashSet::<i32>::from_iter(input_integers_iter);
    
    part_1(&values_hash_set);
    part_2(&values_hash_set);
}

