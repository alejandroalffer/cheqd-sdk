use super::super::super::proto::verimid::verimcosmos::verimcosmos::QueryAllNymRequest as ProtoQueryAllNymRequest;
use super::super::super::VerimMessage;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAllNymRequest {
}

impl QueryAllNymRequest {
    pub fn new() -> Self {
        QueryAllNymRequest { }
    }
}

impl VerimMessage for QueryAllNymRequest {
    type Proto = ProtoQueryAllNymRequest;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            // pagination: self.pagination.clone(),
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        Self {

        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_get_all_nyms_request() {
        let msg = QueryAllNymRequest::new();

        let proto = msg.to_proto();
        let decoded = QueryAllNymRequest::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
