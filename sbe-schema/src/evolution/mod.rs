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
    /// The type of the schema we will be working against
    type SchemaType;
    /// The compatibility level of the strategy.
    fn compatibility_level(&self) -> CompatibilityLevel;
    /// Check if the current schema is compatible with the latest schema.
    fn check(&self, _latest_schema: Self::SchemaType, _current_schema: Self::SchemaType) -> Result<(), EvolutionError>
    {
        match self.compatibility_level() {
            CompatibilityLevel::None => Ok(()),
            _ => Err(EvolutionError::SchemaNotCompatible(self.compatibility_level()))
        }
    }
}

/// A trait for validating schema versions.
pub trait SchemaValidator {
    /// The type of the schema we will be working against
    type SchemaType;
    /// Compare the version of the current schema with the latest schema.
    fn compare_version(&self, _: &Self::SchemaType, _: &Self::SchemaType) -> Result<i8, EvolutionError>;
}

/// A validator for SBE schema versions.
pub struct SbeSchemaValidator;

impl SchemaValidator for SbeSchemaValidator {
    type SchemaType = Schema;

    fn compare_version(&self, latest: &Self::SchemaType, current: &Self::SchemaType) -> Result<i8, EvolutionError> {
        if current.version > latest.version {
            return Ok(1);
        } else if current.version < latest.version {
            return Ok(-1);
        } else {
            return Ok(0);
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
            strategy
        }
    }

    /// Check if the current schema is compatible with the latest schema.
    pub fn check(&self, latest_schema: E::SchemaType, current_schema: E::SchemaType) -> Result<(), EvolutionError> {
        self.strategy.check(latest_schema, current_schema)
    }
}

/// A strategy that accepts all changes.
pub struct NoneCompatibility<V:SchemaValidator> {
    _validator: V
}

impl<V:SchemaValidator> NoneCompatibility<V> {
    /// Create a new `NoneCompatibility` strategy with the given validator.
    pub fn new(validator: V) -> Self {
        Self {
            _validator: validator
        }
    }
}

impl<V:SchemaValidator> EvolutionStrategy for NoneCompatibility<V> {
    type SchemaType = V::SchemaType;

    fn compatibility_level(&self) -> CompatibilityLevel {
        CompatibilityLevel::None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestStrategy<V:SchemaValidator> {
        compatibility_level: CompatibilityLevel,
        _validator: V,
    }

    impl<V:SchemaValidator> EvolutionStrategy for TestStrategy<V> {
        type SchemaType = Schema;
        fn compatibility_level(&self) -> CompatibilityLevel {
            self.compatibility_level
        }
    }

    #[test]
    fn test_validator_none() {
        let latest_schema = Schema::default();
        let current_schema = Schema::default();
        let strategy = TestStrategy { 
            compatibility_level: CompatibilityLevel::None,
            _validator: SbeSchemaValidator {},
        };

        let validator = Validator::new(strategy);
        assert!(validator.check(latest_schema, current_schema).is_ok());
    }

    #[test]
    fn test_validator() {
        let latest_schema = Schema::default();
        let current_schema = Schema::default();
        let strategy = TestStrategy { 
            compatibility_level: CompatibilityLevel::Full,
            _validator: SbeSchemaValidator {},
        };

        let validator = Validator::new(strategy);
        assert!(validator.check(latest_schema, current_schema).is_err());
    }

}