use indy_api_types::errors::IndyResult;

use super::super::super::proto::cheqdid::cheqdnode::cheqd::MsgCreateNymResponse as ProtoMsgCreateNymResponse;
use super::super::super::super::cheqd_ledger::CheqdProto;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MsgCreateNymResponse {
    pub id: u64,
}

impl MsgCreateNymResponse {
    pub fn new(id: u64) -> Self {
        MsgCreateNymResponse { id }
    }
}

impl CheqdProto for MsgCreateNymResponse {
    type Proto = ProtoMsgCreateNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            id: self.id.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(proto.id.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::MsgCreateNymResponse;
    use super::super::super::super::CheqdProto;

    #[test]
    fn test_msg_create_nym_response() {
        let msg = MsgCreateNymResponse::new(456);

        let proto = msg.to_proto();
        let decoded = MsgCreateNymResponse::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
