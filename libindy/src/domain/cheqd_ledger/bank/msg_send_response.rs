use cosmos_sdk::proto::cosmos::bank::v1beta1::MsgSendResponse as ProtoMsgSendResponse;

use indy_api_types::errors::IndyResult;

use super::super::crypto::PubKey;
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
    type Proto = ProtoBaseAccount;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {}
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new())
    }
}
