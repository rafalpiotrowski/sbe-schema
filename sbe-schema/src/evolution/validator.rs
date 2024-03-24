//! SBE Schema Validator

use crate::{CompatibilityLevel, EvolutionError, Schema, SchemaValidator};

/// A validator for SBE schema versions.
pub struct SbeSchemaValidator
{
    latest_schema: Schema,
    current_schema: Schema,
}

impl SbeSchemaValidator {
    /// Create a new `SbeSchemaValidator` with the given schemas.
    pub fn new(latest_schema: Schema, current_schema: Schema) -> Self {
        Self {
            latest_schema,
            current_schema,
        }
    }
}


impl SchemaValidator for SbeSchemaValidator {
    type SchemaType = Schema;

    fn latest(&self) -> &Self::SchemaType {
        &self.latest_schema
    }

    fn current(&self) -> &Self::SchemaType {
        &self.current_schema
    }

    fn compare_version(&self) -> Result<CompatibilityLevel, EvolutionError> {
        match (self.latest().version, self.current().version) {
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
