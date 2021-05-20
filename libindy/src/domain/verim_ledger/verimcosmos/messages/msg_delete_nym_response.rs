use super::super::super::proto::verimid::verimcosmos::verimcosmos::MsgDeleteNymResponse as ProtoMsgDeleteNymResponse;
use super::super::super::VerimMessage;

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
    use super::super::super::super::VerimMessage;
    use super::super::MsgDeleteNym;
    use super::MsgDeleteNymResponse;

    #[test]
    fn test_msg_delete_nym_response() {
        let msg = MsgDeleteNymResponse::new();

        let proto = msg.to_proto();
        let decoded = MsgDeleteNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
