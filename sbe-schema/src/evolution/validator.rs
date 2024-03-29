//! SBE Schema Validator

use crate::{
    CompatibilityLevel, EvolutionError, PartialCompatibility, Schema, SchemaValidator, VTable,
};

/// A validator for SBE schema versions.
pub struct SbeSchemaValidator<'a> {
    latest_schema: &'a Schema,
    current_schema: &'a Schema,
    latest_vtable: VTable<'a>,
    current_vtable: VTable<'a>,
}

impl<'a> SbeSchemaValidator<'a> {
    /// Create a new `SbeSchemaValidator` with the given schemas.
    pub fn new(latest_schema: &'a Schema, current_schema: &'a Schema) -> Self {
        let latest_vtable = crate::build_vtable(latest_schema);
        let current_vtable = crate::build_vtable(latest_schema);

        Self {
            latest_schema,
            current_schema,
            latest_vtable,
            current_vtable,
        }
    }
}

impl<'a> SchemaValidator for SbeSchemaValidator<'a> {
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
                Ok(PartialCompatibility::partial_compatibility(current, latest))
            }
            // SBE requires message header to be defined
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
