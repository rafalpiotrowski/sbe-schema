use crate::{CompatibilityLevel, EvolutionStrategy};

use super::{EvolutionError, SchemaValidator};

/// Schema evolution strategy that accept only adding optional fields.
pub struct FullCompatibility<V: SchemaValidator> {
    validator: V,
}

impl <V: SchemaValidator> FullCompatibility<V> {
    /// Create a new `FullCompatibility` strategy with the given validator.
    pub fn new(validator: V) -> Self {
        Self {
            validator
        }
    }
}

impl<V: SchemaValidator> EvolutionStrategy for FullCompatibility<V> {
    type SchemaType = V::SchemaType;

    fn compatibility_level(&self) -> CompatibilityLevel {
        CompatibilityLevel::Full
    }

    fn check(&self, latest_schema: Self::SchemaType, current_schema: Self::SchemaType) -> Result<(), EvolutionError> {
        let r = self.validator.compare_version(&latest_schema, &current_schema)?;
        if r <= 0 {
            return Err(EvolutionError::SchemaNotCompatible(self.compatibility_level()))
        }
        Ok(())
    }
}