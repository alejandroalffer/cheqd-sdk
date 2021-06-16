use indy_api_types::errors::IndyResult;
use prost::Message;

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

#[cfg(test)]
mod test {
    use cosmos_sdk::tx::{Msg, MsgType};

    use super::super::super::verim_ledger::prost_ext::ProstMessageExt;

    use super::super::super::verim_ledger::proto::verimid::verimnode::verim::MsgCreateNym as ProtoMsgCreateNym;
    use super::super::super::verim_ledger::verim::messages::MsgCreateNym;
    use super::super::super::verim_ledger::VerimProto;
    use super::super::cosmos_ext::CosmosMsgExt;

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
}
