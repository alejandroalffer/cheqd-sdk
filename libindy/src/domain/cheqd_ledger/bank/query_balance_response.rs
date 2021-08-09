use cosmos_sdk::proto::cosmos::bank::v1beta1::QueryBalanceResponse as ProtoQueryBalanceRequest;
use cosmos_sdk::proto::cosmos::base::v1beta1::Coin;

use indy_api_types::errors::IndyResult;

use super::super::crypto::PubKey;
use super::super::CheqdProto;

/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryBalanceResponse {
    pub balance: Option<Coin>,
}

impl QueryBalanceResponse {
    pub fn new(
        balance: Option<Coin>,
    ) -> Self {
        QueryBalanceResponse {
            balance,
        }
    }
}

impl CheqdProto for QueryBalanceResponse {
    type Proto = ProtoMsgSend;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            balance: self.balance.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto.balance.clone(),
        ))
    }
}
