//! # sbe-schema
//!
//! `sbe-schema` is a library for working with SBE schema files.
//!
use serde_with::skip_serializing_none;
use serde::{Deserialize, Serialize};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename = "messageSchema")]
struct Schema {
    #[serde(rename = "@package")]
    package: String,
    #[serde(rename = "@id")]
    id: i32,
    #[serde(rename = "@version")]
    version: Option<u32>,
    #[serde(rename = "@semanticVersion")]
    semantic_version: SematicVersion,
    #[serde(rename = "@description")]
    description: String,
    #[serde(rename = "@byteOrder")]
    byte_order: Option<ByteOrder>,
    #[serde(rename = "include")]
    include: Option<Vec<Include>>,
    #[serde(rename = "types")]
    types: Option<Vec<Types>>,
    #[serde(rename = "message")]
    messages: Option<Vec<Message>>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename = "include")]
struct Include {
    #[serde(rename = "@href")]
    href: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Message {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    desciption: Option<String>,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "field")]
    fields: Option<Vec<Field>>,
    #[serde(rename = "group")]
    groups: Option<Vec<Group>>,
    #[serde(rename = "semanticType")]
    semantic_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Group {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    desciption: Option<String>,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@dimensionType")]
    dimension_type: String,
    #[serde(rename = "field")]
    fields: Option<Vec<Field>>,
    #[serde(rename = "data")]
    data: Option<Vec<Data>>,
    #[serde(rename = "@sinceVersion")]
    since_version: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Field {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    desciption: Option<String>,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@sinceVersion")]
    since_version: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Data {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    desciption: Option<String>,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@sinceVersion")]
    since_version: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Types {
    #[serde(rename = "composite")]
    composites: Option<Vec<Composite>>,
    #[serde(rename = "enum")]
    enums: Option<Vec<EnumType>>,
    #[serde(rename = "set")]
    sets: Option<Vec<SetType>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct EnumType {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "@encodingType")]
    encoding_type: Option<PrimitiveType>,
    #[serde(rename = "validValue")]
    valid_values: Option<Vec<ValidValue>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct ValidValue {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "$text")]
    value: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct SetType {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "@encodingType")]
    encoding_type: Option<PrimitiveType>,
    #[serde(rename = "choice")]
    choices: Option<Vec<Choice>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Choice {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "$text")]
    value: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Composite {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "type")]
    types: Option<Vec<Type>>,
    #[serde(rename = "ref")]
    refs: Option<Vec<Ref>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Ref {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "@type")]
    ref_type: String,
    #[serde(rename = "@presence")]
    presence: Option<Presence>,
    #[serde(rename = "@valueRef")]
    value_ref: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
struct Type {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "@primitiveType")]
    primitive_type: Option<PrimitiveType>,
    #[serde(rename = "@length")]
    length: Option<u32>,
    #[serde(rename = "@maxValue")]
    max_value: Option<u32>,
    #[serde(rename = "@characterEncoding")]
    character_encoding: Option<Encoding>,
    #[serde(rename = "@presence")]
    presence: Option<Presence>,
    #[serde(rename = "@sinceVersion")]
    since_version: Option<u32>,
    #[serde(rename = "$text")]
    value: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum Encoding {
    #[serde(rename = "ASCII")]
    ASCII,
    #[serde(rename = "UTF-8")]
    UTF8,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum PrimitiveType {
    #[serde(rename = "uint8")]
    Uint8,
    #[serde(rename = "int8")]
    Int8,
    #[serde(rename = "uint16")]
    Uint16,
    #[serde(rename = "int16")]
    Int16,
    #[serde(rename = "uint32")]
    Uint32,
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "uint64")]
    Uint64,
    #[serde(rename = "int64")]
    Int64,
    #[serde(rename = "char")]
    Char,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum Presence {
    Constant,
    Required,
    Optional,
}

#[derive(PartialEq, Deserialize, Serialize)]
struct SematicVersion(semver::Version);

impl std::fmt::Debug for SematicVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0.major, self.0.minor, self.0.patch)
    }
}

impl Default for SematicVersion {
    fn default() -> Self {
        SematicVersion(semver::Version::new(0, 0, 0))
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use quick_xml::de::from_str;

    const XML: &str = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <sbe:messageSchema xmlns:sbe="http://fixprotocol.io/2016/sbe"
                       package="since.deprecated"
                       id="876"
                       version="4"
                       semanticVersion="5.2"
                       description="Issue 876 - Test case for Deprecated messages for Java"
                       byteOrder="littleEndian">
        <types>
            <composite name="messageHeader" description="Message identifiers and length of message root">
                <type name="blockLength" primitiveType="uint16"/>
                <type name="templateId" primitiveType="uint16"/>
                <type name="schemaId" primitiveType="uint16"/>
                <type name="version" primitiveType="uint16"/>
            </composite>
        </types>
        <sbe:message name="DeprecatedMessage" id="1" semanticType="n/a" description="Message deprecated since version 3" deprecated="3">
            <field name="v1" id="1" type="uint64" />
        </sbe:message>
    </sbe:messageSchema>
"#;

    #[test]
    fn it_works() {
        let sbe: Schema = from_str(XML).expect("Failed to parse XML");
        dbg!("{:?}", &sbe);

        assert_eq!(sbe.byte_order, Some(ByteOrder::LittleEndian));
        assert_eq!(sbe.semantic_version, SematicVersion(semver::Version::from_str("5.2").unwrap()));

        let xml = quick_xml::se::to_string(&sbe).expect("Failed to serialize XML");
        dbg!("{:?}", &xml);
    }
}
