use cosmrs::proto::cosmos::bank::v1beta1::MsgSend as ProtoMsgSend;

use indy_api_types::errors::IndyResult;

use super::super::CheqdProto;
use super::super::bank::Coin;

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
            amount: self.amount.clone().into_iter().map(|coin| coin.to_proto()).collect(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        let mut amount = Vec::new();
        for coin in proto.amount.clone().into_iter(){
            amount.push(Coin::from_proto(&coin)?);
        };
        Ok(Self::new(
            proto.from_address.clone(),
            proto.to_address.clone(),
            amount,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::environment;

    #[test]
    fn test_msg_send() {
        let coins = Coin::new(environment::cheqd_denom(), "100".to_string());
        let mut amount: Vec<Coin> = Vec::new();
        amount.push(coins);

        let msg = MsgSend::new(
            "cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd".to_string(),
            "cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd".to_string(),
            amount
        );

        let proto = msg.to_proto();
        let decoded = MsgSend::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}