use cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceResponse as ProtoQueryBalanceResponse;

use indy_api_types::errors::IndyResult;

use super::super::CheqdProto;
use super::super::bank::Coin;

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
    type Proto = ProtoQueryBalanceResponse;

    fn to_proto(&self) -> Self::Proto {
        let balance = match &self.balance {
            Some(p) => Some(p.to_proto()),
            None => None,
        };
        Self::Proto { balance }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        let balance = proto
            .balance
            .as_ref()
            .map(|p| Coin::from_proto(p))
            .transpose()?;

        Ok(Self::new(balance))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_balance_response() {
        let msg = QueryBalanceResponse::new(None);

        let proto = msg.to_proto();
        let decoded = QueryBalanceResponse::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}