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
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    LoopStart(SourceLocation),
    LoopEnd(SourceLocation),
}

/// Generates a vector of tokens from the raw source code.
pub fn lex(source: &str) -> Result<Vec<Token>, Vec<SyntaxError>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    // The current location of the lexer in the source code. Used to map the
    // instructions to source positions for debugging.
    let mut source_location = SourceLocation { line: 1, column: 1 };
    let mut source_location = SourceLocation { line: 1, column: 1 };
    // The active loops that are currently open. Used to check for missing
    // opening or closing symbols.
    let mut open_loops = Vec::new();
    let mut open_loops = Vec::new();

    for symbol in source.chars() {
        match symbol {
            '>' => tokens.push(Token::MoveRight),
            '<' => tokens.push(Token::MoveLeft),
            '+' => tokens.push(Token::Increment),
            '-' => tokens.push(Token::Decrement),
            '.' => tokens.push(Token::Write),
            ',' => tokens.push(Token::Read),
            '[' => {
                open_loops.push(source_location.clone());
                tokens.push(Token::LoopStart(source_location.clone()))
                open_loops.push(source_location.clone());
                tokens.push(Token::LoopStart(source_location.clone()))
            }
            ']' => {
                if open_loops.is_empty() {
                if open_loops.is_empty() {
                    errors.push(SyntaxError::from_source_location(
                        &source_location,
                        Box::new(LoopError::MissingStart),
                        &source_location,
                        Box::new(LoopError::MissingStart),
                    ));
                    continue;
                }
                open_loops.pop();
                tokens.push(Token::LoopEnd(source_location.clone()))
                open_loops.pop();
                tokens.push(Token::LoopEnd(source_location.clone()))
            }
            '\n' => {
                source_location.line += 1;
                source_location.column = 0;
                source_location.line += 1;
                source_location.column = 0;
            }
            _ => (),
        };

        source_location.column += 1;
        source_location.column += 1;
    }

    if !open_loops.is_empty() {
        for location in open_loops {
    if !open_loops.is_empty() {
        for location in open_loops {
            errors.push(SyntaxError::from_source_location(
                &location,
                Box::new(LoopError::MissingEnd),
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
            Token::MoveRight => Instruction::MoveRight(count),
            Token::MoveLeft => Instruction::MoveLeft(count),
            Token::Increment => Instruction::Increment(count),
            Token::Decrement => Instruction::Decrement(count),
            Token::Write => Instruction::Write(count),
            Token::Read => Instruction::Read,
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
        };
    }
    Err(LoopError::MissingEnd)
    Err(LoopError::MissingEnd)
}
