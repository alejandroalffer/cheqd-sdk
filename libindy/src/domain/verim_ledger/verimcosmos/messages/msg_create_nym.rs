use super::super::super::proto::verimid::verimnode::verim::MsgCreateNym as ProtoMsgCreateNym;
use super::super::super::VerimProto;
use cosmos_sdk::tx::Msg;
use indy_api_types::errors::IndyResult;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgCreateNym {
    pub creator: String,
    pub alias: String,
    pub verkey: String,
    pub did: String,
    pub role: String,
}

impl MsgCreateNym {
    pub fn new(creator: String, alias: String, verkey: String, did: String, role: String) -> Self {
        MsgCreateNym {
            creator,
            alias,
            verkey,
            did,
            role,
        }
    }
}

impl VerimProto for MsgCreateNym {
    type Proto = ProtoMsgCreateNym;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            creator: self.creator.clone(),
            alias: self.alias.clone(),
            verkey: self.verkey.clone(),
            did: self.did.clone(),
            role: self.role.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self {
            creator: proto.creator.clone(),
            alias: proto.alias.clone(),
            verkey: proto.verkey.clone(),
            did: proto.did.clone(),
            role: proto.role.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::super::super::VerimProto;
    use super::super::super::messages::msg_create_nym::MsgCreateNym;

    #[test]
    fn test_msg_create_nym() {
        let msg = MsgCreateNym::new(
            "creator".to_string(),
            "alias".to_string(),
            "verkey".to_string(),
            "did".to_string(),
            "role".to_string(),
        );

        let proto = msg.to_proto();
        let decoded = MsgCreateNym::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
