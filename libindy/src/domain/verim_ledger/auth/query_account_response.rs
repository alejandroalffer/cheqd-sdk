use cosmos_sdk::proto::cosmos::auth::v1beta1::QueryAccountResponse as ProtoQueryAccountResponse;
use indy_api_types::errors::IndyResult;

use super::super::VerimProto;

use super::Account;

/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    pub account: Option<Account>,
}

impl QueryAccountResponse {
    pub fn new(account: Option<Account>) -> Self {
        QueryAccountResponse { account }
    }
}

impl VerimProto for QueryAccountResponse {
    type Proto = ProtoQueryAccountResponse;

    fn to_proto(&self) -> Self::Proto {
        unimplemented!()
    }

    fn from_proto(proto: &Self::Proto) -> IndyResult<Self> {
        Ok(Self::new(
            proto
                .account
                .as_ref()
                .map(|acc| Account::from_proto(acc))
                .transpose()?,
        ))
    }
}
