use super::super::super::super::verim_ledger::VerimProto;
use super::super::super::proto::verimid::verimnode::verim::MsgCreateNymResponse as ProtoMsgCreateNymResponse;
use indy_api_types::errors::IndyResult;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MsgCreateNymResponse {
    pub id: u64,
}

impl MsgCreateNymResponse {
    pub fn new(id: u64) -> Self {
        MsgCreateNymResponse { id }
    }
}

impl VerimProto for MsgCreateNymResponse {
    type Proto = ProtoMsgCreateNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            id: self.id.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(proto.id.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::super::super::super::VerimProto;
    use super::super::MsgCreateNym;
    use super::MsgCreateNymResponse;

    #[test]
    fn test_msg_create_nym_response() {
        let msg = MsgCreateNymResponse::new(456);

        let proto = msg.to_proto();
        let decoded = MsgCreateNymResponse::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
