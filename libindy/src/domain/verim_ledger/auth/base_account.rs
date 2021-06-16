use cosmos_sdk::proto::cosmos::auth::v1beta1::BaseAccount as ProtoBaseAccount;
use indy_api_types::errors::IndyResult;

use crate::domain::verim_ledger::crypto::PubKey;
use crate::domain::verim_ledger::VerimProto;

/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct BaseAccount {
    pub address: String,
    pub pub_key: Option<PubKey>,
    pub account_number: u64,
    pub sequence: u64,
}

impl BaseAccount {
    pub fn new(
        address: String,
        pub_key: Option<PubKey>,
        account_number: u64,
        sequence: u64,
    ) -> Self {
        BaseAccount {
            address,
            pub_key,
            account_number,
            sequence,
        }
    }
}

impl VerimProto for BaseAccount {
    type Proto = ProtoBaseAccount;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            address: self.address.clone(),
            pub_key: self.pub_key.as_ref().map(|k| k.to_proto()),
            account_number: self.account_number,
            sequence: self.sequence,
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto.address.clone(),
            proto
                .pub_key
                .as_ref()
                .map(|pk| PubKey::from_proto(pk))
                .transpose()?,
            proto.account_number,
            proto.sequence,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::domain::verim_ledger::auth::QueryAccountRequest;

    use super::*;

    #[test]
    fn test_query_account_request() {
        let msg =
            QueryAccountRequest::new("cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd".to_string());

        let proto = msg.to_proto();
        let decoded = QueryAccountRequest::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
