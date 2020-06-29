use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential::Credential;
use v3::messages::status::Status;
use v3::messages::error::ProblemReport;
use v3::messages::connection::did_doc::DidDoc;
use v3::handlers::connection::agent::AgentInfo;
use v3::handlers::connection::types::CompletedConnectionInfo;
use messages::thread::Thread;

use error::{VcxResult, VcxError, VcxErrorKind};

// Possible Transitions:
// Initial -> OfferSent
// Initial -> Finished
// OfferSent -> CredentialSent
// OfferSent -> Finished
// CredentialSent -> Finished
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IssuerState {
    Initial(InitialState),
    OfferSent(OfferSentState),
    RequestReceived(RequestReceivedState),
    CredentialSent(CredentialSentState),
    Finished(FinishedState),
}

impl IssuerState {
    pub fn get_agent_info(&self) -> VcxResult<&AgentInfo> {
        match self {
            IssuerState::OfferSent(ref state) => {
                Ok(&state.connection.agent)
            }
            IssuerState::RequestReceived(ref state) => {
                Ok(&state.connection.agent)
            }
            IssuerState::CredentialSent(ref state) => {
                Ok(&state.connection.agent)
            }
            IssuerState::Initial(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected IssuerSM state: could not get Connection AgentInfo"))
            }
            IssuerState::Finished(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected IssuerSM state: could not get Connection AgentInfo"))
            }
        }
    }

    pub fn remote_connection_info(&self) -> VcxResult<&DidDoc> {
        match self {
            IssuerState::OfferSent(ref state) => {
                Ok(&state.connection.remote_did_doc)
            }
            IssuerState::RequestReceived(ref state) => {
                Ok(&state.connection.remote_did_doc)
            }
            IssuerState::CredentialSent(ref state) => {
                Ok(&state.connection.remote_did_doc)
            }
            IssuerState::Initial(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected IssuerSM state: could not get Connection Remote DidDoc"))
            }
            IssuerState::Finished(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected IssuerSM state: could not get Connection Remote DidDoc"))
            }
        }
    }

    pub fn thread_id(&self) -> String {
        match self {
            IssuerState::Initial(_) => String::new(),
            IssuerState::OfferSent(state) => state.thread.thid.clone().unwrap_or_default(),
            IssuerState::RequestReceived(state) => state.thread.thid.clone().unwrap_or_default(),
            IssuerState::CredentialSent(state) => state.thread.thid.clone().unwrap_or_default(),
            IssuerState::Finished(state) => state.thread.thid.clone().unwrap_or_default(),
        }
    }
}

impl InitialState {
    pub fn new(cred_def_id: &str, credential_json: &str, rev_reg_id: Option<String>, tails_file: Option<String>, credential_name: Option<String>) -> Self {
        InitialState {
            cred_def_id: cred_def_id.to_string(),
            credential_json: credential_json.to_string(),
            rev_reg_id,
            tails_file,
            credential_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitialState {
    pub cred_def_id: String,
    pub credential_json: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfferSentState {
    pub offer: String,
    pub cred_data: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
    pub connection: CompletedConnectionInfo,
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestReceivedState {
    pub offer: String,
    pub cred_data: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
    pub request: CredentialRequest,
    pub connection: CompletedConnectionInfo,
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CredentialSentState {
    pub connection: CompletedConnectionInfo,
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinishedState {
    pub cred_id: Option<String>,
    pub status: Status,
    pub thread: Thread,
}

impl From<(InitialState, String, CompletedConnectionInfo, Thread)> for OfferSentState {
    fn from((state, offer, connection, thread): (InitialState, String, CompletedConnectionInfo, Thread)) -> Self {
        trace!("IssuerSM: transit state from InitialState to OfferSentState");
        trace!("Thread: {:?}", thread);
        OfferSentState {
            offer,
            cred_data: state.credential_json,
            rev_reg_id: state.rev_reg_id,
            tails_file: state.tails_file,
            connection,
            thread,
        }
    }
}

impl From<InitialState> for FinishedState {
    fn from(_state: InitialState) -> Self {
        trace!("IssuerSM: transit state from InitialState to FinishedState");
        FinishedState {
            cred_id: None,
            status: Status::Undefined,
            thread: Thread::default(),
        }
    }
}

impl From<(OfferSentState, CredentialRequest, Thread)> for RequestReceivedState {
    fn from((state, request, thread): (OfferSentState, CredentialRequest, Thread)) -> Self {
        trace!("IssuerSM: transit state from OfferSentState to RequestReceivedState");
        trace!("Thread: {:?}", thread);
        RequestReceivedState {
            offer: state.offer,
            cred_data: state.cred_data,
            rev_reg_id: state.rev_reg_id,
            tails_file: state.tails_file,
            request,
            connection: state.connection,
            thread,
        }
    }
}

impl From<(RequestReceivedState, Thread)> for CredentialSentState {
    fn from((state, thread): (RequestReceivedState, Thread)) -> Self {
        trace!("IssuerSM: transit state from RequestReceivedState to CredentialSentState");
        trace!("Thread: {:?}", thread);
        CredentialSentState {
            connection: state.connection,
            thread,
        }
    }
}

impl From<(OfferSentState, Thread)> for FinishedState {
    fn from((_state, thread): (OfferSentState, Thread)) -> Self {
        trace!("IssuerSM: transit state from OfferSentState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            cred_id: None,
            status: Status::Undefined,
            thread,
        }
    }
}

impl From<(OfferSentState, ProblemReport, Thread)> for FinishedState {
    fn from((_state, err, thread): (OfferSentState, ProblemReport, Thread)) -> Self {
        trace!("IssuerSM: transit state from OfferSentState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            cred_id: None,
            status: Status::Failed(err),
            thread,
        }
    }
}

impl From<(RequestReceivedState, Thread)> for FinishedState {
    fn from((_state, thread): (RequestReceivedState, Thread)) -> Self {
        trace!("IssuerSM: transit state from RequestReceivedState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            cred_id: None,
            status: Status::Success,
            thread,
        }
    }
}

impl From<(RequestReceivedState, ProblemReport, Thread)> for FinishedState {
    fn from((_state, err, thread): (RequestReceivedState, ProblemReport, Thread)) -> Self {
        trace!("IssuerSM: transit state from RequestReceivedState to FinishedState");
        trace!("Thread: {:?}", err.thread);
        FinishedState {
            cred_id: None,
            status: Status::Failed(err),
            thread,
        }
    }
}

impl From<(CredentialSentState, Thread)> for FinishedState {
    fn from((_state, thread): (CredentialSentState, Thread)) -> Self {
        trace!("IssuerSM: transit state from CredentialSentState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            cred_id: None,
            status: Status::Success,
            thread,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HolderState {
    OfferReceived(OfferReceivedState),
    RequestSent(RequestSentState),
    Finished(FinishedHolderState),
}

impl HolderState {
    pub fn get_agent_info(&self) -> VcxResult<&AgentInfo> {
        match self {
            HolderState::RequestSent(ref state) => {
                Ok(&state.connection.agent)
            }
            HolderState::OfferReceived(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected Holder state: could not get Connection  AgentInfo"))
            }
            HolderState::Finished(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected Holder state: could not get Connection  AgentInfo"))
            }
        }
    }

    pub fn remote_connection_info(&self) -> VcxResult<&DidDoc> {
        match self {
            HolderState::RequestSent(ref state) => {
                Ok(&state.connection.remote_did_doc)
            }
            HolderState::OfferReceived(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected Holder state: could not get Connection  Remote DidDoc"))
            }
            HolderState::Finished(_) => {
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Unexpected Holder state: could not get Connection  Remote DidDoc"))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestSentState {
    pub req_meta: String,
    pub cred_def_json: String,
    pub connection: CompletedConnectionInfo,
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfferReceivedState {
    pub offer: CredentialOffer,
    pub thread: Thread,
}

impl OfferReceivedState {
    pub fn new(offer: CredentialOffer) -> Self {
        OfferReceivedState {
            thread: offer.thread.clone().unwrap_or_default(),
            offer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinishedHolderState {
    pub cred_id: Option<String>,
    pub credential: Option<Credential>,
    pub status: Status,
    pub thread: Thread,
}

impl From<(OfferReceivedState, String, String, CompletedConnectionInfo, Thread)> for RequestSentState {
    fn from((state, req_meta, cred_def_json, connection, thread): (OfferReceivedState, String, String, CompletedConnectionInfo, Thread)) -> Self {
        trace!("HolderSM: transit state from OfferReceivedState to RequestSentState");
        trace!("Thread: {:?}", state.thread);
        RequestSentState {
            req_meta,
            cred_def_json,
            connection,
            thread,
        }
    }
}

impl From<(RequestSentState, String, Credential, Thread)> for FinishedHolderState {
    fn from((_, cred_id, credential, thread): (RequestSentState, String, Credential, Thread)) -> Self {
        trace!("HolderSM: transit state from RequestSentState to FinishedHolderState");
        trace!("Thread: {:?}", thread);
        FinishedHolderState {
            cred_id: Some(cred_id),
            credential: Some(credential),
            status: Status::Success,
            thread,
        }
    }
}

impl From<(RequestSentState, ProblemReport, Thread)> for FinishedHolderState {
    fn from((_, problem_report, thread): (RequestSentState, ProblemReport, Thread)) -> Self {
        trace!("HolderSM: transit state from RequestSentState to FinishedHolderState");
        trace!("Thread: {:?}", thread);
        FinishedHolderState {
            cred_id: None,
            credential: None,
            status: Status::Failed(problem_report),
            thread,
        }
    }
}

impl From<(OfferReceivedState, ProblemReport, Thread)> for FinishedHolderState {
    fn from((_state, problem_report, thread): (OfferReceivedState, ProblemReport, Thread)) -> Self {
        trace!("HolderSM: transit state from OfferReceivedState to FinishedHolderState");
        trace!("Thread: {:?}", problem_report.thread);
        FinishedHolderState {
            cred_id: None,
            credential: None,
            status: Status::Failed(problem_report),
            thread,
        }
    }
}