use std::io::{stdin, Read};

use crate::preprocessor::Instruction;

/// Executes the given intermediate instructions immediately.
pub fn execute(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, pointer: &mut usize) {
    for instruction in instructions {
        match instruction {
            Instruction::IncrementPointer => {
                *pointer += 1;
                if *pointer == tape.len() {
                    *pointer = 0;
                }
            }
            Instruction::DecrementPointer => {
                if *pointer == 0 {
                    *pointer = tape.len();
                }
                *pointer -= 1;
            }
            Instruction::IncrementValue => {
                if tape[*pointer] == 255 {
                    tape[*pointer] = 0;
                } else {
                    tape[*pointer] += 1;
                }
            }
            Instruction::DecrementValue => {
                if tape[*pointer] == 0 {
                    tape[*pointer] = 255;
                } else {
                    tape[*pointer] -= 1;
                }
            }
            Instruction::Write => print!("{}", tape[*pointer] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0];
                stdin()
                    .read_exact(&mut input)
                    .expect("ERROR: Failed to read input.");
                tape[*pointer] = input[0];
            }
            Instruction::Loop(loop_instructions) => {
                while tape[*pointer] != 0 {
                    execute(loop_instructions, tape, pointer);
                }
            }
        }
    }
}
