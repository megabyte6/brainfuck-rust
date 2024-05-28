use std::env;
use std::fs;
use std::io::Read;

enum Instruction {
    Add,
    Subtract,
    MoveLeft,
    MoveRight,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

fn trim(source: String) -> String {
    let mut trimmed = Vec::new();
    for ch in source.chars() {
        match ch {
            '+' | '-' | '<' | '>' | '.' | ',' | '[' | ']' => trimmed.push(ch),
            _ => (),
        };
    }
    trimmed.iter().collect::<String>()
}

fn check_loops(source: &str) -> bool {
    let mut count = 0;
    for ch in source.chars() {
        match ch {
            '[' => count += 1,
            ']' => count -= 1,
            _ => (),
        };
    }
    count == 0
}

fn compile(source: String) -> Vec<Instruction> {
    let mut compiled: Vec<Instruction> = Vec::new();

    // How many loops are nested.
    let mut loop_stack = 0;
    // The index of the beginning of the loop.
    let mut loop_begin = 0;

    for (index, symbol) in source.chars().enumerate() {
        // Check if the current code is in a loop.
        if loop_stack != 0 {
            match symbol {
                '[' => {
                    loop_stack += 1;
                }
                ']' => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        // Get a substring of the code to loop.
                        let code_to_compile: String = source[loop_begin + 1..index].to_string();
                        // Compile the code to produce an Instruction::Loop().
                        let loop_instruction = Instruction::Loop(compile(code_to_compile));
                        // Add the instruction to the compiled code.
                        compiled.push(loop_instruction);
                    }
                }
                _ => (),
            }

            continue;
        }

        let operation = match symbol {
            '+' => Some(Instruction::Add),
            '-' => Some(Instruction::Subtract),
            '<' => Some(Instruction::MoveLeft),
            '>' => Some(Instruction::MoveRight),
            '.' => Some(Instruction::Write),
            ',' => Some(Instruction::Read),
            '[' => {
                loop_stack += 1;
                loop_begin = index;
                None
            }
            _ => None,
        };

        if let Some(operation) = operation {
            compiled.push(operation);
        }
    }

    compiled
}

fn run(instructions: &Vec<Instruction>, tape: &mut [u8; 30000], index: &mut usize) {
    for operation in instructions {
        match operation {
            Instruction::Add => tape[*index] += 1,
            Instruction::Subtract => tape[*index] -= 1,
            Instruction::MoveLeft => {
                if *index == 0 {
                    *index = 29999;
                } else {
                    *index -= 1;
                }
            }
            Instruction::MoveRight => {
                if *index == 29999 {
                    *index = 0;
                } else {
                    *index += 1;
                }
            }
            Instruction::Write => print!("{}", tape[*index] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("ERROR: Failed to read input.");
                tape[*index] = input[0];
            }
            Instruction::Loop(loop_instructions) => {
                while tape[*index] != 0 {
                    run(loop_instructions, tape, index);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        println!("Please specify a source file.");
    }

    let source = fs::read_to_string(&args[1]).expect("ERROR: Unable to read file.");

    // Trim comments and other characters from the source.
    let trimmed = trim(source);

    // Check if all loops are complete.
    let loops_valid = check_loops(&trimmed);
    if !loops_valid {
        println!("Please check that all of your loops are complete and then run again.");
        return;
    }

    // Compile the source to a vector of Instruction.
    let compiled = compile(trimmed);

    // Run the compiled code.
    let mut tape: [u8; 30000] = [0; 30000];
    let mut index: usize = 0;
    run(&compiled, &mut tape, &mut index);
}
