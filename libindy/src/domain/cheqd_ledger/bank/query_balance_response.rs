use cosmos_sdk::proto::cosmos::bank::v1beta1::QueryBalanceResponse as ProtoQueryBalanceRequest;

use indy_api_types::errors::IndyResult;

use super::super::crypto::PubKey;
use super::super::CheqdProto;
use super::Coin;

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
        let balance = match &self.balance {
            Some(p) => Some(p.to_proto()),
            None => None,
        };
        Self::Proto { balance }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        let balance = proto
            .pagination
            .as_ref()
            .map(|p| Coin::from_proto(p))
            .transpose()?;

        Ok(Self::new(balance))
    }
}
