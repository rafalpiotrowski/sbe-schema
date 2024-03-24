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

    fn compare_message_header(&self) -> Result<CompatibilityLevel, EvolutionError> {
        let latest = self.latest().message_header();
        let current = self.current().message_header();

        match (latest, current) {
            (Some(latest), Some(current)) => {
                let latest_types_len = latest.types.as_ref().and_then(|x| Some(x.len())).unwrap_or(0);
                let current_types_len = current.types.as_ref().and_then(|x| Some(x.len())).unwrap_or(0);
                let latest_refs_len = latest.refs.as_ref().and_then(|x| Some(x.len())).unwrap_or(0);
                let current_refs_len = current.refs.as_ref().and_then(|x| Some(x.len())).unwrap_or(0);

                // check if we have the same number of types and refs
                if latest_types_len == current_types_len
                {
                    // now we need to check if they are they same
                } else {
                    // we have a different number of types or refs
                    // if current has more types or refs, we are good for full compatibility
                }
                Ok(CompatibilityLevel::NoChange)
            }
            _ => Err(EvolutionError::MissingMessageHeader),
        }
    }

    fn compare_types(&self) -> Result<CompatibilityLevel, EvolutionError> {
        panic!("Not implemented")
    }    

    fn compare_messages(&self) -> Result<CompatibilityLevel, EvolutionError> {
        panic!("Not implemented")
    }
}
