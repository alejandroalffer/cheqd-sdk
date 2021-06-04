//! Helper class to handle accounts generic proto conversion

use super::*;
use crate::domain::verim_ledger::cosmos::crypto::secp256k1;
use crate::domain::verim_ledger::prost_ext::ProstMessageExt;
use crate::domain::verim_ledger::VerimProto;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Account {
    BaseAccount(BaseAccount),
}

impl VerimProto for Account {
    type Proto = prost_types::Any;

    fn to_proto(&self) -> Self::Proto {
        unimplemented!()
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        match &proto.type_url[..] {
            "secp256k" => {
                let val = BaseAccount::from_proto_bytes(&proto.value)?;
                Ok(Account::BaseAccount(val))
            }
            _ => Err(IndyError::from_msg(
                IndyErrorKind::InvalidStructure,
                "Unknown account type",
            )),
        }
    }
}
