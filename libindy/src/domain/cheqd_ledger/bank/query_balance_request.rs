use cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest as ProtoQueryBalanceRequest;

use indy_api_types::errors::IndyResult;

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
    type Proto = ProtoQueryBalanceRequest;

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


#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::environment;

    #[test]
    fn test_query_balance() {
        let msg = QueryBalanceRequest::new(
            "cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd".to_string(),
            environment::cheqd_denom(),
        );

        let proto = msg.to_proto();
        let decoded = QueryBalanceRequest::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}