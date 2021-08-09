use cosmos_sdk::proto::cosmos::bank::v1beta1::MsgSend as ProtoMsgSend;

use indy_api_types::errors::IndyResult;

use super::super::crypto::PubKey;
use super::super::CheqdProto;
use super::Coin;

// MsgSend represents a message to send coins from one account to another.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MsgSend {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Coin>,
}

impl MsgSend {
    pub fn new(
        from_address: String,
        to_address: String,
        amount: Vec<Coin>,
    ) -> Self {
        MsgSend {
            from_address,
            to_address,
            amount,
        }
    }
}

impl CheqdProto for MsgSend {
    type Proto = ProtoMsgSend;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            from_address: self.from_address.clone(),
            to_address: self.to_address.clone(),
            amount: self.amount.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto.from_address.clone(),
            proto.to_address.clone(),
            proto.amount.clone(),
        ))
    }
}
