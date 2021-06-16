//! Helper class to handle accounts generic proto conversion

use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;

use crate::domain::verim_ledger::VerimProto;

use super::*;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
            "/cosmos.auth.v1beta1.BaseAccount" => {
                let val = BaseAccount::from_proto_bytes(&proto.value)?;
                Ok(Account::BaseAccount(val))
            }
            unknown_type => Err(IndyError::from_msg(
                IndyErrorKind::InvalidStructure,
                format!("Unknown account type: {}", unknown_type),
            )),
        }
    }
}
