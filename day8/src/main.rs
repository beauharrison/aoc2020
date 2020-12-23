use std::fs;
use std::collections::HashSet;

#[derive(Clone)]
enum OpCode {
    ACC(i32),
    JMP(i32),
    NOP(i32)
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

fn execute_program(instructions: Vec<OpCode>) -> Option<i32> {
    let mut acc = 0;
    let mut ptr = 0;

    let mut seen_instructions = HashSet::<usize>::new();

    loop {
        if seen_instructions.contains(&ptr) {
            return None;
        }

        if ptr == instructions.len() {
            return Some(acc);
        }

        seen_instructions.insert(ptr);
        let instruction = instructions.get(ptr);
        
        match instruction {
            Some(OpCode::ACC(v)) => {
                acc += v;
                ptr += 1;
            },
            Some(OpCode::JMP(v)) => {
                if *v < 0 {
                    ptr -= v.abs() as usize;
                } else {
                    ptr += *v as usize;
                }                
            },
            Some(OpCode::NOP(_)) => {
                ptr += 1;
            },
            None => panic!("Invalid instruction pointer")
        }
    }
}


fn part_2(instructions: &Vec<OpCode>) {
    for i in 0..instructions.len() {
        let swap_instruction_op = instructions.get(i);

        if let Some(op) = swap_instruction_op {
            if let OpCode::ACC(_) = op {
                continue;
            }

            let mut mod_instructions = Vec::<OpCode>::new();
            mod_instructions.clone_from(instructions);

            match op {
                OpCode::JMP(jmp_val) => {
                    mod_instructions[i] = OpCode::NOP(*jmp_val);
                },
                OpCode::NOP(nop_val) => {
                    mod_instructions[i] = OpCode::JMP(*nop_val);
                },
                _ => {}
            }

            let result = execute_program(mod_instructions);

            if let Some(result_acc) = result {
                println!("part 2 acc: {}", result_acc);
                break;
            }
        }    
    }    
}

fn main() {
    let input = fs::read_to_string("..\\input\\day8.txt").expect("poop");
    let instructions = parse_input(input);

    part_1(&instructions);
    part_2(&instructions);
}
