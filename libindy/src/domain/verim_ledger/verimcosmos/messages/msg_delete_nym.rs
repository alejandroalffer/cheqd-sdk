use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgDeleteNym as ProtoMsgDeleteNym;
use crate::domain::verim_ledger::VerimMessage;

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

impl VerimMessage for MsgDeleteNym {
    type Proto = ProtoMsgDeleteNym;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            creator: self.creator.clone(),
            id: self.id.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self {
            creator: proto.creator.clone(),
            id: proto.id.clone(),
        }
    }
}
