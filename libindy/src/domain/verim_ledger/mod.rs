use cosmos_sdk::tx::{Msg, MsgProto, MsgType};
use indy_api_types::errors::IndyResult;
use prost::Message;
use prost_types::Any;
use std::fmt::Debug;

pub mod proto;
pub mod verimcosmos;

pub trait VerimMessage: Eq + Debug {
    type Proto: MsgType;

    fn to_proto(&self) -> Self::Proto;
    fn from_proto(proto: &Self::Proto) -> Self;

    fn to_msg(&self) -> IndyResult<Msg> {
        Ok(self.to_proto().to_msg()?)
    }

    fn from_msg(msg: &Msg) -> IndyResult<Self>
    where
        Self: Sized,
    {
        let proto = Self::Proto::from_msg(msg)?;
        Ok(Self::from_proto(&proto))
    }
}

pub trait ProstMessageExt {
    fn to_bytes(&self) -> IndyResult<Vec<u8>>;
    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized;
}

impl<T> ProstMessageExt for T
where
    T: Message + Default,
{
    fn to_bytes(&self) -> IndyResult<Vec<u8>> {
        let mut bytes = Vec::new();
        Message::encode(self, &mut bytes)?;
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized,
    {
        Ok(Self::decode(bytes)?)
    }
}

pub trait CosmosMsgExt {
    fn to_bytes(&self) -> IndyResult<Vec<u8>>;
    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized;
}

impl CosmosMsgExt for Msg {
    fn to_bytes(&self) -> IndyResult<Vec<u8>> {
        let proto: Any = self.clone().into();
        Ok(proto.to_bytes()?)
    }

    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized,
    {
        let res = Any::from_bytes(bytes)?;
        Ok(res.into())
    }
}

#[cfg(test)]
mod test {
    use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgCreateNym as ProtoMsgCreateNym;
    use crate::domain::verim_ledger::verimcosmos::messages::MsgCreateNym;
    use crate::domain::verim_ledger::CosmosMsgExt;
    use crate::domain::verim_ledger::ProstMessageExt;
    use crate::domain::verim_ledger::VerimMessage;
    use cosmos_sdk::tx::Msg;

    #[test]
    fn test_prost_message_ext() {
        let message = MsgCreateNym::new(
            "creator".to_string(),
            "alias".to_string(),
            "verkey".to_string(),
            "did".to_string(),
            "role".to_string(),
        );

        let proto: ProtoMsgCreateNym = message.to_proto();

        let bytes: Vec<u8> = proto.to_bytes().unwrap();
        let decoded = ProtoMsgCreateNym::from_bytes(bytes.as_slice()).unwrap();

        assert_eq!(proto, decoded);
    }

    #[test]
    fn test_cosmos_msg_ext() {
        let message = MsgCreateNym::new(
            "creator".to_string(),
            "alias".to_string(),
            "verkey".to_string(),
            "did".to_string(),
            "role".to_string(),
        );

        let msg = message.to_msg().unwrap();

        let bytes: Vec<u8> = msg.to_bytes().unwrap();
        let decoded = Msg::from_bytes(bytes.as_slice()).unwrap();

        assert_eq!(msg, decoded);
    }
}
