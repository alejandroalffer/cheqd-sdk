use cosmos_sdk::proto::cosmos::bank::v1beta1::QueryBalanceRequest as ProtoQueryBalanceRequest;

use indy_api_types::errors::IndyResult;

use super::super::crypto::PubKey;
use super::super::CheqdProto;

/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryBalanceRequest {
    pub address: String,
    pub denom: String,
}

impl QueryBalanceRequest {
    pub fn new(
        address: String,
        denom: String,
    ) -> Self {
        QueryBalanceRequest {
            address,
            denom,
        }
    }
}

impl CheqdProto for QueryBalanceRequest {
    type Proto = ProtoMsgSend;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            address: self.address.clone(),
            denom: self.denom.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto.address.clone(),
            proto.denom.clone(),
        ))
    }
}
