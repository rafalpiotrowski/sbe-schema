//! Full compatibility strategy
//!
//! Allowed:
//! - Add optional fields to messages
//! - Delete optional fields
//!

use crate::{CompatibilityLevel, EvolutionStrategy};

use super::{EvolutionError, SchemaValidator};

/// Schema evolution strategy that accept only adding optional fields.
pub struct FullCompatibility<V: SchemaValidator> {
    validator: V,
}

impl<V: SchemaValidator> FullCompatibility<V> {
    /// Create a new `FullCompatibility` strategy with the given validator.
    pub fn new(validator: V) -> Self {
        Self { validator }
    }
}

impl<V: SchemaValidator> EvolutionStrategy for FullCompatibility<V> {
    type SchemaType = V::SchemaType;

    fn compatibility_level(&self) -> CompatibilityLevel {
        CompatibilityLevel::Full
    }

    fn check(&self) -> Result<CompatibilityLevel, EvolutionError> {
        let r = self
            .validator
            .compare_version()?;
        if r == self.compatibility_level() {
            Ok(r)
        } else {
            Err(EvolutionError::SchemaNotCompatible(
                self.compatibility_level(),
            ))
        }
    }
}
