
use serde_with::skip_serializing_none;
use serde::{Deserialize, Serialize};


/// Structure that represent top level SBE schema.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename = "messageSchema")]
pub struct Schema {
    /// The package name of the schema.
    #[serde(rename = "@package")]
    pub package: String,
    /// The id of the schema.
    #[serde(rename = "@id")]
    pub id: i32,
    /// The version of the schema.
    #[serde(rename = "@version")]
    pub version: Option<u32>,
    /// The semantic version of the schema.
    #[serde(rename = "@semanticVersion")]
    pub semantic_version: SematicVersion,
    /// The description of the schema.
    #[serde(rename = "@description")]
    pub description: String,
    /// The byte order of the bytes.
    #[serde(rename = "@byteOrder")]
    pub byte_order: Option<ByteOrder>,
    /// The include section of the schema.
    #[serde(rename = "include")]
    pub include: Option<Vec<Include>>,
    /// The types of the schema.
    #[serde(rename = "types")]
    pub types: Option<Vec<Types>>,
    /// The messages of the schema.
    #[serde(rename = "message")]
    pub messages: Option<Vec<Message>>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename = "include")]
pub struct Include {
    #[serde(rename = "@href")]
    pub href: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub desciption: Option<String>,
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "field")]
    pub fields: Option<Vec<Field>>,
    #[serde(rename = "group")]
    pub groups: Option<Vec<Group>>,
    #[serde(rename = "semanticType")]
    pub semantic_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub desciption: Option<String>,
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@dimensionType")]
    pub dimension_type: String,
    #[serde(rename = "field")]
    pub fields: Option<Vec<Field>>,
    #[serde(rename = "data")]
    pub data: Option<Vec<Data>>,
    #[serde(rename = "@sinceVersion")]
    pub since_version: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Field {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub desciption: Option<String>,
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@sinceVersion")]
    pub since_version: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub desciption: Option<String>,
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@sinceVersion")]
    pub since_version: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Types {
    #[serde(rename = "composite")]
    pub composites: Option<Vec<Composite>>,
    #[serde(rename = "enum")]
    pub enums: Option<Vec<EnumType>>,
    #[serde(rename = "set")]
    pub sets: Option<Vec<SetType>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct EnumType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@encodingType")]
    pub encoding_type: Option<PrimitiveType>,
    #[serde(rename = "validValue")]
    pub valid_values: Option<Vec<ValidValue>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct ValidValue {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "$text")]
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct SetType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@encodingType")]
    pub encoding_type: Option<PrimitiveType>,
    #[serde(rename = "choice")]
    pub choices: Option<Vec<Choice>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Choice {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "$text")]
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Composite {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub types: Option<Vec<Type>>,
    #[serde(rename = "ref")]
    pub refs: Option<Vec<Ref>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Ref {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@type")]
    pub ref_type: String,
    #[serde(rename = "@presence")]
    pub presence: Option<Presence>,
    #[serde(rename = "@valueRef")]
    pub value_ref: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Type {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@primitiveType")]
    pub primitive_type: Option<PrimitiveType>,
    #[serde(rename = "@length")]
    pub length: Option<u32>,
    #[serde(rename = "@maxValue")]
    pub max_value: Option<u32>,
    #[serde(rename = "@characterEncoding")]
    pub character_encoding: Option<Encoding>,
    #[serde(rename = "@presence")]
    pub presence: Option<Presence>,
    #[serde(rename = "@sinceVersion")]
    pub since_version: Option<u32>,
    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Encoding {
    #[serde(rename = "ASCII")]
    ASCII,
    #[serde(rename = "UTF-8")]
    UTF8,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum PrimitiveType {
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
pub enum Presence {
    Constant,
    Required,
    Optional,
}

#[derive(PartialEq, Deserialize, Serialize)]
pub struct SematicVersion(semver::Version);

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
pub enum ByteOrder {
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
            <set name="OptionalExtras" encodingType="uint8">
                <choice name="sunRoof">0</choice>
                <choice name="sportsPack">1</choice>
                <choice name="cruiseControl">2</choice>
            </set>            
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
