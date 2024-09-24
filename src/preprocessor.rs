use std::mem::discriminant;

use crate::error::{LoopError, SyntaxError};

/// The location of a token in the source code.
#[derive(Clone, Debug)]
pub struct SourceLocation {
    pub line: u32,
    pub column: u32,
}

/// The options for tokens that are generated from the lexing process.
#[derive(Clone, Debug)]
pub enum Token {
    MoveRight(SourceLocation),
    MoveLeft(SourceLocation),
    Increment(SourceLocation),
    Decrement(SourceLocation),
    Write(SourceLocation),
    Read(SourceLocation),
    LoopStart(SourceLocation),
    LoopEnd(SourceLocation),
}

/// Generates a vector of tokens from the raw source code.
pub fn lex(source: &str) -> Result<Vec<Token>, Vec<SyntaxError>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    // The current location of the lexer in the source code. Used to map the
    // instructions to source positions for debugging.
    let mut current_location = SourceLocation { line: 1, column: 1 };
    // The active loops that are currently open. Used to check for missing
    // opening or closing symbols.
    let mut active_loops = Vec::new();

    for symbol in source.chars() {
        match symbol {
            '>' => tokens.push(Token::MoveRight(current_location.clone())),
            '<' => tokens.push(Token::MoveLeft(current_location.clone())),
            '+' => tokens.push(Token::Increment(current_location.clone())),
            '-' => tokens.push(Token::Decrement(current_location.clone())),
            '.' => tokens.push(Token::Write(current_location.clone())),
            ',' => tokens.push(Token::Read(current_location.clone())),
            '[' => {
                active_loops.push(current_location.clone());
                tokens.push(Token::LoopStart(current_location.clone()))
            }
            ']' => {
                if active_loops.is_empty() {
                    errors.push(SyntaxError::from_source_location(
                        &current_location,
                        Box::new(LoopError::MissingStart),
                    ));
                    continue;
                }
                active_loops.pop();
                tokens.push(Token::LoopEnd(current_location.clone()))
            }
            '\n' => {
                current_location.line += 1;
                current_location.column = 0;
            }
            _ => (),
        };

        current_location.column += 1;
    }

    if !active_loops.is_empty() {
        for location in active_loops {
            errors.push(SyntaxError::from_source_location(
                &location,
                Box::new(LoopError::MissingEnd),
            ));
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

/// The instruction options for the intermediate representation of the source
/// code.
#[derive(Clone, Debug)]
pub enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(usize),
    Decrement(usize),
    Write(usize),
    Read,
    Loop(Vec<Instruction>),
}

/// Generates a vector of instructions from the vector of tokens.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, SyntaxError> {
    let mut instructions = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let mut count = count_repeated(&tokens[index..]);
        let instruction = match &tokens[index] {
            Token::MoveRight(_) => Instruction::MoveRight(count),
            Token::MoveLeft(_) => Instruction::MoveLeft(count),
            Token::Increment(_) => Instruction::Increment(count),
            Token::Decrement(_) => Instruction::Decrement(count),
            Token::Write(_) => Instruction::Write(count),
            Token::Read(_) => Instruction::Read,
            Token::LoopStart(source_location) => {
                let end_index = match end_loop_index(&tokens, index) {
                    Ok(index) => index,
                    Err(error) => {
                        return Err(SyntaxError::from_source_location(
                            source_location,
                            Box::new(error),
                        ))
                    }
                };
                let loop_content = tokens[index + 1..end_index].to_vec();
                // Skip to the end of the loop
                count = end_index - index + 1;
                Instruction::Loop(parse(loop_content)?)
            }
            Token::LoopEnd(source_location) => {
                return Err(SyntaxError::from_source_location(
                    source_location,
                    Box::new(LoopError::MissingStart),
                ));
            }
        };
        instructions.push(instruction);
        index += count;
    }
    Ok(instructions)
}

/// Count the number of consecutive tokens of the same type from the beginning
/// of the slice.
fn count_repeated(tokens: &[Token]) -> usize {
    let initial_type = discriminant(&tokens[0]);
    let mut count = 1;
    for token in tokens[1..].iter() {
        if discriminant(token) != initial_type {
            break;
        }
        count += 1;
    }
    count
}

/// Find the corresponding closing end of a given opening end of a loop.
fn end_loop_index(tokens: &[Token], start_loop_index: usize) -> Result<usize, LoopError> {
    let mut loop_stack = 0;
    for (index, token) in tokens.iter().enumerate().skip(start_loop_index + 1) {
        match token {
            Token::LoopStart(_) => loop_stack += 1,
            Token::LoopEnd(_) => {
                if loop_stack == 0 {
                    return Ok(index);
                }
                loop_stack -= 1;
            }
            _ => (),
        }
    }
    Err(LoopError::MissingEnd)
}
