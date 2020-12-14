use std::fs;
use std::collections::{HashSet, VecDeque};

enum OpCode {
    ACC(i32),
    JMP(i32),
    NOP(i32)
}

enum MachineState {
    Acc: i32
    Ptr: usize
}

fn parse_input(input: String) -> Vec<OpCode> {
    input
        .lines()
        .map(|l| {
            let mut instruction_split = l.split(" ");
            
            let op_string = instruction_split.next().unwrap();
            let v: i32 = instruction_split.next().unwrap().to_string().parse::<i32>().unwrap();

            match op_string {
                "acc" => OpCode::ACC(v),
                "jmp" => OpCode::JMP(v),
                "nop" => OpCode::NOP(v),
                _ => panic!("unexpected op code: {}", op_string)
            }
        })
        .collect::<Vec<_>>()
}

fn part_1(instructions: &Vec<OpCode>) {
    let mut acc = 0;
    let mut ptr = 0;

    let mut seen_instructions = HashSet::<usize>::new();

    loop {
        if seen_instructions.contains(&ptr) {
            break;
        }

        seen_instructions.insert(ptr);
        let instruction = instructions.get(ptr).unwrap();
        
        match instruction {
            OpCode::ACC(v) => {
                acc += v;
                ptr += 1;
            },
            OpCode::JMP(v) => {
                if *v < 0 {
                    ptr -= v.abs() as usize;
                } else {
                    ptr += *v as usize;
                }                
            },
            OpCode::NOP(_) => {
                ptr += 1;
            }
        }
    }

    println!("part 1 acc: {}", acc);
}


fn part_2(instructions: &Vec<OpCode>) {
    let winning = HashSet::<usize>::new();

    



    println!("part 2 acc: {}", acc);
}

fn main() {
    let input = fs::read_to_string("..\\input\\day8.txt").expect("poop");
    let instructions = parse_input(input);

    part_1(&instructions);
    part_2(&instructions);
}
