use indy_api_types::errors::IndyResult;

use super::super::super::base::query::PageResponse;

use super::super::models::Nym;
use super::super::super::proto::cheqdid::cheqdnode::cheqd::Nym as ProtoNym;
use super::super::super::proto::cheqdid::cheqdnode::cheqd::QueryAllNymResponse as ProtoQueryAllNymResponse;
use super::super::super::CheqdProto;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAllNymResponse {
    pub nym: Vec<Nym>,
    pub pagination: Option<PageResponse>,
}

impl QueryAllNymResponse {
    pub fn new(nym: Vec<Nym>, pagination: Option<PageResponse>) -> Self {
        QueryAllNymResponse { nym, pagination }
    }
}

impl CheqdProto for QueryAllNymResponse {
    type Proto = ProtoQueryAllNymResponse;

    fn to_proto(&self) -> Self::Proto {
        let nym: Vec<ProtoNym> = self.nym.iter().map(|n| n.to_proto()).collect();
        let pagination = match &self.pagination {
            Some(p) => Some(p.to_proto()),
            None => None,
        };
        Self::Proto { nym, pagination }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        let nym: IndyResult<Vec<Nym>> = proto.nym.iter().map(|n| Nym::from_proto(n)).collect();
        let nym = nym?;

        let pagination = proto
            .pagination
            .as_ref()
            .map(|p| PageResponse::from_proto(p))
            .transpose()?;

        Ok(Self::new(nym, pagination))
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

        let pagination = PageResponse {
            next_key: vec![0],
            total: 3,
        };
        let msg = QueryAllNymResponse::new(nym, Some(pagination));

        let proto = msg.to_proto();
        let decoded = QueryAllNymResponse::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
