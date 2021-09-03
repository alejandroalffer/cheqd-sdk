use cosmrs::proto::cosmos::bank::v1beta1::MsgSendResponse as ProtoMsgSendResponse;

use indy_api_types::errors::IndyResult;

use super::super::CheqdProto;

/// MsgSendResponse defines the Msg/Send response type.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MsgSendResponse {
}

impl MsgSendResponse {
    pub fn new(
    ) -> Self {
        MsgSendResponse {}
    }
}

impl CheqdProto for MsgSendResponse {
    type Proto = ProtoMsgSendResponse;

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
    fn test_msg_send_response() {
        let msg = MsgSendResponse::new();

        let proto = msg.to_proto();
        let decoded = MsgSendResponse::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}