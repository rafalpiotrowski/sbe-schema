use anyhow::{bail, Result};
use clap::{command, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GenerateArgs {
    #[arg(long, short)]
    pub language: Language,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Rust
}

pub fn run(args: GenerateArgs) -> Result<()> {
    Ok(())
}