//! SBE Schema Validator

use core::panic;

use crate::{
	evolution::check_vec,
	types::{Composite, EnumType, SetType, Types},
	CompatibilityLevel, EvolutionError, PartialCompatibility, Schema, SchemaValidator,
};

/// A validator for SBE schema versions.
pub struct SbeSchemaValidator<'a> {
	latest_schema: &'a Schema,
	current_schema: &'a Schema,
	// latest_vtable: VTable<'a>,
	// current_vtable: VTable<'a>,
}

impl<'a> SbeSchemaValidator<'a> {
	/// Create a new `SbeSchemaValidator` with the given schemas.
	pub fn new(latest_schema: &'a Schema, current_schema: &'a Schema) -> Self {
		// let latest_vtable = crate::build_vtable(latest_schema);
		// let current_vtable = crate::build_vtable(latest_schema);

		Self {
			latest_schema,
			current_schema,
			// latest_vtable,
			// current_vtable,
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
			(latest, current) => {
				if current == latest {
					Ok(CompatibilityLevel::NoChange)
				} else if current > latest {
					if current - latest == 1 {
						Ok(CompatibilityLevel::Full)
					} else {
						Ok(CompatibilityLevel::None)
					}
				} else {
					// current < latest
					Ok(CompatibilityLevel::None)
				}
			},
		}
	}

	fn compare_message_header(&self) -> Result<CompatibilityLevel, EvolutionError> {
		let latest = self.latest().message_header();
		let current = self.current().message_header();

		match (latest, current) {
			(Some(latest), Some(current)) => {
				Ok(PartialCompatibility::partial_compatibility(current, latest))
			},
			// SBE requires message header to be defined
			_ => Err(EvolutionError::MissingMessageHeader),
		}
	}

	fn compare_types(&self) -> Result<CompatibilityLevel, EvolutionError> {
		let latest_flat_types = flat_types(self.latest().types.as_ref());
		let current_flat_types = flat_types(self.current().types.as_ref());

		let composites = check_vec::<&Composite>(
			latest_flat_types.composites.as_ref(),
			current_flat_types.composites.as_ref(),
		);

		if composites == CompatibilityLevel::None {
			return Ok(CompatibilityLevel::None);
		}

		let enums = check_vec::<&EnumType>(
			latest_flat_types.enums.as_ref(),
			current_flat_types.enums.as_ref(),
		);

		if enums == CompatibilityLevel::None {
			return Ok(CompatibilityLevel::None);
		}

		let sets = check_vec::<&SetType>(
			latest_flat_types.sets.as_ref(),
			current_flat_types.sets.as_ref(),
		);

		if sets == CompatibilityLevel::None {
			return Ok(CompatibilityLevel::None);
		}

		match (composites, enums, sets) {
			(CompatibilityLevel::None, _, _) => Ok(CompatibilityLevel::None),
			(_, CompatibilityLevel::None, _) => Ok(CompatibilityLevel::None),
			(_, _, CompatibilityLevel::None) => Ok(CompatibilityLevel::None),
			(CompatibilityLevel::Backward, _, _) => Ok(CompatibilityLevel::Backward),
			(_, CompatibilityLevel::Backward, _) => Ok(CompatibilityLevel::Backward),
			(_, _, CompatibilityLevel::Backward) => Ok(CompatibilityLevel::Backward),
			(CompatibilityLevel::Forward, _, _) => Ok(CompatibilityLevel::Forward),
			(_, CompatibilityLevel::Forward, _) => Ok(CompatibilityLevel::Forward),
			(_, _, CompatibilityLevel::Forward) => Ok(CompatibilityLevel::Forward),
			(CompatibilityLevel::Full, _, _) => Ok(CompatibilityLevel::Full),
			(_, CompatibilityLevel::Full, _) => Ok(CompatibilityLevel::Full),
			(_, _, CompatibilityLevel::Full) => Ok(CompatibilityLevel::Full),
			(
				CompatibilityLevel::NoChange,
				CompatibilityLevel::NoChange,
				CompatibilityLevel::NoChange,
			) => Ok(CompatibilityLevel::NoChange),
		}
	}

	fn compare_messages(&self) -> Result<CompatibilityLevel, EvolutionError> {
		panic!("Not implemented")
	}
}

fn flat_types(types: Option<&Vec<Types>>) -> FlatTypes {
	let mut flat_types = FlatTypes { enums: None, composites: None, sets: None };

	if let Some(types) = types {
		flat_types.composites = Some(
			types
				.iter()
				.filter(|t| t.composites.is_some())
				.flat_map(|t| t.composites.as_ref().unwrap())
				.filter(|c| c.name != "messageHeader")
				.collect::<Vec<_>>(),
		);
		flat_types.enums = Some(
			types
				.iter()
				.filter(|t| t.enums.is_some())
				.flat_map(|t| t.enums.as_ref().unwrap())
				.collect::<Vec<_>>(),
		);
		flat_types.sets = Some(
			types
				.iter()
				.filter(|t| t.sets.is_some())
				.flat_map(|t| t.sets.as_ref().unwrap())
				.collect::<Vec<_>>(),
		);
	}

	flat_types
}

/// SBE allows to have multiple <types> secions in the schema file.
/// this structure is to flatten the types into a single structure to make it easier to compare.
struct FlatTypes<'a> {
	enums: Option<Vec<&'a EnumType>>,
	composites: Option<Vec<&'a Composite>>,
	sets: Option<Vec<&'a SetType>>,
}
