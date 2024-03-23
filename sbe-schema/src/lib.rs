//! # sbe-schema
//!
//! `sbe-schema` is a library for working with SBE schema files.
//!

mod evolution;
mod types;
mod schema;

use std::result::Result as StdResult;
use thiserror::Error;
use evolution::EvolutionError;

pub use evolution::{CompatibilityLevel, EvolutionStrategy, Validator, NoneCompatibility, FullCompatibility, SbeSchemaValidator, SchemaValidator};
pub use types::Schema;

/// Result type returned from methods that have [`enum@self::Error`].
pub type Result<T> = StdResult<T, SbeSchemaError>;

/// Errors that can occur when working with SBE schema files.
#[derive(Error, Debug)]
pub enum SbeSchemaError {
    /// An error occurred while validating schema evolution.
    #[error(transparent)]
    Evolution(#[from] EvolutionError),
}