use super::super::super::proto::verimid::verimcosmos::verimcosmos::QueryGetNymResponse as ProtoQueryGetNymResponse;
use super::super::super::VerimMessage;
use super::super::models::nym::Nym;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryGetNymResponse {
    pub nym: Option<Nym>,
}

impl QueryGetNymResponse {
    pub fn new(nym: Option<Nym>) -> Self {
        QueryGetNymResponse { nym }
    }
}

impl VerimMessage for QueryGetNymResponse {
    type Proto = ProtoQueryGetNymResponse;

    fn to_proto(&self) -> Self::Proto {
        let nym = match &self.nym {
            Some(n) => Some(n.to_proto()),
            None => None,
        };
        Self::Proto { nym }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        let nym = match &proto.nym {
            Some(n) => Some(Nym::from_proto(n)),
            None => None,
        };
        Self { nym }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_get_nym_response() {
        let nym = Nym::new(
            "creator".to_string(),
            123,
            "alias".to_string(),
            "verkey".to_string(),
            "did".to_string(),
            "role".to_string(),
        );
        let msg = QueryGetNymResponse::new(Some(nym));

        let proto = msg.to_proto();
        let decoded = QueryGetNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
