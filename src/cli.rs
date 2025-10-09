use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(
    name = "lamc",
    version,
    about = "Lambda Compiler - expressive, procedural, powerful"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run { file: String },
    Build { file: String },
    Info,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let version = "1.90.0"; // manual update with rust updates i guess

    match cli.command {
        Commands::Run { file } => {
            println!("Running {}", file);
        }

        Commands::Build { file } => {
            println!("Building {}", file)
        }

        Commands::Info => {
            println!("Lambda Compiler {}", env!("CARGO_PKG_VERSION"));
            println!("Built with Rust {}", version.to_string());
        }
    }

    Ok(())
}

