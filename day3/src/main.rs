use std::fs;

const PATTERN_HEIGHT: usize = 323;
const PATTERN_WIDTH: usize = 31;

#[derive(Debug, Copy, Clone)]
enum Landscape {
    Open,
    Tree
}

fn parse_pattern(pattern_string: String) -> [[Landscape; PATTERN_WIDTH]; PATTERN_HEIGHT] {
    let lines_iter = pattern_string.lines();

    let mut pattern = [[Landscape::Open; PATTERN_WIDTH]; PATTERN_HEIGHT];

    let mut x: usize = 0;
    let mut y: usize = 0;

    for line in lines_iter {
        for c in line.chars() {
            let landscape = match c {
                '.' => Landscape::Open,
                '#' => Landscape::Tree,
                _ => panic!("Invalid landscape: {}", c)
            };

            pattern[y][x] = landscape;

            x += 1;
        }
        
        y += 1;
        x = 0;
    }

    pattern
}

fn get_tree_count(
    pattern: &[[Landscape; PATTERN_WIDTH]; PATTERN_HEIGHT],
    x_interval: usize,
    y_interval: usize
) -> i64 {
    let mut tree_count = 0;

    let mut x = 0;

    for y in (0..PATTERN_HEIGHT).step_by(y_interval) {
        if let Landscape::Tree = pattern[y][x] {
            tree_count += 1;
        }

        x += x_interval;
        x = x % PATTERN_WIDTH;
    }

    tree_count
}

fn part_1(pattern: &[[Landscape; PATTERN_WIDTH]; PATTERN_HEIGHT]) {
    let mut tree_count = 0;

    let mut x = 0;

    for y in 0..PATTERN_HEIGHT {
        if let Landscape::Tree = pattern[y][x] {
            tree_count += 1;
        }

        x += 3;
        x = x % PATTERN_WIDTH;
    }

    println!("Part 1 count: {}", tree_count);
}

fn part_2(pattern: &[[Landscape; PATTERN_WIDTH]; PATTERN_HEIGHT]) {
    let count_1_1 = get_tree_count(pattern, 1, 1); 
    let count_3_1 = get_tree_count(pattern, 3, 1); 
    let count_5_1 = get_tree_count(pattern, 5, 1); 
    let count_7_1 = get_tree_count(pattern, 7, 1); 
    let count_1_2 = get_tree_count(pattern, 1, 2);
    
    println!("1, 1: {}", count_1_1);
    println!("3, 1: {}", count_3_1);
    println!("5, 1: {}", count_5_1);
    println!("7, 1: {}", count_7_1);
    println!("1, 2: {}", count_1_2);

    let tree_count: i64 = [ 
        count_1_1,
        count_3_1,
        count_5_1,
        count_7_1,
        count_1_2
    ].iter().fold(1, |acc, x| acc * x);

    println!("Part 1 count: {}", tree_count);
}

fn main() {
    let input = fs::read_to_string("..\\input\\day3.txt").expect("poop");
    let pattern = parse_pattern(input);  

    part_1(&pattern);
    part_2(&pattern);
}
