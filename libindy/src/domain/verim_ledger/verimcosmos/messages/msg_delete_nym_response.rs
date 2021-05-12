use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgDeleteNymResponse as ProtoMsgDeleteNymResponse;
use crate::domain::verim_ledger::VerimMessage;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgDeleteNymResponse {
    pub id: u64,
}

impl MsgDeleteNymResponse {
    pub fn new(id: u64) -> Self {
        MsgDeleteNymResponse { id }
    }
}

impl VerimMessage for MsgDeleteNymResponse {
    type Proto = ProtoMsgDeleteNymResponse;

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
    use crate::domain::verim_ledger::verimcosmos::messages::msg_delete_nym::MsgDeleteNym;
    use crate::domain::verim_ledger::verimcosmos::messages::msg_delete_nym_response::MsgDeleteNymResponse;
    use crate::domain::verim_ledger::VerimMessage;

    #[test]
    fn test_msg_delete_nym_response() {
        let msg = MsgDeleteNymResponse::new(456);

        let proto = msg.to_proto();
        let decoded = MsgDeleteNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
