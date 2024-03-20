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
use semver::Version;
use serde_semver::SemverReq;

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename = "sbe:messageSchema")]
struct Schema {
    #[serde(rename = "@package")]
    package: String,
    #[serde(rename = "@id")]
    id: i32,
    #[serde(rename = "@version")]
    version: u32,  
    #[serde(rename = "@semanticVersion")]
    semantic_version: SematicVersion,
    #[serde(rename = "@description")]
    description: String,
    #[serde(rename = "@byteOrder")]
    byte_order: ByteOrder,
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

impl Default for ByteOrder {
    fn default() -> Self {
        ByteOrder::LittleEndian
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    const XML: &str = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <sbe:messageSchema xmlns:sbe="http://fixprotocol.io/2016/sbe"
                       xmlns:xi="http://www.w3.org/2001/XInclude"
                       package="sbe.messages"
                       id="1"
                       version="1"
                       semanticVersion="5.2.0"
                       description="sbe messages"
                       byteOrder="littleEndian"> 
        <sbe:message name="HelloMessage" id="1" description="hello message">
            <field name="id" id="1" type="uint8"/>
        </sbe:message>
    </sbe:messageSchema>
"#;

    #[test]
    fn it_works() {
        let sbe: Schema = from_str(XML).expect("Failed to parse XML");
        dbg!("{:?}", &sbe);
    }
}
