//! SBE Schema Validator

use crate::{types::{Composite, Ref, Type, Presence}, CompatibilityLevel, EvolutionError, PartialCompatibility, Schema, SchemaValidator, VTable};

/// A validator for SBE schema versions.
pub struct SbeSchemaValidator<'a>
{
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
            current_vtable
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

        let latest_vtable = &self.latest_vtable;
        let current_vtable = &self.current_vtable;

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


impl PartialCompatibility for Composite {
    fn partial_compatibility(&self, latest: &Self) -> CompatibilityLevel {
        if self.name != latest.name {
            // early return if the name is different
            return CompatibilityLevel::None
        } 

        let mut level = CompatibilityLevel::None;

        if self.types == latest.types {       
            level = CompatibilityLevel::NoChange;
        }

        if level != CompatibilityLevel::NoChange {
            return level
        }

        level = check_types(self.types.as_ref(), latest.types.as_ref());
        let ref_level = match level {
            CompatibilityLevel::None => return level,
            _ => {
                if self.refs == latest.refs {
                    CompatibilityLevel::NoChange
                } else {
                    check_refs(self.refs.as_ref(), latest.refs.as_ref())
                }
            }
        };

        match (level, ref_level) {
            (CompatibilityLevel::None, _) => CompatibilityLevel::None, 
            (_, CompatibilityLevel::None) => CompatibilityLevel::None,
            (CompatibilityLevel::NoChange, l) => l,
            (l, CompatibilityLevel::NoChange) => l,
            (CompatibilityLevel::Full, _) => ref_level,
            (_, CompatibilityLevel::Full) => level,
            (_, CompatibilityLevel::Backward) => CompatibilityLevel::Backward,
            (CompatibilityLevel::Backward, _) => CompatibilityLevel::Backward,
            (CompatibilityLevel::Forward, CompatibilityLevel::Forward) => CompatibilityLevel::Forward,
        }
    }
}

fn check_types(current: Option<&Vec<Type>>, latest: Option<&Vec<Type>>) -> CompatibilityLevel {
    match (current, latest) {
        (Some(current_types), Some(latest_types)) => {
            // if vecs are the same, we are good for no change
            if current_types == latest_types {
                return CompatibilityLevel::NoChange
            }
            // if current has more types than latest, but all latest are present in current
            // then we are good for full compatibility
            // 1. let check if all latest types are present in current
            let mut level = CompatibilityLevel::NoChange;
            let mut found = 0;
            for latest_type in latest_types {
                if current_types.contains(latest_type) {
                    found += 1;
                    // we found the type, now we just need to update the compatibility level
                    // against the current level so we return lowest compatibility level
                    level = get_compatibility_from_current_and_new_level(level, CompatibilityLevel::NoChange);
                } else { 
                    // not found, we should check if the field was optional
                    // if it was optional, then Forward compatibility is allowed
                    // if it was not optional, then we are backward compatible
                    match latest_type.presence.as_ref() {
                        Some(presence) => {
                            level = get_compatibility_from_current_and_deleted_field_optionality(level, *presence == crate::types::Presence::Optional);
                        },
                        None => { // we treat it as required
                            level = get_compatibility_from_current_and_new_level(level, CompatibilityLevel::Forward);
                        }
                    }
                }
            }

            // 2. if we have the same number of types, we return NoChange
            if found == current_types.len() {
                // no change
                return CompatibilityLevel::NoChange;
            }

            // 3. if we found less types, so some types were removed
            // and we already checked if the types were optional
            // therefore level is properly set, so we return level
            if found < latest_types.len() {
                return level;
            }

            // 3. we have more types then last time
            // all latest types are present in current with no changes, therefore we just need to assert that
            // all new types are optional to have full compatibility otherwise we are forward compatible
            for current_type in current_types {
                if latest_types.contains(current_type) {
                    continue;
                }
                match current_type.presence.as_ref() {
                    Some(presence) => {
                        level = get_compatibility_from_current_and_added_field_optionality(level, *presence == crate::types::Presence::Optional);
                    },
                    None => { // we treat it as required
                        level = get_compatibility_from_current_and_new_level(level, CompatibilityLevel::Forward);
                    }
                }
            }

            level
        },
        // check if the added types are optional. If they are optional then we are backward compatible
        // otherwise we are forward compatible
        (Some(types), None) => {
            types.into_iter().all(
                |t| t.presence.is_some() && *t.presence.as_ref().unwrap() == Presence::Optional)
                .then(|| CompatibilityLevel::Backward)
                .unwrap_or(CompatibilityLevel::Forward)
        },
        // check if the removed types were optional. If they were optional then we are Forward compatible,
        // otherwise Backward
        (None, Some(types)) => {
            types.into_iter().all(
                |t| t.presence.is_some() && *t.presence.as_ref().unwrap() == Presence::Optional)
                .then(|| CompatibilityLevel::Forward)
                .unwrap_or(CompatibilityLevel::Backward)
        },
        (None, None) => CompatibilityLevel::NoChange,
    }
}

fn get_compatibility_from_current_and_new_level(level: CompatibilityLevel, new_level: CompatibilityLevel) -> CompatibilityLevel {
    match (level, new_level) {
        (CompatibilityLevel::None, _) => CompatibilityLevel::None,
        (_, CompatibilityLevel::None) => CompatibilityLevel::None,
        (CompatibilityLevel::NoChange, _) => new_level,
        (_, CompatibilityLevel::NoChange) => level,
        (CompatibilityLevel::Full, _) => new_level,
        (_, CompatibilityLevel::Full) => level,
        (CompatibilityLevel::Backward, _) => CompatibilityLevel::Backward,
        (_, CompatibilityLevel::Backward) => CompatibilityLevel::Backward,
        (CompatibilityLevel::Forward, CompatibilityLevel::Forward) => CompatibilityLevel::Forward,
    }
}

fn get_compatibility_from_current_and_deleted_field_optionality(level: CompatibilityLevel, is_optional: bool) -> CompatibilityLevel {
    match (level, is_optional) {
        (CompatibilityLevel::None, _) => CompatibilityLevel::None,
        (_, false) => CompatibilityLevel::Backward,
        (_, true) => CompatibilityLevel::Forward,
    }
}

fn get_compatibility_from_current_and_added_field_optionality(level: CompatibilityLevel, is_optional: bool) -> CompatibilityLevel {
    match (level, is_optional) {
        (CompatibilityLevel::None, _) => CompatibilityLevel::None,
        (_, false) => CompatibilityLevel::Forward,
        (_, true) => CompatibilityLevel::Full,
    }
}

fn check_refs(current: Option<&Vec<Ref>>, latest: Option<&Vec<Ref>>) -> CompatibilityLevel {
    CompatibilityLevel::None
}
