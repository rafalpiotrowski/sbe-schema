use anyhow::Result;
use clap::{command, Parser, ValueEnum};

///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CompatibilityArgs {
    #[arg(long, short)]
    pub level: CompatibilityLevel,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CompatibilityLevel {
    Backward,
    BackwardTransitive,
    Forward,
    ForwardTransitive,
    Full,
    FullTransitive,
    None,
}

pub fn check(_args: CompatibilityArgs) -> Result<()> {
    Ok(())
}
