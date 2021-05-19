use super::super::super::super::verim_ledger::VerimMessage;
use super::super::super::proto::verimid::verimcosmos::verimcosmos::MsgCreateNymResponse as ProtoMsgCreateNymResponse;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgCreateNymResponse {
    pub id: u64,
}

impl MsgCreateNymResponse {
    pub fn new(id: u64) -> Self {
        MsgCreateNymResponse { id }
    }
}

impl VerimMessage for MsgCreateNymResponse {
    type Proto = ProtoMsgCreateNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            id: self.id.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self {
            id: proto.id.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::super::super::VerimMessage;
    use super::super::MsgCreateNym;
    use super::MsgCreateNymResponse;

    #[test]
    fn test_msg_create_nym_response() {
        let msg = MsgCreateNymResponse::new(456);

        let proto = msg.to_proto();
        let decoded = MsgCreateNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
