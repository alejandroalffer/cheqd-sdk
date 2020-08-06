use v3::messages::a2a::message_family::MessageFamilies;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use error::prelude::*;
use regex::{Regex, Match};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MessageType {
    pub did: String,
    pub family: MessageFamilies,
    pub version: String,
    pub type_: String,
}

impl MessageType {
    pub fn build_with_did(family: MessageFamilies, name: &str) -> MessageType {
        MessageType {
            did: MessageFamilies::DID.to_string(),
            version: family.version().to_string(),
            family,
            type_: name.to_string(),
        }
    }

    pub fn build_with_endpoint(family: MessageFamilies, name: &str) -> MessageType {
        MessageType {
            did: MessageFamilies::ENDPOINT.to_string(),
            version: family.version().to_string(),
            family,
            type_: name.to_string(),
        }
    }
}


impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match value.as_str() {
            Some(type_) => {
                let (did, family, version, type_) = parse_message_type(type_).map_err(de::Error::custom)?;
                Ok(MessageType {
                    did,
                    family: MessageFamilies::from(family),
                    version,
                    type_,
                })
            }
            val => Err(de::Error::custom(format!("Unexpected @type field structure: {:?}", val)))
        }
    }
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = Value::String(self.to_string());
        value.serialize(serializer)
    }
}

pub fn parse_message_type(message_type: &str) -> VcxResult<(String, String, String, String)> {
    trace!("parse_message_type >>> message_type: {:?}", secret!(message_type));

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            (?P<prefix>did:\w+:\w+;spec|https://didcomm.org)/
            (?P<family>.*)/
            (?P<version>.*)/
            (?P<type>.*)").unwrap();
    }

    let message_type = RE.captures(message_type)
        .and_then(|cap| {
            let prefix = cap.name("prefix").as_ref().map(Match::as_str);
            let family = cap.name("family").as_ref().map(Match::as_str);
            let version = cap.name("version").as_ref().map(Match::as_str);
            let type_ = cap.name("type").as_ref().map(Match::as_str);

            match (prefix, family, version, type_) {
                (Some(prefix), Some(family), Some(version), Some(type_)) =>
                    Some((prefix.to_string(), family.to_string(), version.to_string(), type_.to_string())),
                _ => None
            }
        }).ok_or(VcxError::from_msg(VcxErrorKind::InvalidOption, format!("Cannot parse @type from string: {}", message_type)))?;

    trace!("parse_message_type <<< message_type: {:?}", secret!(message_type));
    Ok(message_type)
}

impl ::std::string::ToString for MessageType {
    fn to_string(&self) -> String {
        format!("{}/{}/{}/{}", self.did, self.family.to_string(), self.version, self.type_)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn _message_type_with_did() -> &'static str {
        "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/connections/1.0/invitation"
    }

    fn _message_type_with_endpoint() -> &'static str {
        "https://didcomm.org/connections/1.0/invitation"
    }

    #[test]
    fn test_build_message_type_with_did() {
        let message_type = MessageType::build_with_did(MessageFamilies::Connections, "invitation");
        assert_eq!(_message_type_with_did().to_string(), message_type.to_string());
    }

    #[test]
    fn test_build_message_type_with_endpoint() {
        let message_type = MessageType::build_with_endpoint(MessageFamilies::Connections, "invitation");
        assert_eq!(_message_type_with_endpoint().to_string(), message_type.to_string());
    }

    #[test]
    fn test_parse_message_type_with_did() {
        let message_type: MessageType = ::serde_json::from_value(serde_json::Value::String(_message_type_with_did().to_string())).unwrap();
        assert_eq!(MessageType::build_with_did(MessageFamilies::Connections, "invitation"), message_type);
    }

    #[test]
    fn test_parse_message_type_with_endpoint() {
        let message_type: MessageType = ::serde_json::from_value(serde_json::Value::String(_message_type_with_endpoint().to_string())).unwrap();
        assert_eq!(MessageType::build_with_endpoint(MessageFamilies::Connections, "invitation"), message_type);
    }
}