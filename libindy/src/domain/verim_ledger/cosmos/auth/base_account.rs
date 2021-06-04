use cosmos_sdk::proto::cosmos::auth::v1beta1::BaseAccount as ProtoBaseAccount;
use super::super::super::VerimProto;
use super::super::crypto::secp256k1::PubKey;

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
    pub fn new(address: String, pub_key: Option<PubKey>, account_number: u64, sequence: u64) -> Self {
        BaseAccount { address, pub_key, account_number, sequence }
    }
}


impl VerimProto for BaseAccount {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_account_request() {
        let msg = QueryAccountRequest::new(
            "cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd".to_string()
        );

        let proto = msg.to_proto();
        let decoded = QueryAccountRequest::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}


