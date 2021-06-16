use super::super::verim_ledger::prost_ext::ProstMessageExt;
use cosmos_sdk::proto::cosmos::tx::v1beta1::{SignDoc as ProtoSignDoc, TxRaw};
use cosmos_sdk::tx::{Msg, Raw, SignDoc};
use indy_api_types::errors::IndyResult;
use prost::Message;
use prost_types::Any;

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

pub trait CosmosSignDocExt {
    fn to_bytes(&self) -> IndyResult<Vec<u8>>;
    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized;
}

impl CosmosSignDocExt for SignDoc {
    fn to_bytes(&self) -> IndyResult<Vec<u8>> {
        let proto: ProtoSignDoc = self.clone().into();
        Ok(proto.to_bytes()?)
    }

    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized,
    {
        let proto = ProtoSignDoc::from_bytes(bytes)?;
        Ok(proto.into())
    }
}

pub trait CosmosRawExt {
    fn to_bytes(&self) -> IndyResult<Vec<u8>>;
    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized;
}

impl CosmosRawExt for Raw {
    fn to_bytes(&self) -> IndyResult<Vec<u8>> {
        let proto: TxRaw = self.clone().into();
        Ok(proto.to_bytes()?)
    }

    fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    where
        Self: Sized,
    {
        let proto = TxRaw::from_bytes(bytes)?;
        Ok(proto.into())
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
    fn test_cosmos_msg_ext() {
        let message = MsgCreateNym::new(
            "creator".to_string(),
            "alias".to_string(),
            "verkey".to_string(),
            "did".to_string(),
            "role".to_string(),
        );

        let msg = message.to_proto().to_msg().unwrap();

        let bytes: Vec<u8> = msg.to_bytes().unwrap();
        let decoded = Msg::from_bytes(bytes.as_slice()).unwrap();

        assert_eq!(msg, decoded);
    }
}
