//! SBE Schema Validator

use crate::{CompatibilityLevel, EvolutionError, Schema, SchemaValidator};

/// A validator for SBE schema versions.
pub struct SbeSchemaValidator;

impl SchemaValidator for SbeSchemaValidator {
    type SchemaType = Schema;

    fn compare_version(
        &self,
        latest_schema: &Self::SchemaType,
        current_schema: &Self::SchemaType,
    ) -> Result<CompatibilityLevel, EvolutionError> {
        match (latest_schema.version, current_schema.version) {
            (Some(latest), Some(current)) => {
                if current == latest {
                    Ok(CompatibilityLevel::NoChange)
                } else if current > latest {
                    if current - latest == 1 {
                        Ok(CompatibilityLevel::Forward)
                    } else {
                        Ok(CompatibilityLevel::None)
                    }
                } else {
                    // current < latest
                    Ok(CompatibilityLevel::None)
                }
            }
            _ => Err(EvolutionError::MissingVersion),
        }
    }
}
