//! Regression coverage for the original public schema and strategy APIs.
#![allow(unused_crate_dependencies)]

use sbe_schema::{
	build_vtable, CompatibilityLevel, EvolutionStrategy, FullCompatibility, NoneCompatibility,
	Optional, PartialCompatibility, SbeSchemaValidator, Schema, SchemaValidator, VTable, Validator,
};

const XML: &str = r#"<messageSchema package="contracts" id="42" version="1" semanticVersion="5.2">
<types><composite name="messageHeader">
<type name="blockLength" primitiveType="uint16"/>
<type name="templateId" primitiveType="uint16"/>
<type name="schemaId" primitiveType="uint16"/>
<type name="version" primitiveType="uint16"/>
</composite></types>
<message name="Order" id="1"><field name="quantity" id="1" type="uint32"/></message>
</messageSchema>"#;

#[test]
fn schema_still_supports_serde_typed_mutation_and_xml_output() {
	fn serde_contract<T: serde::Serialize + for<'de> serde::Deserialize<'de> + Default>() {}
	serde_contract::<Schema>();
	let mut schema: Schema = quick_xml::de::from_str(XML).unwrap();
	assert_eq!(format!("{:?}", schema.semantic_version), "Some(5.2.0)");
	schema.version = 2;
	schema.package = Some("edited".into());
	schema.messages.as_mut().unwrap()[0].fields.as_mut().unwrap()[0].r#type = "uint64".into();
	let output = quick_xml::se::to_string(&schema).unwrap();
	let restored: Schema = quick_xml::de::from_str(&output).unwrap();
	assert_eq!(schema, restored);
	assert_eq!(restored.version, 2);
	assert_eq!(restored.messages.unwrap()[0].fields.as_ref().unwrap()[0].r#type, "uint64");
}

#[test]
fn default_schema_result_error_and_vtable_interfaces_remain_available() {
	let schema = Schema::default();
	let result: sbe_schema::Result<()> = Ok(());
	let _: Option<sbe_schema::SbeSchemaError> = None;
	assert!(result.is_ok());
	assert_eq!(build_vtable(&schema), VTable::new());
	let populated: Schema = quick_xml::de::from_str(XML).unwrap();
	assert!(populated.message_header().is_some());
	assert_ne!(build_vtable(&populated), VTable::new());
}

#[test]
fn original_validator_and_strategy_wrappers_remain_usable() {
	let baseline = Schema::default();
	let candidate = Schema { version: 1, ..Schema::default() };
	let schema_validator = SbeSchemaValidator::new(&baseline, &candidate);
	assert_eq!(schema_validator.latest().version, 0);
	assert_eq!(schema_validator.current().version, 1);
	let full = FullCompatibility::new(schema_validator);
	assert_eq!(full.compatibility_level(), CompatibilityLevel::Full);
	assert_eq!(Validator::new(full).check().unwrap(), CompatibilityLevel::Full);
	let none = NoneCompatibility::new(SbeSchemaValidator::new(&baseline, &candidate));
	assert_eq!(Validator::new(none).check().unwrap(), CompatibilityLevel::None);
	let _ = CompatibilityLevel::NoChange;
}

#[test]
fn optional_and_partial_compatibility_extension_traits_remain_available() {
	struct Example;
	impl Optional for Example {
		fn is_optional(&self) -> bool {
			true
		}
	}
	impl PartialCompatibility for Example {
		fn partial_compatibility(&self, _: &Self) -> CompatibilityLevel {
			CompatibilityLevel::NoChange
		}
	}
	assert!(Example.is_optional());
	assert_eq!(Example.partial_compatibility(&Example), CompatibilityLevel::NoChange);
}
