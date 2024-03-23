//!
//!

mod backward;
mod forward;
mod full;

use crate::types::Schema;
use thiserror::Error;

pub use full::FullCompatibility;

/// The compatibility level of a schema evolution strategy.
#[derive(Debug, Clone, Copy)]
pub enum CompatibilityLevel {
    /// Delete fields and Add optional fields
    Backward,
    /// Add fields and Delete optional fields
    Forward,
    /// Add optional fields and Delete optional fields
    Full,
    /// All changes are accepted
    None,
}

#[derive(Error, Debug)]
pub enum EvolutionError {
    #[error("Schema is not compatible with the latest schema! Compatibility level: {0:?}")]
    SchemaNotCompatible(CompatibilityLevel),
}

/// A strategy for schema evolution.
pub trait EvolutionStrategy {
    /// The compatibility level of the strategy.
    fn compatibility_level(&self) -> CompatibilityLevel;
    /// Check if the current schema is compatible with the latest schema.
    fn check(&self, _latest_schema: Schema, _current_schema: Schema) -> Result<(), EvolutionError>
    {
        match self.compatibility_level() {
            CompatibilityLevel::None => Ok(()),
            _ => Err(EvolutionError::SchemaNotCompatible(self.compatibility_level()))
        }
    }
}

/// A validator for schema evolution.
pub struct Validator<E: EvolutionStrategy> {
    strategy: E,
}

// Implement the Validator struct
impl<E: EvolutionStrategy> Validator<E> {
    /// Create a new validator with the given strategy.
    pub fn new(strategy: E) -> Self {
        Self {
            strategy,
        }
    }

    /// Check if the current schema is compatible with the latest schema.
    pub fn check(&self, latest_schema: Schema, current_schema: Schema) -> Result<(), EvolutionError> {
        self.strategy.check(latest_schema, current_schema)
    }
}

/// A strategy that accepts all changes.
pub struct NoneCompatibility;

impl EvolutionStrategy for NoneCompatibility {
    fn compatibility_level(&self) -> CompatibilityLevel {
        CompatibilityLevel::None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestStrategy(CompatibilityLevel);

    impl EvolutionStrategy for TestStrategy {
        fn compatibility_level(&self) -> CompatibilityLevel {
            self.0
        }
    }

    #[test]
    fn test_validator_none() {
        let latest_schema = Schema::default();
        let current_schema = Schema::default();
        let strategy = TestStrategy(CompatibilityLevel::None);

        let validator = Validator::new(strategy);
        assert!(validator.check(latest_schema, current_schema).is_ok());
    }

    #[test]
    fn test_validator() {
        let latest_schema = Schema::default();
        let current_schema = Schema::default();
        let strategy = TestStrategy(CompatibilityLevel::Backward);

        let validator = Validator::new(strategy);
        assert!(validator.check(latest_schema, current_schema).is_err());
    }

}