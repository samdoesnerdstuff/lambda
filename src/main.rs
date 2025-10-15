//! main.rs
//! 
//! This is the compiler entrypoint, it takes in user passed arguments
//! before handing off to the lexer, which will search for the source
//! code and start the compilation process.
//! 
//! 

// code style is OK, this is to make sure logic and performance aren't abysmal
#![warn(clippy::perf, clippy::correctness)]
#![allow(
    clippy::style,
    clippy::pedantic,
    clippy::nursery,
    clippy::too_many_lines,
    clippy::needless_lifetimes,
    clippy::needless_return
)]

mod lexer;
mod typecheck;
mod parser;

use std::env;
use colored::*;
use lexer::lex;
use parser::{Parser, ParseError};
use parser::ast::Stmt;
use typecheck::typecheck;

#[allow(
    unused_assignments,
    unused_variables
)]
fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let mut verbose: bool = false;
    
    for arg in cli_args.iter().skip(1) {
        match arg.as_str() {
            "-help" => display_help(),
            "-version" => display_version(),
            "-help-compiler-flags" => all_compiler_flags(),
            "-test-lex" => test_lex(),
            "-test-parse" => test_parse(),
            "-verbose" => { verbose = true },

            "-s" => {
                if let Some(path) = cli_args.iter().skip_while(|a| *a != "-s").nth(1) {
                    match std::fs::read_to_string(path) {
                        Ok(src) => {
                            compile_source(&src, verbose).map_or_else(
                                |e| eprintln!("Error during compilation: {:?}", e),
                                |_| println!("Compilation successful!")
                            );
                        }
                        Err(e) => eprintln!("Failed to read file: \"{}\": {}", path, e),
                    }
                } else {
                    eprintln!("-s flag requires a file path argument!");
                }
            },

            &_ => println!("")
        }
    }
}

pub fn display_help() {
    println!("{}", "* Lambda Compiler".bold().bright_yellow());
    println!("* Version: {}", env!("CARGO_PKG_VERSION").bright_cyan());
    println!("* Target Arch: {}", std::env::consts::ARCH.green());
    println!("* Target OS: {}", std::env::consts::OS.green());
    println!();
    println!("{}", "* Usage:".bold().bright_yellow());
    println!("  lamc [options]");
    println!();
    println!("{}", "* General Options:".bold().bright_yellow());
    println!("  {:<20} {}", "-help".cyan(), "Display this help menu");
    println!("  {:<20} {}", "-version".cyan(), "Display compiler version");
    println!("  {:<20} {}", "-verbose".cyan(), "Display verbose info during compilation.");
    println!();
    println!("{}", "* Compiler Flags:".bold().bright_yellow());
    println!("  {:<20} {}", "-s <file>".cyan(), "Specify source file(s) for compilation");
    println!("  {:<20} {}", "-o <path>".cyan(), "Specify output location");
    println!("  {:<20} {}", "-wl <low|med|hi>".cyan(), "Set compiler warning level");
    println!("  {:<20} {}", "-opt <low|med|hi>".cyan(), "Set compiler optimization level");
    println!("  {:<20} {}", "-entry <func>".cyan(), "Change entry point from main to <func>");
    println!();
    println!("{}", "* Notes:".bold().bright_yellow());
    println!(
        "  {}",
        "Use '-help-compiler-flags' for detailed documentation on compiler flags."
            .truecolor(180, 180, 180)
    );
    println!(
        "  {}",
        "Output can be piped to a file for easier reading and navigation."
            .truecolor(180, 180, 180)
    );
}

pub fn all_compiler_flags() {

}

pub fn display_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn test_lex() {
    let source = r#"
        fn hello()
            write("hello!")
        end
    "#;

    let tokens = lex(source, false);
    println!("Lexer Output:");
    for (token, span) in tokens {
        println!("{:?} @ {:?}", token, span);
    }
}

pub fn test_parse() {
    let source = r#"
        fn hello()
            write("hello!")
        end
    "#;

    let tokens = lex(source, false);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("{:#?}", ast);
}

pub fn compile_source(source: &str, verbose: bool) -> Result<Vec<Stmt>, ParseError> {
    // Lexing step
    let tokens = lex(source, verbose);

    if verbose {
        println!("LEXER: {:?}", tokens);
    }

    // Parsing step
    let mut parser = Parser::new(tokens);

    // AST
    let ast = parser.parse()?;

    if verbose {
        println!("PARSED AST: {:?}", ast);
    }

    // Check those types!
    typecheck(&ast);
}