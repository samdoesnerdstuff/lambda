//! main.rs
//! 
//! This is the compiler entrypoint, it takes in user passed arguments
//! before handing off to the lexer, which will search for the source
//! code and start the compilation process.
//! 
//! 

use std::env;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    
    for arg in cli_args.iter().skip(1) {
        match arg.as_str() {
            "-help" => display_help(),
            "-version" => display_version(),
            "-help-compiler-flags" => all_compiler_flags(),
            &_ => println!("")
        }
    }
}

fn display_help() {
    println!("* Lambda Compiler");
    println!("* Version {}", env!("CARGO_PKG_VERSION"));
    println!("* Target Arch: {}", std::env::consts::ARCH);
    println!("* Target OS: {}", std::env::consts::OS);
    println!("* Usage:");
    println!("*    -help    => Displays help menu");
    println!("*    -version => Displays compiler version");
    println!("*");
    println!("* Compiler Flags:");
    println!("*    -s => Specify source file(s) for compilation");
    println!("*    -o => Specify output location (defaults to wherever lamc is invoked)");
    println!("*    -wl (low,med,hi) => Compiler warning level");
    println!("*    -opt (0-3) => Compiler optimization level");
    println!("*");
    println!("* For ALL compilation flags, use '-help-compiler-flags' ");
    println!("* (Recommended that you pipe the output to a text file)");
}

fn all_compiler_flags() {

}

fn display_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}