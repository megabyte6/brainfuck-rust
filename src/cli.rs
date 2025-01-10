use std::num::NonZeroUsize;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Print verbose information. Useful for debugging.
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run the source file in interpreter mode
    Run {
        /// The source file to run
        file: String,

        /// The amount of memory available to the program in cells(bytes)
        #[arg(long, default_value_t = NonZeroUsize::new(30000).expect("Default memory available is invalid."))]
        memory_available: NonZeroUsize,
    },
    // TODO : Implement the 'build' subcommand
    /*
    /// Build the source code to an executable file
    Build {
        /// The source file to compile
        file: String,

        /// The file to output the binary to
        #[arg(short, long)]
        output: String,
    },
    */
}
