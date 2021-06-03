use cosmos_sdk::proto::cosmos::auth::v1beta1::QueryAccountRequest as ProtoQueryAccountRequest;
// use crate::domain::verim_ledger::VerimProto;
use super::super::super::VerimProto;

/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAccountRequest {
    /// address defines the address to query for.
    pub address: String,
}

impl QueryAccountRequest {
    pub fn new(
        address: String,
    ) -> Self {
        QueryAccountRequest {
            address,
        }
    }
}

impl VerimProto for QueryAccountRequest {
    type Proto = ProtoQueryAccountRequest;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            address: self.address.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self::new(
            proto.address.clone(),
        )
    }
}