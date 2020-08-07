use error::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AgencyErrorCode {
    #[serde(rename = "GNR-115")]
    InvalidValue,
    #[serde(rename = "CS-101")]
    AlreadyConnected,
    #[serde(rename = "CS-102")]
    NotConnected,
    #[serde(rename = "CS-103")]
    ConnectionIsDeleted,
    #[serde(rename = "CS-104")]
    PairwiseKeyAlreadyInWallet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgencyError {
    pub status_code: AgencyErrorCode,
    pub status_msg: String,
}

impl AgencyError {
    pub fn from_response(error: &str) -> Option<AgencyError> {
        serde_json::from_str::<AgencyError>(error).ok()
    }

    pub fn to_vcx_error(self) -> VcxError {
        match self.status_code {
            AgencyErrorCode::InvalidValue =>
                VcxError::from_msg(VcxErrorKind::InvalidAgencyRequest,
                                   format!("Sending message on the Agency failed. Err: {:?}", self.status_msg)),
            AgencyErrorCode::AlreadyConnected =>
                VcxError::from_msg(VcxErrorKind::ConnectionAlreadyExists,
                                   "Connection invitation has been already accepted. You have to use another invitation to set up a new connection."),
            AgencyErrorCode::NotConnected =>
                VcxError::from_msg(VcxErrorKind::ConnectionDoesNotExist,
                                   "Connection does not exist."),
            AgencyErrorCode::ConnectionIsDeleted =>
                VcxError::from_msg(VcxErrorKind::ConnectionDoesNotExist,
                                   "Connection is deleted."),
            AgencyErrorCode::PairwiseKeyAlreadyInWallet =>
                VcxError::from_msg(VcxErrorKind::ConnectionAlreadyExists,
                                   "Connection invitation has been already accepted. You have to use another invitation to set up a new connection.")
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn _invalid_value() -> &'static str {
        r#"{"statusCode":"GNR-115","statusMsg":"no pairwise connection found with these DIDs: abc"}"#
    }

    fn _pairwise_key_already_in_wallet() -> &'static str {
        r#"{"statusCode":"CS-104","statusMsg":"pairwise key already in wallet"}"#
    }

    #[test]
    fn test_parse_agency_error() {
        let error = AgencyError::from_response(_invalid_value()).unwrap();
        assert_eq!(AgencyErrorCode::InvalidValue, error.status_code);

        let error = AgencyError::from_response(_pairwise_key_already_in_wallet()).unwrap();
        assert_eq!(AgencyErrorCode::PairwiseKeyAlreadyInWallet, error.status_code);
    }

    #[test]
    fn test_agency_error_to_vcx_error() {
        let error = AgencyError::from_response(_invalid_value()).unwrap();
        assert_eq!(VcxErrorKind::InvalidAgencyRequest, error.to_vcx_error().kind());

        let error = AgencyError::from_response(_pairwise_key_already_in_wallet()).unwrap();
        assert_eq!(VcxErrorKind::ConnectionAlreadyExists, error.to_vcx_error().kind());
    }
}