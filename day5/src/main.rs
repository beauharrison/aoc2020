use std::fs;

enum Partition {
    Lower,
    Higher
}


fn map_to_partition(character: &str) -> Partition {
    match character {
        "F" | "L" => Partition::Lower,
        "B" | "R" => Partition::Higher,
        _ => panic!("???")
    }
}

fn main() {
    let input = fs::read_to_string("..\\input\\day4.txt").expect("poop");
    let lines = input.lines();

     lines.map(|l| map_to_partition(l));
}
