use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgUpdateNym as ProtoMsgUpdateNym;
use crate::domain::verim_ledger::VerimMessage;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgUpdateNym {
    pub creator: String,
    pub id: u64,
    pub alias: String,
    pub verkey: String,
    pub did: String,
    pub role: String,
}

impl MsgUpdateNym {
    pub fn new(
        creator: String,
        id: u64,
        alias: String,
        verkey: String,
        did: String,
        role: String,
    ) -> Self {
        MsgUpdateNym {
            creator,
            id,
            alias,
            verkey,
            did,
            role,
        }
    }
}

impl VerimMessage for MsgUpdateNym {
    type Proto = ProtoMsgUpdateNym;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            creator: self.creator.clone(),
            id: self.id.clone(),
            alias: self.alias.clone(),
            verkey: self.verkey.clone(),
            did: self.did.clone(),
            role: self.role.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self {
            creator: proto.creator.clone(),
            id: proto.id.clone(),
            alias: proto.alias.clone(),
            verkey: proto.verkey.clone(),
            did: proto.did.clone(),
            role: proto.role.clone(),
        }
    }
}
