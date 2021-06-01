use super::super::super::proto::verimid::verimcosmos::verimcosmos::QueryAllNymResponse as ProtoQueryAllNymResponse;
use super::super::super::VerimMessage;
use super::super::models::Nym;
use cosmos_sdk::proto::cosmos::base::query::v1beta1::PageResponse;
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAllNymResponse {
    pub nym: Vec<Nym>,
    pub pagination: Option<PageResponse>
}

impl QueryAllNymResponse {
    pub fn new(nym: Vec<Nym>, pagination: Option<PageResponse>) -> Self {
        QueryAllNymResponse { nym, pagination }
    }
}

impl VerimMessage for QueryAllNymResponse {
    type Proto = ProtoQueryAllNymResponse;

    fn to_proto(&self) -> Self::Proto {
        let nym = &self.nym;
        let pagination = match &self.pagination {
            Some(p) => p,
            None => None,
        };
        Self::Proto { nym, pagination }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        let nym: Vec<Nym> = proto.nym;
        let pagination = match proto.pagination {
            Some(p) => p,
            None => None,
        };
        Self { nym, pagination }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_all_nym_response() {
        let nym: Vec<Nym> = vec![
            Nym::new(
                "creator0".to_string(),
                0,
                "alias0".to_string(),
                "verkey0".to_string(),
                "did0".to_string(),
                "role0".to_string(),
            ),
            Nym::new(
                "creator1".to_string(),
                1,
                "alias1".to_string(),
                "verkey1".to_string(),
                "did1".to_string(),
                "role1".to_string(),
            ),
            Nym::new(
                "creator2".to_string(),
                2,
                "alias2".to_string(),
                "verkey2".to_string(),
                "did2".to_string(),
                "role2".to_string(),
            ),
        ];

        let pagination = PageResponse{ next_key: vec![0], total: 3 };
        let msg = QueryAllNymResponse::new(nym, Some(pagination));

        let proto = msg.to_proto();
        let decoded = QueryAllNymResponse::from_proto(&proto);

        assert_eq!(msg, decoded);
    }
}
