use std::{collections::HashSet, fs};

const PREAMBLE_SIZE: usize = 25;

struct DataWindowIterator<'a> {
    data: &'a Vec<i64>,
    index: usize
}

impl<'a> DataWindowIterator<'a> {
    fn new(data: &'a Vec<i64>) -> DataWindowIterator<'a> {
        DataWindowIterator::<'a> {
            data: data,
            index: PREAMBLE_SIZE - 1
        }
    }

    fn get_window(&self) -> Option<&[i64]> {
        if self.index < PREAMBLE_SIZE || self.index >= self.data.len() {
            return None;
        }

        let preamble_start = self.index - PREAMBLE_SIZE;

        Some(&self.data[preamble_start..self.index])
    }
}

impl Iterator for DataWindowIterator<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.index += 1;

        if self.index >= PREAMBLE_SIZE || self.index < self.data.len() {
            return Some(self.data[self.index])
        } else {
            return None;
        }
    }
}

fn part_1(data: &Vec<i64>) {
    let mut window_iter = DataWindowIterator::new(data);

    'w: while let Some(value) = window_iter.next() {
        let window = window_iter.get_window().unwrap();

        let mut need = HashSet::<i64>::new();

        for window_value in window {
            if need.contains(window_value) {
                continue 'w;
            }
            
            need.insert(value - *window_value);
        }
        
        println!("part 1 ans: {}", value);
        break;
    }

}

fn main() {
    let input = fs::read_to_string("..\\input\\day9.txt").expect("poop");
    
    let mut data = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    part_1(&mut data);

}
