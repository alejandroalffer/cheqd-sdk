use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgUpdateNymResponse as ProtoMsgUpdateNymResponse;
use crate::domain::verim_ledger::VerimMessage;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgUpdateNymResponse { }

impl MsgUpdateNymResponse {
    pub fn new() -> Self {
        MsgUpdateNymResponse { }
    }
}

impl VerimMessage for MsgUpdateNymResponse {
    type Proto = ProtoMsgUpdateNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto { }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self { }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::verim_ledger::verimcosmos::messages::msg_update_nym::MsgUpdateNym;
    use crate::domain::verim_ledger::verimcosmos::messages::msg_update_nym_response::MsgUpdateNymResponse;
    use crate::domain::verim_ledger::VerimMessage;

    #[test]
    fn test_msg_update_nym_response() {
        let msg = MsgUpdateNymResponse::new();

        let proto = msg.to_proto();
        let decoded = MsgUpdateNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
