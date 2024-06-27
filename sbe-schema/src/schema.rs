use std::path::PathBuf;

use crate::{types::Schema, SbeSchemaError};
use quick_xml::de::from_str;

impl TryFrom<PathBuf> for Schema {
	type Error = SbeSchemaError;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		let xml = std::fs::read_to_string(path).expect("Failed to read file");
		let schema: Schema = from_str(&xml).expect("Failed to parse XML");
		Ok(schema)
	}
}
