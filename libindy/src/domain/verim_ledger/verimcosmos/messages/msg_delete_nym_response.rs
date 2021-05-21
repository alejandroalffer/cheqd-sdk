use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgDeleteNymResponse as ProtoMsgDeleteNymResponse;
use crate::domain::verim_ledger::VerimMessage;

#[derive(Eq, PartialEq, Debug)]
pub struct MsgDeleteNymResponse {

}

impl MsgDeleteNymResponse {
    pub fn new() -> Self {
        MsgDeleteNymResponse {  }
    }
}

impl VerimMessage for MsgDeleteNymResponse {
    type Proto = ProtoMsgDeleteNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto { }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self { }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::verim_ledger::verimcosmos::messages::msg_delete_nym::MsgDeleteNym;
    use crate::domain::verim_ledger::verimcosmos::messages::msg_delete_nym_response::MsgDeleteNymResponse;
    use crate::domain::verim_ledger::VerimMessage;

    #[test]
    fn test_msg_delete_nym_response() {
        let msg = MsgDeleteNymResponse::new();

        let proto = msg.to_proto();
        let decoded = MsgDeleteNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
