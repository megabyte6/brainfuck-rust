use std::io::{stdin, Read};

use crate::preprocessor::Instruction;

/// Executes the given intermediate instructions immediately.
pub fn execute(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, pointer: &mut usize) {
    for instruction in instructions {
        match instruction {
            Instruction::MoveRight(count) => {
                let increment = *count % tape.len();
                let (new_pointer, overflow) = pointer.overflowing_add(increment);
                *pointer = if overflow {
                    new_pointer + (usize::MAX - tape.len() + 1)
                } else if new_pointer >= tape.len() {
                    new_pointer - tape.len()
                } else {
                    new_pointer
                };
            }
            Instruction::MoveLeft(count) => {
                let decrement = *count % tape.len();
                if decrement > *pointer {
                    *pointer = tape.len() - (decrement - *pointer);
                } else {
                    *pointer -= decrement;
                }
            }
            Instruction::Increment(count) => {
                tape[*pointer] =
                    tape[*pointer].wrapping_add((*count % (u8::MAX as usize + 1)) as u8)
            }

            Instruction::Decrement(count) => {
                tape[*pointer] =
                    tape[*pointer].wrapping_sub((*count % (u8::MAX as usize + 1)) as u8);
            }
            Instruction::Write(count) => {
                if *count == 1 {
                    print!("{}", tape[*pointer] as char);
                } else {
                    let data = vec![tape[*pointer]; *count];
                    print!("{}", String::from_utf8(data).unwrap());
                }
            }
            Instruction::Read => {
                let mut input: [u8; 1] = [0];
                stdin()
                    .read_exact(&mut input)
                    .expect("error: failed to read input.");
                tape[*pointer] = input[0];
            }
            Instruction::Loop(loop_instructions) => {
                while tape[*pointer] != 0 {
                    execute(loop_instructions, tape, pointer);
                }
            }
        };
    }
}
