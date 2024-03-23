use crate::{CompatibilityLevel, EvolutionStrategy, Schema};

use super::EvolutionError;

/// Schema evolution strategy that accept only adding optional fields.
pub struct FullCompatibility;

impl EvolutionStrategy for FullCompatibility {
    fn compatibility_level(&self) -> CompatibilityLevel {
        CompatibilityLevel::Full
    }

    fn check(&self, _latest_schema: Schema, _current_schema: Schema) -> Result<(), EvolutionError> {
        Err(EvolutionError::SchemaNotCompatible(self.compatibility_level()))
    }
}