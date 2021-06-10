use super::super::super::proto::verimid::verimcosmos::verimcosmos::MsgDeleteNym as ProtoMsgDeleteNym;
use super::super::super::VerimProto;
use cosmos_sdk::tx::Msg;
use indy_api_types::errors::IndyResult;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgDeleteNym {
    pub creator: String,
    pub id: u64,
}

impl MsgDeleteNym {
    pub fn new(creator: String, id: u64) -> Self {
        MsgDeleteNym { creator, id }
    }
}

impl VerimProto for MsgDeleteNym {
    type Proto = ProtoMsgDeleteNym;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            creator: self.creator.clone(),
            id: self.id.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(proto.creator.clone(), proto.id.clone()))
    }
}
