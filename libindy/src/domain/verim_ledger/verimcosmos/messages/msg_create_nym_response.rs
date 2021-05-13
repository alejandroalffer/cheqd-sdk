use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgCreateNymResponse as ProtoMsgCreateNymResponse;
use crate::domain::verim_ledger::VerimMessage;

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
    use crate::domain::verim_ledger::verimcosmos::messages::msg_create_nym::MsgCreateNym;
    use crate::domain::verim_ledger::verimcosmos::messages::msg_create_nym_response::MsgCreateNymResponse;
    use crate::domain::verim_ledger::VerimMessage;

    #[test]
    fn test_msg_create_nym_response() {
        let msg = MsgCreateNymResponse::new(456);

        let proto = msg.to_proto();
        let decoded = MsgCreateNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
