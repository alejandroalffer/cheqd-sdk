use cosmos_sdk::proto::cosmos::auth::v1beta1::QueryAccountRequest as ProtoQueryAccountResponse;
use super::super::super::VerimProto;

/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    pub account: String,
}

impl QueryAccountResponse {
    pub fn new(
        account: String,
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
            address: self.account.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self::new(
            proto.account.clone(),
        )
    }
}