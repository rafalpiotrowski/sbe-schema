use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Structure that represent top level SBE schema.
#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        self.package == other.package
            && self.id == other.id
            && self.version == other.version
            && self.semantic_version == other.semantic_version
            && self.byte_order == other.byte_order
            && self.types == other.types
            && self.messages == other.messages
    }
}

impl Eq for Schema {}

impl Hash for Schema {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.package.hash(state);
        self.id.hash(state);
        self.version.hash(state);
        self.semantic_version.hash(state);
        if self.byte_order.is_some() {
            self.byte_order.hash(state);
        }
        self.types.hash(state);
        self.messages.hash(state);
    }
}

const MESSAGE_HEADER: &str = "messageHeader";

impl Schema {
    /// Get the message header composite type.
    pub fn message_header(&self) -> Option<&Composite> {
        self.types.as_ref().and_then(|types| {
            types.iter().find_map(|t| {
                t.composites
                    .as_ref()
                    .iter()
                    .find_map(|c| c.iter().find(|c| c.name == MESSAGE_HEADER))
            })
        })
    }
}

/// build vtable for lookups
/// type name -> composite/enum/set/type
pub fn build_vtable(schema: &Schema) -> VTable {
    let mut vtable = VTable::new();
    if let Some(types) = schema.types.as_ref() {
        for t in types {
            if let Some(composites) = t.composites.as_ref() {
                for c in composites {
                    vtable.add(c.name.clone(), VTableObject::Composite(c));
                }
            }
        }
    }
    vtable
}

#[derive(Debug, PartialEq)]
pub enum VTableObject<'a> {
    Composite(&'a Composite),
    Enum(&'a EnumType),
    Set(&'a SetType),
    Type(&'a Type),
    Message(&'a Message),
}

///
#[derive(Debug, Default, PartialEq)]
pub struct VTable<'a> {
    objects: HashMap<String, VTableObject<'a>>,
}

impl<'a> VTable<'a> {
    ///
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    ///
    pub fn add(&mut self, name: String, obj: VTableObject<'a>) {
        self.objects.insert(name, obj);
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(rename = "include")]
pub struct Include {
    #[serde(rename = "@href")]
    pub href: String,
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.id == other.id
            && self.fields == other.fields
            && self.groups == other.groups
            && self.semantic_type == other.semantic_type
    }
}

impl Eq for Message {}

impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // self.name.hash(state); // name might change, protocl will not break since id must remain the same
        self.id.hash(state);
        if let Some(fields) = self.fields.as_ref() {
            fields.iter().for_each(|f| f.hash(state));
        };
        if let Some(groups) = self.groups.as_ref() {
            groups.iter().for_each(|f| f.hash(state));
        };
        self.semantic_type.hash(state);
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.id == other.id
            && self.dimension_type == other.dimension_type
            && self.fields == other.fields
            && self.data == other.data
            && self.since_version == other.since_version
    }
}

impl Eq for Group {}

impl Hash for Group {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.dimension_type.hash(state);
        if let Some(fields) = self.fields.as_ref() {
            fields.iter().for_each(|f| f.hash(state));
        };
        if let Some(data) = self.data.as_ref() {
            data.iter().for_each(|f| f.hash(state));
        };
        self.since_version.hash(state);
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.id == other.id
            && self.r#type == other.r#type
            && self.since_version == other.since_version
    }
}

impl Eq for Field {}

impl Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.r#type.hash(state);
        self.since_version.hash(state);
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.id == other.id
            && self.r#type == other.r#type
            && self.since_version == other.since_version
    }
}

impl Eq for Data {}

impl Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.r#type.hash(state);
        self.since_version.hash(state);
    }
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

impl Hash for Types {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(composites) = self.composites.as_ref() {
            composites.iter().for_each(|f| f.hash(state));
        };
        if let Some(enums) = self.enums.as_ref() {
            enums.iter().for_each(|f| f.hash(state));
        };
        if let Some(sets) = self.sets.as_ref() {
            sets.iter().for_each(|f| f.hash(state));
        };
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for EnumType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.encoding_type == other.encoding_type
            && self.valid_values == other.valid_values
    }
}

impl Eq for EnumType {}

impl Hash for EnumType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        if self.encoding_type.is_some() {
            self.encoding_type.hash(state);
        }
        if let Some(valid_values) = self.valid_values.as_ref() {
            valid_values.iter().for_each(|f| f.hash(state));
        };
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ValidValue {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "$text")]
    pub value: String,
}

impl PartialEq for ValidValue {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl Eq for ValidValue {}

impl Hash for ValidValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.value.hash(state);
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for SetType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.encoding_type == other.encoding_type
            && self.choices == other.choices
    }
}

impl Eq for SetType {}

impl Hash for SetType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        if self.encoding_type.is_some() {
            self.encoding_type.hash(state);
        }
        if let Some(choices) = self.choices.as_ref() {
            choices.iter().for_each(|f| f.hash(state));
        };
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Choice {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "$text")]
    pub value: String,
}

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl Eq for Choice {}

impl Hash for Choice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.value.hash(state);
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for Composite {
    // compare is two composite are the same, i.e. contain the same list of types and refs
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.types == other.types && self.refs == other.refs
    }
}

impl Eq for Composite {}

impl Hash for Composite {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        if let Some(types) = self.types.as_ref() {
            types.iter().for_each(|f| f.hash(state));
        };
        if let Some(refs) = self.refs.as_ref() {
            refs.iter().for_each(|f| f.hash(state));
        };
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl PartialEq for Ref {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.ref_type == other.ref_type
            && self.value_ref == other.value_ref
            && self.presence == other.presence
    }
}

impl Eq for Ref {}

impl Hash for Ref {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ref_type.hash(state);
        if self.presence.is_some() {
            self.presence.hash(state);
        }
        if self.value_ref.is_some() {
            self.value_ref.hash(state);
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
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
    pub max_value: Option<String>,
    #[serde(rename = "@minValue")]
    pub min_value: Option<String>,
    /// A special value that indicates that an optional value is not set.
    /// See encodings below for default nullValue for each type.
    /// Mutually exclusive with presence=required and constant.
    #[serde(rename = "@nullValue")]
    pub null_value: Option<String>,
    #[serde(rename = "@characterEncoding")]
    pub character_encoding: Option<String>,
    #[serde(rename = "@presence")]
    pub presence: Option<Presence>,
    #[serde(rename = "@sinceVersion")]
    pub since_version: Option<u32>,
    #[serde(rename = "$text")]
    pub value: Option<String>,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.primitive_type == other.primitive_type
            && self.length == other.length
            && self.max_value == other.max_value
            && self.min_value == other.min_value
            && self.null_value == other.null_value
            && self.character_encoding == other.character_encoding
            && self.since_version == other.since_version
            && self.value == other.value
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        if self.primitive_type.is_some() {
            self.primitive_type.hash(state);
        }
        if self.length.is_some() {
            self.length.hash(state);
        }
        if self.max_value.is_some() {
            self.max_value.hash(state);
        }
        if self.min_value.is_some() {
            self.min_value.hash(state);
        }
        if self.null_value.is_some() {
            self.null_value.hash(state);
        }
        if self.character_encoding.is_some() {
            self.character_encoding.hash(state);
        }
        if self.presence.is_some() {
            self.presence.hash(state);
        }
        if self.since_version.is_some() {
            self.since_version.hash(state);
        }
        if self.value.is_some() {
            self.value.hash(state);
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Hash)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Presence {
    /// The field has a constant value that need not be transmitted on the wire.
    /// Mutually exclusive with nullValue, minValue, and maxValue attributes.
    Constant,
    /// The field must always be set. This is the default presence. Mutually exclusive with nullValue.
    Required,
    /// The field need not be populated. A special null value indicates that a field is not set.
    /// The presence attribute may be specified on either on a field or its encoding.
    Optional,
}

#[derive(PartialEq, Deserialize, Serialize, Hash)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Hash)]
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
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

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
            <field name="v1" id="1" type="uint64" desctription="asdf"/>
        </sbe:message>
    </sbe:messageSchema>
"#;

    #[test]
    fn it_works() {
        let sbe: Schema = from_str(XML).expect("Failed to parse XML");
        dbg!("{:?}", &sbe);
        let mut h = DefaultHasher::new();
        sbe.hash(&mut h);
        let hash = h.finish();
        dbg!("hash {:?}", hash);

        let expected_hash = 9964189947062957224;
        assert_eq!(hash, expected_hash);

        assert_eq!(sbe.byte_order, Some(ByteOrder::LittleEndian));
        assert_eq!(
            sbe.semantic_version,
            SematicVersion(semver::Version::from_str("5.2").unwrap())
        );

        // let xml = quick_xml::se::to_string(&sbe).expect("Failed to serialize XML");
        // dbg!("{:?}", &xml);
    }
}
