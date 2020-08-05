use v3::messages::discovery::disclose::ProtocolDescriptor;
use v3::handlers::connection::agent::AgentInfo;
use v3::handlers::connection::states::CompleteState;
use v3::messages::connection::invite::Invitation;
use v3::messages::outofband::invitation::Invitation as OutofbandInvitation;
use v3::messages::connection::did_doc::DidDoc;

/*
    object returning by vcx_connection_info
*/

#[derive(Debug, Serialize)]
pub struct PairwiseConnectionInfo {
    pub my: SideConnectionInfo,
    pub their: Option<SideConnectionInfo>,
    pub invitation: Option<Invitations>,
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
pub struct CompletedConnection {
    pub agent: AgentInfo,
    pub data: CompleteState,
}

/*
    helper structure to store Out-of-Band metadata
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutofbandMeta {
    pub goal_code: Option<String>,
    pub goal: Option<String>,
    pub handshake: bool,
    pub request_attach: Option<String>,
}

impl OutofbandMeta {
    pub fn new(goal_code: Option<String>, goal: Option<String>,
               handshake: bool, request_attach: Option<String>) -> OutofbandMeta {
        OutofbandMeta {
            goal_code,
            goal,
            handshake,
            request_attach,
        }
    }
}

/*
    Connection can be created with either Invitation of `connections` or `out-of-band` protocols
*/
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Invitations {
    ConnectionInvitation(Invitation),
    OutofbandInvitation(OutofbandInvitation),
}

impl From<Invitations> for DidDoc {
    fn from(invitation: Invitations) -> DidDoc {
        match invitation {
            Invitations::ConnectionInvitation(invitation_)=> DidDoc::from(invitation_),
            Invitations::OutofbandInvitation(invitation_)=> DidDoc::from(invitation_),
        }
    }
}

impl Invitations {
    pub fn recipient_key(&self) -> Option<String> {
        match self {
            Invitations::ConnectionInvitation(invitation_)=>
                invitation_.recipient_keys.get(0).cloned(),
            Invitations::OutofbandInvitation(invitation_)=>
                invitation_.service.get(0).and_then(|service| service.recipient_keys.get(0).cloned()),
        }
    }

    pub fn pthid(&self) -> Option<String>{
        match self {
            Invitations::ConnectionInvitation(_)=> None,
            Invitations::OutofbandInvitation(invitation_)=> Some(invitation_.id.to_string()),
        }
    }
}