use std::collections::HashSet;
use std::fs;

fn determine<TIter>(
    min: i32,
    max: i32,
    take_lower: char,
    take_higher: char,
    location: TIter
) -> i32 
where TIter: Iterator<Item = char> 
{
    let mut range_min = min as f64;
    let mut range_max = max as f64;

    for lc in location {
        let diff = range_max - range_min;

        if lc == take_lower {
            if diff == 1.0 {
                return range_min as i32;
            } else {
                range_max = (range_max - diff / 2.0).floor();
            }            
        } else if lc == take_higher {
            if diff == 1.0 {
                return range_max as i32;
            } else {
                range_min = (range_min + diff / 2.0).ceil();
            }
        } else {
            panic!("who knows? {}", lc);
        }
    }

    panic!("inconclusive");
}

fn part_1(lines: &Vec<&str>) {
    let mut ans = 0;

    for line in lines {
        let mut chars_iter = line.chars();
        
        let row_chars = chars_iter.by_ref().take(7);        
        let row = determine(0, 127, 'F', 'B', row_chars);
        
        let col_chars = chars_iter.take(3);
        let col = determine(0, 7, 'L', 'R', col_chars);

        let seat_id = row * 8 + col;

        if seat_id > ans {
            ans = seat_id;
        }
    }

    println!("part 1: {}", ans);
}

fn part_2(lines: &Vec<&str>) {
    let ids_set = lines.iter().map(|line| {
        let mut chars_iter = line.chars();
        
        let row_chars = chars_iter.by_ref().take(7);        
        let row = determine(0, 127, 'F', 'B', row_chars);
        
        let col_chars = chars_iter.take(3);
        let col = determine(0, 7, 'L', 'R', col_chars);

        row * 8 + col
    }).collect::<HashSet<i32>>();

    for i in 0..1000 {
        if !ids_set.contains(&i) && ids_set.contains(&(i-1)) && ids_set.contains(&(i+1)) {
            println!("part 2: {}", i);
            break;
        }
    }
}

fn main() {
    let input = fs::read_to_string("..\\input\\day5.txt").expect("poop");
    let lines = input.lines().collect::<Vec<&str>>();

    part_1(&lines);
    part_2(&lines);
}
