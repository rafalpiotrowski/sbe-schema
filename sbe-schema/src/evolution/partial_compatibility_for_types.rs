use crate::{
    types::{Composite, Ref, Type},
    CompatibilityLevel, Optional, PartialCompatibility,
};

impl PartialCompatibility for Composite {
    fn partial_compatibility(&self, latest: &Self) -> CompatibilityLevel {
        if self.name != latest.name {
            // early return if the name is different
            return CompatibilityLevel::None;
        }

        let mut level = CompatibilityLevel::None;

        if self.types == latest.types {
            level = CompatibilityLevel::NoChange;
        }

        if level != CompatibilityLevel::NoChange {
            return level;
        }

        level = check_vec::<Type>(self.types.as_ref(), latest.types.as_ref());
        let ref_level = match level {
            CompatibilityLevel::None => return level,
            _ => {
                if self.refs == latest.refs {
                    CompatibilityLevel::NoChange
                } else {
                    check_vec::<Ref>(self.refs.as_ref(), latest.refs.as_ref())
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
            (CompatibilityLevel::Forward, CompatibilityLevel::Forward) => {
                CompatibilityLevel::Forward
            }
        }
    }
}

/// Check the compatibility of two vectors of types.
pub fn check_vec<T>(current: Option<&Vec<T>>, latest: Option<&Vec<T>>) -> CompatibilityLevel
where
    T: PartialEq + Optional,
{
    match (current, latest) {
        (Some(current), Some(latest)) => {
            // if vecs are the same, we are good for no change
            if current == latest {
                return CompatibilityLevel::NoChange;
            }
            // if current has more types than latest, but all latest are present in current
            // then we are good for full compatibility
            // 1. let check if all latest types are present in current
            let mut level = CompatibilityLevel::NoChange;
            let mut found = 0;
            for latest_item in latest {
                if current.contains(latest_item) {
                    found += 1;
                    // we found the type, now we just need to update the compatibility level
                    // against the current level so we return lowest compatibility level
                    level = get_compatibility_from_current_and_new_level(
                        level,
                        CompatibilityLevel::NoChange,
                    );
                } else {
                    // not found, we should check if the field was optional
                    // if it was optional, then Forward compatibility is allowed
                    // if it was not optional, then we are backward compatible
                    level = get_compatibility_from_current_level_and_deleted_field_optionality(
                        level,
                        latest_item.is_optional(),
                    );
                }
            }

            // 2. if we have the same number of types, we return NoChange
            if found == current.len() {
                // no change
                return CompatibilityLevel::NoChange;
            }

            // 3. if we found less types, so some types were removed
            // and we already checked if the types were optional
            // therefore level is properly set, so we return level
            if found < latest.len() {
                return level;
            }

            // 3. we have more types then last time
            // all latest types are present in current with no changes, therefore we just need to assert that
            // all new types are optional to have full compatibility otherwise we are forward compatible
            for current_item in current {
                if latest.contains(current_item) {
                    continue;
                }
                level = get_compatibility_from_current_level_and_added_field_optionality(
                    level,
                    current_item.is_optional(),
                );
            }

            level
        }
        // check if the added types are optional. If they are optional then we are backward compatible
        // otherwise we are forward compatible
        (Some(current_items), None) => current_items
            .into_iter()
            .all(|t| t.is_optional())
            .then(|| CompatibilityLevel::Backward)
            .unwrap_or(CompatibilityLevel::Forward),
        // check if the removed types were optional. If they were optional then we are Forward compatible,
        // otherwise Backward
        (None, Some(latest_items)) => latest_items
            .into_iter()
            .all(|t| t.is_optional())
            .then(|| CompatibilityLevel::Forward)
            .unwrap_or(CompatibilityLevel::Backward),
        (None, None) => CompatibilityLevel::NoChange,
    }
}

fn get_compatibility_from_current_and_new_level(
    level: CompatibilityLevel,
    new_level: CompatibilityLevel,
) -> CompatibilityLevel {
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

fn get_compatibility_from_current_level_and_deleted_field_optionality(
    level: CompatibilityLevel,
    is_optional: bool,
) -> CompatibilityLevel {
    match (level, is_optional) {
        (CompatibilityLevel::None, _) => CompatibilityLevel::None,
        (_, false) => CompatibilityLevel::Backward,
        (_, true) => CompatibilityLevel::Forward,
    }
}

fn get_compatibility_from_current_level_and_added_field_optionality(
    level: CompatibilityLevel,
    is_optional: bool,
) -> CompatibilityLevel {
    match (level, is_optional) {
        (CompatibilityLevel::None, _) => CompatibilityLevel::None,
        (_, false) => CompatibilityLevel::Forward,
        (_, true) => CompatibilityLevel::Full,
    }
}
