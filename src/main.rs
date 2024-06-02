mod cli;
mod compiler;
mod error;
mod interpreter;
mod preprocessor;

use std::fs::read_to_string;

use clap::Parser;

use cli::{CliArgs, Command::*};
use interpreter::execute;
use preprocessor::{generate_intermediate, lex};

fn main() {
    let args = CliArgs::parse();

    match args.command {
        Some(Run {
            file,
            memory_available,
        }) => {
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
            let intermediate = match generate_intermediate(tokens) {
                Ok(intermediate) => intermediate,
                Err(error) => {
                    eprintln!("Error: {}", error);
                    eprintln!("Please fix errors before continuing.");
                    return;
                }
            };

            let mut tape = vec![0u8; memory_available];
            let mut pointer: usize = 0;

            if args.verbose {
                println!("Running program...");
            }
            execute(&intermediate, &mut tape, &mut pointer);
        } // TODO : Uncomment when the 'build' subcommand is implemented.
          // Some(Build { file: _, output: _ }) => {
          //     eprintln!("The 'build' subcommand is not currently implemented. Please use 'run' for the time being.");
          // }
          // None => (),
    }
}
