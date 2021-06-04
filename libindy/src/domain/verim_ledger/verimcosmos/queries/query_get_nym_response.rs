use super::super::super::proto::verimid::verimcosmos::verimcosmos::QueryGetNymResponse as ProtoQueryGetNymResponse;
use super::super::super::VerimProto;
use super::super::models::nym::Nym;
use indy_api_types::errors::IndyResult;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryGetNymResponse {
    pub nym: Option<Nym>,
}

impl QueryGetNymResponse {
    pub fn new(nym: Option<Nym>) -> Self {
        QueryGetNymResponse { nym }
    }
}

impl VerimProto for QueryGetNymResponse {
    type Proto = ProtoQueryGetNymResponse;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            nym: self.nym.as_ref().map(|n| n.to_proto()),
        }
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto
                .nym
                .as_ref()
                .map(|n| Nym::from_proto(&n))
                .transpose()?,
        ))
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
