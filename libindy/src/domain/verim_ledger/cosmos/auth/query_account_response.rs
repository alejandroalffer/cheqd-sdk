use cosmos_sdk::proto::cosmos::auth::v1beta1::QueryAccountResponse as ProtoQueryAccountResponse;
use super::super::super::VerimProto;
use failure::_core::any::Any;

/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    pub account: Option<Any>,
}

impl QueryAccountResponse {
    pub fn new(
        account: Option<Any>,
    ) -> Self {
        QueryAccountResponse {
            account,
        }
    }
}

impl VerimProto for QueryAccountResponse {
    type Proto = ProtoQueryAccountResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            account: self.account.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self::new(
            proto.account.clone(),
        )
    }
}