use crate::domain::verim_ledger::ProtoStruct;
use cosmos_sdk::proto::cosmos::tx::v1beta1::SignDoc as SignDocProto;
use prost::Message;

/// SignDoc is the type used for generating sign bytes for SIGN_MODE_DIRECT.
#[derive(Debug, Serialize)]
pub struct SignDoc {
    /// body_bytes is protobuf serialization of a TxBody that matches the
    /// representation in TxRaw.
    pub body_bytes: Vec<u8>,
    /// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
    /// representation in TxRaw.
    pub auth_info_bytes: Vec<u8>,
    /// chain_id is the unique identifier of the chain this transaction targets.
    /// It prevents signed transactions from being used on another chain by an
    /// attacker
    pub chain_id: String,
    /// account_number is the account number of the account in state
    pub account_number: u64,
}

impl ProtoStruct for SignDoc {
    type Proto = SignDocProto;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            body_bytes: self.body_bytes.clone(),
            auth_info_bytes: self.auth_info_bytes.clone(),
            chain_id: self.chain_id.clone(),
            account_number: self.account_number.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self {
            body_bytes: proto.body_bytes.clone(),
            auth_info_bytes: proto.auth_info_bytes.clone(),
            chain_id: proto.chain_id.clone(),
            account_number: proto.account_number.clone(),
        }
    }
}
