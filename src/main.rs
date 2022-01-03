use std::env;
use std::fs;
use std::io;
use std::io::Read;

enum Instruction {
    Add,
    Subtract,
    MoveLeft,
    MoveRight,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

fn trim(source: String) -> String {
    let mut trimmed = Vec::new();

    for ch in source.chars() {
        match ch {
            '+' | '-' |
            '<' | '>' |
            '.' | ',' |
            '[' | ']' => trimmed.push(ch),
            _ => ()
        };
    }

    trimmed.iter().collect::<String>()
}

fn check_loops(source: &String) -> bool {
    let mut count = 0;

    for ch in source.chars() {
        match ch {
            '[' => count = count + 1,
            ']' => count = count - 1,
            _ => ()
        };
    }

    if count == 0 {
        true
    } else {
        false
    }
}

fn compile(source: String) -> Vec<Instruction> {
    let mut compiled_code: Vec<Instruction> = Vec::new();

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
                },
                ']' => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        // Get a substring of the code to loop.
                        let code_to_compile: String = source[loop_begin+1..index].to_string();
                        // Compile the code to produce an Instruction::Loop().
                        let loop_instruction = Instruction::Loop(compile(code_to_compile));
                        // Add the instruction to the compiled code.
                        compiled_code.push(loop_instruction);
                    }
                },
                _ => ()
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
            _ => None
        };

        match operation {
            Some(operation) => compiled_code.push(operation),
            None => ()
        }
    }

    compiled_code
}

fn run(instructions: &Vec<Instruction>, tape: &mut[u8; 30000], pointer: &mut usize) {

    for operation in instructions {
        match operation {
            Instruction::Add => tape[*pointer] += 1,

            Instruction::Subtract => tape[*pointer] -= 1,

            Instruction::MoveLeft => {
                if *pointer == 0 {
                    *pointer = 29999;
                } else {
                    *pointer -= 1;
                }
            },

            Instruction::MoveRight => {
                if *pointer == 29999 {
                    *pointer = 0;
                } else {
                    *pointer += 1;
                }
            },

            Instruction::Write => print!("{}", tape[*pointer] as char),

            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                io::stdin().read_exact(&mut input).expect("ERROR: Failed to read input.");
                tape[*pointer] = input[0];
            },

            Instruction::Loop(loop_instructions) => {
                while tape[*pointer] != 0 {
                    run(loop_instructions, tape, pointer);
                } 
            }

        }
    }
}

fn main() {
    // Get any arguments passed to this application.
    let args: Vec<String> = env::args().collect();

    // Read source file.
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("ERROR: Unable to read file.");

    // Trim comments and other characters from the source.
    let trimmed = trim(content);

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
    let mut pointer: usize = 0;
    run(&compiled, &mut tape, &mut pointer);
}
