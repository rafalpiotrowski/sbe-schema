//! # sbe-schema
//! 
//! `sbe-schema` is a library for working with SBE schema files.
//! 
//! ## Example
//! 
//! ```rust
//! use sbe_schema::add;
//! 
//! let result = add(2, 2);
//! assert_eq!(result, 4);
//! ```
//! 
use serde::Deserialize;
use serde_semver::SemverReq;

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "sbe:messageSchema")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "xi:include")]
struct Include {
    #[serde(rename = "@href")]
    href: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "sbe:message")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "group")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "field")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "data")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "types")]
struct Types {
    #[serde(rename = "composite")]
    composites: Option<Vec<Composite>>,
    #[serde(rename = "enum")]
    enums: Option<Vec<EnumType>>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "enum")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValidValue {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@description")]
    description: Option<String>,
    #[serde(rename = "$text")]
    value: String,
}


#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "composite")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
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
}

#[derive(Debug, PartialEq, Deserialize)]
enum Encoding {
    #[serde(rename = "ASCII")]
    ASCII,
    #[serde(rename = "UTF-8")]
    UTF8,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Presence {
    Constant,
    Required,
    Optional,
}

#[derive(SemverReq, PartialEq, Default, )]
#[version("5.2.0")]
struct SematicVersion;

impl std::fmt::Debug for SematicVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SematicVersion::version())
    }

}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    const XML: &str = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <sbe:messageSchema xmlns:sbe="http://fixprotocol.io/2016/sbe"
                       package="since.deprecated"
                       id="876"
                       version="4"
                       semanticVersion="5.2.0"
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
    }
}
