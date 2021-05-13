use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::Nym as ProtoNym;
use crate::domain::verim_ledger::VerimMessage;

#[derive(Eq, PartialEq, Debug)]
pub struct Nym {
    pub creator: String,
    pub id: u64,
    pub alias: String,
    pub verkey: String,
    pub did: String,
    pub role: String,
}

impl Nym {
    pub fn new(creator: String, id: u64, alias: String, verkey: String, did: String, role: String) -> Self {
        Nym {
            creator,
            id,
            alias,
            verkey,
            did,
            role,
        }
    }
}

impl VerimMessage for Nym {
    type Proto = ProtoNym;

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

#[cfg(test)]
mod test {
    use crate::domain::verim_ledger::VerimMessage;
    use crate::domain::verim_ledger::verimcosmos::models::nym::Nym;

    #[test]
    fn test_msg_create_nym_response() {
        let msg = Nym::new("creator".to_string(),
                           456,
                           "alias".to_string(),
                           "verkey".to_string(),
                           "did".to_string(),
                           "role".to_string(),
        );

        let proto = msg.to_proto();
        let decoded = Nym::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
