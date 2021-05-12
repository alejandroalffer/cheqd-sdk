use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgUpdateNymResponse as ProtoMsgUpdateNymResponse;
use crate::domain::verim_ledger::VerimMessage;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgUpdateNymResponse {
    pub id: u64,
}

impl MsgUpdateNymResponse {
    pub fn new(id: u64) -> Self {
        MsgUpdateNymResponse { id }
    }
}

impl VerimMessage for MsgUpdateNymResponse {
    type Proto = ProtoMsgUpdateNymResponse;

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
    use crate::domain::verim_ledger::verimcosmos::messages::msg_update_nym::MsgUpdateNym;
    use crate::domain::verim_ledger::verimcosmos::messages::msg_update_nym_response::MsgUpdateNymResponse;
    use crate::domain::verim_ledger::VerimMessage;

    #[test]
    fn test_msg_update_nym_response() {
        let msg = MsgUpdateNymResponse::new(456);

        let proto = msg.to_proto();
        let decoded = MsgUpdateNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
