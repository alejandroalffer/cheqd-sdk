use v3::messages::discovery::disclose::ProtocolDescriptor;
use v3::handlers::connection::agent::AgentInfo;
use v3::messages::connection::did_doc::DidDoc;

/*
    object returning by vcx_connection_info
*/

#[derive(Debug, Serialize)]
pub struct PairwiseConnectionInfo {
    pub my: SideConnectionInfo,
    pub their: Option<SideConnectionInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SideConnectionInfo {
    pub did: String,
    pub recipient_keys: Vec<String>,
    pub routing_keys: Vec<String>,
    pub service_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<ProtocolDescriptor>>,
}

/*
    object store within Issuer / Holder / Verifier / Prover
    state machines as relationship to specific pairwise connection
*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InternalConnectionInfo {
    pub agent: AgentInfo,
    pub remote_did_doc: DidDoc,
}