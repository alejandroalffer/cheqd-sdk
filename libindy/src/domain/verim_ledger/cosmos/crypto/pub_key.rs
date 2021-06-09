//! Helper class to handle private keys generic proto conversion

use super::secp256k1;
use super::super::super::prost_ext::ProstMessageExt;
use super::super::super::VerimProto;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PubKey {
    Secp256k1(secp256k1::PubKey),
}

impl VerimProto for PubKey {
    type Proto = prost_types::Any;

    fn to_proto(&self) -> Self::Proto {
        unimplemented!()
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        match &proto.type_url[..] {
            "/cosmos.crypto.secp256k1.PubKey" => {
                let val = secp256k1::PubKey::from_proto_bytes(&proto.value)?;
                Ok(PubKey::Secp256k1(val))
            }
            unknown_type => Err(IndyError::from_msg(
                IndyErrorKind::InvalidStructure,
                format!("Unknown pub_key type: {}", unknown_type),
            )),
        }
    }
}
