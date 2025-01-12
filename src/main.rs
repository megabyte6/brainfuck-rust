mod cli;
mod compiler;
mod error;
mod interpreter;
mod preprocessor;

use std::{fs::read_to_string, process::exit};

use clap::Parser;

use cli::{CliArgs, Command::*};
use interpreter::execute;
use preprocessor::{lex, parse};

fn main() {
    let args = CliArgs::parse();

    /// A macro that conditionally prints to the console based on the verbosity
    /// flag.
    ///
    /// This macro behaves like `println!` and can be used as a drop-in
    /// replacement, but only prints the output if the `verbose` flag in the
    /// `args` structure is set to `true`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let args = Args { verbose: true };
    /// vprintln!("This will be printed because verbose is true.");
    ///
    /// let args = Args { verbose: false };
    /// vprintln!("This will not be printed because verbose is false.");
    /// ```
    ///
    /// # Arguments
    ///
    /// * `$($arg:tt)*` - The format string and arguments, similar to
    ///   `println!`.
    ///
    /// # Note
    ///
    /// The `args` structure must be in scope and contain a `verbose` field of
    /// type `bool`.
    macro_rules! vprintln {
        ($($arg:tt)*) => {{
            if args.verbose {
                println!($($arg)*);
            }
        }};
    }

    match args.command {
        Some(Run { file }) => {
            vprintln!("Reading file: `{file}`");
            let source = read_to_string(file).expect("error: unable to read file.");

            vprintln!("Lexing source code...");
            let tokens = lex(source.as_str()).unwrap_or_else(|errors| {
                for error in errors {
                    eprintln!("error: {error}");
                }
                eprintln!("Please fix errors before continuing.");
                exit(1);
            });

            vprintln!("Generating intermediate representation...");
            let instructions = parse(tokens).unwrap_or_else(|error| {
                eprintln!("error: {error}");
                eprintln!("Please fix errors before continuing.");
                exit(1);
            });

            let mut tape = vec![0u8; args.memory_available.into()];
            let mut pointer: usize = 0;

            vprintln!("Running program...");
            execute(&instructions, &mut tape, &mut pointer);
        }
        Some(Build { file: _, output: _ }) => {
            eprintln!("The 'build' subcommand is not currently implemented. Please use 'run' for the time being.");
        }
        None => (),
    };
}
