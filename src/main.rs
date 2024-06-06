mod cli;
mod compiler;
mod error;
mod interpreter;
mod preprocessor;

use std::fs::read_to_string;

use clap::Parser;

use cli::{CliArgs, Command::*};
use interpreter::execute;
use preprocessor::{lex, parse};

fn main() {
    let args = CliArgs::parse();

    if args.memory_available == 0 {
        eprintln!("Error: The amount of memory available must be greater than 0.");
        return;
    }

    match args.command {
        Some(Run { file }) => {
            if args.verbose {
                println!("Reading file: {}", file);
            }
            let source = read_to_string(file).expect("Error: Unable to read file.");

            if args.verbose {
                println!("Lexing source code...");
            }
            let tokens = match lex(source.as_str()) {
                Ok(tokens) => tokens,
                Err(errors) => {
                    for error in errors {
                        eprintln!("Error: {}", error);
                    }
                    eprintln!("Please fix errors before continuing.");
                    return;
                }
            };

            if args.verbose {
                println!("Generating intermediate representation...")
            }
            let instructions = match parse(tokens) {
                Ok(intermediate) => intermediate,
                Err(error) => {
                    eprintln!("Error: {}", error);
                    eprintln!("Please fix errors before continuing.");
                    return;
                }
            };

            let mut tape = vec![0u8; args.memory_available];
            let mut pointer: usize = 0;

            if args.verbose {
                println!("Running program...");
            }
            execute(&instructions, &mut tape, &mut pointer);
        }
        Some(Build { file: _, output: _ }) => {
            // TODO : Remove when the 'build' subcommand is implemented.
            eprintln!("The 'build' subcommand is not currently implemented. Please use 'run' for the time being.");
        }
        None => (),
    }
}
