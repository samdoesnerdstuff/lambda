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
    let rs_version = "1.90.0"; // manual update with rust updates i guess

    match cli.command {
        Commands::Run { file } => {
            println!("Running {}", file);
        }

        Commands::Build { file } => {
            println!("Building {}", file)
        }

        Commands::Info => {
            println!("Lambda Compiler: {}", env!("CARGO_PKG_VERSION"));
            println!("Built with Rust: {}", rs_version.to_string());
            println!("Targeting arch: {}", std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or("UNKNOWN".to_string()));
            println!("For OS: {}", std::env::var("CARGO_CFG_TARGET_OS").unwrap_or("UNKNOWN".to_string()));
        }
    }

    Ok(())
}

