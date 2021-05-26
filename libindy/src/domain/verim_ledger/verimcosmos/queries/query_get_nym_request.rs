use super::super::super::proto::verimid::verimcosmos::verimcosmos::QueryGetNymRequest as ProtoQueryGetNymRequest;
use super::super::super::VerimMessage;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryGetNymRequest {
    pub id: u64,
}

impl QueryGetNymRequest {
    pub fn new(id: u64) -> Self {
        QueryGetNymRequest { id }
    }
}

impl VerimMessage for QueryGetNymRequest {
    type Proto = ProtoQueryGetNymRequest;

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
    use super::*;

    #[test]
    fn test_query_get_nym_request() {
        let msg = QueryGetNymRequest::new(456);

        let proto = msg.to_proto();
        let decoded = QueryGetNymRequest::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
