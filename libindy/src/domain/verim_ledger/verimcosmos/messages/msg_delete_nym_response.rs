use super::super::super::proto::verimid::verimcosmos::verimcosmos::MsgDeleteNymResponse as ProtoMsgDeleteNymResponse;
use super::super::super::VerimProto;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MsgDeleteNymResponse {}

impl MsgDeleteNymResponse {
    pub fn new() -> Self {
        MsgDeleteNymResponse {}
    }
}

impl VerimProto for MsgDeleteNymResponse {
    type Proto = ProtoMsgDeleteNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {}
    }

    fn from_proto(_proto: &Self::Proto) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_msg_delete_nym_response() {
        let msg = MsgDeleteNymResponse::new();

        let proto = msg.to_proto();
        let decoded = MsgDeleteNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
