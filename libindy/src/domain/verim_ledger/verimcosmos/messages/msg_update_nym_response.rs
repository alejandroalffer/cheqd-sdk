use super::super::super::proto::verimid::verimcosmos::verimcosmos::MsgUpdateNymResponse as ProtoMsgUpdateNymResponse;
use super::super::super::VerimProto;
use indy_api_types::errors::IndyResult;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MsgUpdateNymResponse {}

impl MsgUpdateNymResponse {
    pub fn new() -> Self {
        MsgUpdateNymResponse {}
    }
}

impl VerimProto for MsgUpdateNymResponse {
    type Proto = ProtoMsgUpdateNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {}
    }

    fn from_proto(_proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_msg_update_nym_response() {
        let msg = MsgUpdateNymResponse::new();

        let proto = msg.to_proto();
        let decoded = MsgUpdateNymResponse::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
