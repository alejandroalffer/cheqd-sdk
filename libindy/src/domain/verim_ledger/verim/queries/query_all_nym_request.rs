use super::super::super::cosmos::base::query::PageRequest;
use super::super::super::proto::verimid::verimnode::verim::QueryAllNymRequest as ProtoQueryAllNymRequest;
use super::super::super::VerimProto;
use indy_api_types::errors::IndyResult;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAllNymRequest {
    pub pagination: Option<PageRequest>,
}

impl QueryAllNymRequest {
    pub fn new(pagination: Option<PageRequest>) -> Self {
        QueryAllNymRequest { pagination }
    }
}

impl VerimProto for QueryAllNymRequest {
    type Proto = ProtoQueryAllNymRequest;

    fn to_proto(&self) -> Self::Proto {
        let pagination = match &self.pagination {
            Some(p) => Some(p.to_proto()),
            None => None,
        };
        Self::Proto { pagination }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto
                .pagination
                .as_ref()
                .map(|p| PageRequest::from_proto(p))
                .transpose()?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_query_get_all_nyms_request() {
        let pagination = PageRequest {
            key: vec![0],
            offset: 0,
            limit: 3,
            count_total: false,
        };
        let msg = QueryAllNymRequest::new(Some(pagination));

        let proto = msg.to_proto();
        let decoded = QueryAllNymRequest::from_proto(&proto).unwrap();

        assert_eq!(msg, decoded);
    }
}
