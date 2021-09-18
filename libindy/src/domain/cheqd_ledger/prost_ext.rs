use indy_api_types::errors::{IndyResult, IndyResultExt, IndyErrorKind};
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
        Message::encode(self, &mut bytes).to_indy(
            IndyErrorKind::InvalidStructure,
            "Protobuf Message object cannot be encoded into the bytes vector"
        )?;
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
        where
            Self: Sized,
    {
        let decoded = Self::decode(bytes).to_indy(
            IndyErrorKind::InvalidStructure,
            "Protobuf Bytes cannot be decoded into the Message object"
        )?;
        Ok(decoded)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::cheqd_ledger::prost_ext::ProstMessageExt;
    use super::super::super::cheqd_ledger::proto::cheqdid::cheqdnode::cheqd::MsgCreateNym as ProtoMsgCreateNym;
    use super::super::super::cheqd_ledger::cheqd::messages::MsgCreateNym;
    use super::super::super::cheqd_ledger::CheqdProto;

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
