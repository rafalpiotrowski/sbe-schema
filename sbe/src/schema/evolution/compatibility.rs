use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser, ValueEnum};
use sbe_schema::{FullCompatibility, NoneCompatibility, SbeSchemaValidator};

use crate::term::info;

///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CompatibilityArgs {
    #[arg(long, short)]
    pub level: CompatibilityLevel,
    #[arg(long)]
    pub latest: PathBuf,
    #[arg(long)]
    pub current: PathBuf,    
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CompatibilityLevel {
    Backward,
    Forward,
    Full,
    None,
}

pub fn check(args: CompatibilityArgs) -> Result<()> {

    match args {
        CompatibilityArgs { 
            level: CompatibilityLevel::Backward,
            latest: _, current: _
         } => {
            info("Checking backward compatibility")?;
        },
        CompatibilityArgs { 
            level: CompatibilityLevel::Forward,
            latest: _, current: _
         } => {
            info("Checking forward compatibility")?;
        },
        CompatibilityArgs { 
            level: CompatibilityLevel::Full,
            latest, current
         } => {
            info("Checking full compatibility")?;
            let latest_schema = latest.try_into()?;
            let current_schema = current.try_into()?;
            let v = sbe_schema::Validator::new(FullCompatibility::<SbeSchemaValidator>::new(SbeSchemaValidator {}));
            v.check(latest_schema, current_schema)?;
        },
        CompatibilityArgs { 
            level: CompatibilityLevel::None,
            latest: _, current: _
         } => {
            info("Checking no compatibility")?;
            let v = sbe_schema::Validator::new(NoneCompatibility::<SbeSchemaValidator>::new(SbeSchemaValidator{}));
            v.check(sbe_schema::Schema::default(), sbe_schema::Schema::default())?;
        },
    }

    Ok(())
}
