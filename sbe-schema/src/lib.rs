//! # sbe-schema
//!
//! `sbe-schema` is a library for working with SBE schema files.
//!

mod evolution;
mod schema;
mod types;

use evolution::EvolutionError;
use std::result::Result as StdResult;
use thiserror::Error;

pub use evolution::{
    CompatibilityLevel, PartialCompatibility, EvolutionStrategy, FullCompatibility, NoneCompatibility,
    SbeSchemaValidator, SchemaValidator, Validator,
};
pub use types::{Schema, VTable, build_vtable};

/// Result type returned from methods that have [`enum@self::Error`].
pub type Result<T> = StdResult<T, SbeSchemaError>;

/// Errors that can occur when working with SBE schema files.
#[derive(Error, Debug)]
pub enum SbeSchemaError {
    /// An error occurred while validating schema evolution.
    #[error(transparent)]
    Evolution(#[from] EvolutionError),
}
