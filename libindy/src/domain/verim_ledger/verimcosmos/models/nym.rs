use super::super::super::proto::verimid::verimnode::verim::Nym as ProtoNym;
use super::super::super::VerimProto;
use indy_api_types::errors::IndyResult;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Nym {
    pub creator: String,
    pub id: u64,
    pub alias: String,
    pub verkey: String,
    pub did: String,
    pub role: String,
}

impl Nym {
    pub fn new(
        creator: String,
        id: u64,
        alias: String,
        verkey: String,
        did: String,
        role: String,
    ) -> Self {
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

impl VerimProto for Nym {
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

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto.creator.clone(),
            proto.id.clone(),
            proto.alias.clone(),
            proto.verkey.clone(),
            proto.did.clone(),
            proto.role.clone(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_msg_create_nym_response() {
        let msg = Nym::new(
            "creator".to_string(),
            456,
            "alias".to_string(),
            "verkey".to_string(),
            "did".to_string(),
            "role".to_string(),
        );

        let proto = msg.to_proto();
        let decoded = Nym::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
