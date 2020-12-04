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

fn main() {
    let input = fs::read_to_string("..\\input\\day3.txt").expect("poop");
    let pattern = parse_pattern(input);  

    part_1(&pattern);
}
