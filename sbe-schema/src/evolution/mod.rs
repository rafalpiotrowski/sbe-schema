//!
//!

mod backward;
mod forward;
mod full;
mod validator;

use thiserror::Error;

pub use full::FullCompatibility;
pub use validator::SbeSchemaValidator;

/// The compatibility level of a schema evolution strategy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompatibilityLevel {
    /// Delete fields and Add optional fields
    Backward,
    /// Add fields and Delete optional fields
    Forward,
    /// Add optional fields and Delete optional fields
    Full,
    /// All changes are accepted
    None,
    /// No changes are accepted
    NoChange,
}

/// A trait for checking partial compatibility.
pub trait PartialCompatibility<Rhs = Self> 
where
    Rhs: ?Sized,
{
    /// Check the partial compatibility. First argument beeing current version and second argument beeing the latest version.
    fn partial_compatibility(&self, latest: &Rhs) -> CompatibilityLevel;
}

#[derive(Error, Debug)]
pub enum EvolutionError {
    #[error("Schema is not compatible with the latest schema! Compatibility level: {0:?}")]
    SchemaNotCompatible(CompatibilityLevel),
    #[error("Missing schema version")]
    MissingVersion,    
    #[error("Missing message header")]
    MissingMessageHeader,
}

/// A strategy for schema evolution.
pub trait EvolutionStrategy {
    /// The type of the schema we will be working against
    type SchemaType;
    /// The compatibility level of the strategy.
    fn compatibility_level(&self) -> CompatibilityLevel;
    /// Check if the current schema is compatible with the latest schema.
    fn check(&self) -> Result<CompatibilityLevel, EvolutionError>;
}

/// A trait for validating schema versions.
pub trait SchemaValidator {
    /// The type of the schema we will be working against
    type SchemaType;
    /// Get the latest schema.
    fn latest(&self) -> &Self::SchemaType;
    /// Get the current schema.
    fn current(&self) -> &Self::SchemaType;
    /// Compare the version of the current schema with the latest schema.
    fn compare_version(&self) -> Result<CompatibilityLevel, EvolutionError>;
    /// Compare the message header of the current schema with the latest schema.
    fn compare_message_header(&self) -> Result<CompatibilityLevel, EvolutionError>;
    /// Compare the types of the current schema with the latest schema.
    fn compare_types(&self) -> Result<CompatibilityLevel, EvolutionError>;
    /// Compare the messages of the current schema with the latest schema.
    fn compare_messages(&self) -> Result<CompatibilityLevel, EvolutionError>;
}

/// A validator for schema evolution.
pub struct Validator<E: EvolutionStrategy> {
    strategy: E,
}

// Implement the Validator struct
impl<E: EvolutionStrategy> Validator<E> {
    /// Create a new validator with the given strategy.
    pub fn new(strategy: E) -> Self {
        Self { strategy }
    }

    /// Check if the current schema is compatible with the latest schema.
    pub fn check(&self) -> Result<CompatibilityLevel, EvolutionError> {
        self.strategy.check()
    }
}

/// A strategy that accepts all changes.
pub struct NoneCompatibility<V: SchemaValidator> {
    _validator: V,
}

impl<V: SchemaValidator> NoneCompatibility<V> {
    /// Create a new `NoneCompatibility` strategy with the given validator.
    pub fn new(validator: V) -> Self {
        Self {
            _validator: validator,
        }
    }
}

impl<V: SchemaValidator> EvolutionStrategy for NoneCompatibility<V> {
    type SchemaType = V::SchemaType;

    fn compatibility_level(&self) -> CompatibilityLevel {
        CompatibilityLevel::None
    }

    fn check(&self) -> Result<CompatibilityLevel, EvolutionError> {
        match self.compatibility_level() {
            CompatibilityLevel::None => Ok(CompatibilityLevel::None),
            _ => Err(EvolutionError::SchemaNotCompatible(
                self.compatibility_level(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use tests::validator::SbeSchemaValidator;

    use crate::Schema;

    use super::*;

    #[test]
    fn test_validator_none() {
        let latest_schema = Schema::default();
        let current_schema = Schema::default();
        let strategy = NoneCompatibility {
            _validator: SbeSchemaValidator::new(&latest_schema, &current_schema),
        };

        let validator = Validator::new(strategy);
        let result = validator.check();
        let expected = CompatibilityLevel::None;
        assert!(result.is_ok());
        let returnd_result = result.unwrap();
        assert_eq!(returnd_result, expected);
    }
}
