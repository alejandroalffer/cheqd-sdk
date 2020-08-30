use v3::messages::issuance::credential_request::CredentialRequest;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential::Credential;
use v3::messages::status::Status;
use v3::messages::error::{ProblemReport, Reason};
use v3::handlers::connection::types::CompletedConnection;
use messages::thread::Thread;

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
    pub offer: CredentialOffer,
    pub cred_data: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
    pub connection: CompletedConnection,
    #[serde(default)]
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestReceivedState {
    pub offer: CredentialOffer,
    pub cred_data: String,
    pub rev_reg_id: Option<String>,
    pub tails_file: Option<String>,
    pub request: CredentialRequest,
    pub connection: CompletedConnection,
    #[serde(default)]
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CredentialSentState {
    pub offer: CredentialOffer,
    pub connection: CompletedConnection,
    #[serde(default)]
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinishedState {
    pub offer: Option<CredentialOffer>,
    pub cred_id: Option<String>,
    pub status: Status,
    #[serde(default)]
    pub thread: Thread,
}

impl From<(InitialState, CredentialOffer, CompletedConnection, Thread)> for OfferSentState {
    fn from((state, offer, connection, thread): (InitialState, CredentialOffer, CompletedConnection, Thread)) -> Self {
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
            offer: state.offer,
            connection: state.connection,
            thread,
        }
    }
}

impl From<(OfferSentState, ProblemReport, Thread)> for FinishedState {
    fn from((state, err, thread): (OfferSentState, ProblemReport, Thread)) -> Self {
        trace!("IssuerSM: transit state from OfferSentState to FinishedState with ProblemReport: {:?}", err);
        trace!("Thread: {:?}", thread);
        FinishedState {
            cred_id: None,
            offer: Some(state.offer),
            status: Status::Failed(err),
            thread,
        }
    }
}

impl From<(RequestReceivedState, Thread)> for FinishedState {
    fn from((state, thread): (RequestReceivedState, Thread)) -> Self {
        trace!("IssuerSM: transit state from RequestReceivedState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            cred_id: None,
            offer: Some(state.offer),
            status: Status::Success,
            thread,
        }
    }
}

impl From<(RequestReceivedState, ProblemReport, Thread)> for FinishedState {
    fn from((state, err, thread): (RequestReceivedState, ProblemReport, Thread)) -> Self {
        trace!("IssuerSM: transit state from RequestReceivedState to FinishedState with ProblemReport: {:?}", err);
        trace!("Thread: {:?}", err.thread);
        FinishedState {
            cred_id: None,
            offer: Some(state.offer),
            status: Status::Failed(err),
            thread,
        }
    }
}

impl From<(CredentialSentState, Thread)> for FinishedState {
    fn from((state, thread): (CredentialSentState, Thread)) -> Self {
        trace!("IssuerSM: transit state from CredentialSentState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            cred_id: None,
            offer: Some(state.offer),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestSentState {
    pub offer: Option<CredentialOffer>,
    pub req_meta: String,
    pub cred_def_json: String,
    pub connection: CompletedConnection,
    #[serde(default)]
    pub thread: Thread,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfferReceivedState {
    pub offer: CredentialOffer,
    #[serde(default)]
    pub thread: Thread,
}

impl OfferReceivedState {
    pub fn new(offer: CredentialOffer) -> Self {
        OfferReceivedState {
            thread: Thread::new().set_thid(offer.id.to_string()),
            offer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinishedHolderState {
    pub offer: Option<CredentialOffer>,
    pub cred_id: Option<String>,
    pub credential: Option<Credential>,
    pub status: Status,
    #[serde(default)]
    pub thread: Thread,
}

impl From<(OfferReceivedState, String, String, CompletedConnection, Thread)> for RequestSentState {
    fn from((state, req_meta, cred_def_json, connection, thread): (OfferReceivedState, String, String, CompletedConnection, Thread)) -> Self {
        trace!("HolderSM: transit state from OfferReceivedState to RequestSentState");
        trace!("Thread: {:?}", state.thread);
        RequestSentState {
            offer: Some(state.offer),
            req_meta,
            cred_def_json,
            connection,
            thread,
        }
    }
}

impl From<(RequestSentState, String, Credential, Thread)> for FinishedHolderState {
    fn from((state, cred_id, credential, thread): (RequestSentState, String, Credential, Thread)) -> Self {
        trace!("HolderSM: transit state from RequestSentState to FinishedHolderState");
        trace!("Thread: {:?}", thread);
        FinishedHolderState {
            offer: state.offer,
            cred_id: Some(cred_id),
            credential: Some(credential),
            status: Status::Success,
            thread,
        }
    }
}

impl From<(RequestSentState, ProblemReport, Thread, Reason)> for FinishedHolderState {
    fn from((state, problem_report, thread, reason): (RequestSentState, ProblemReport, Thread, Reason)) -> Self {
        trace!("HolderSM: transit state from RequestSentState to FinishedHolderState with ProblemReport: {:?}", problem_report);
        trace!("Thread: {:?}", thread);
        FinishedHolderState {
            offer: state.offer,
            cred_id: None,
            credential: None,
            status: reason.to_status(problem_report),
            thread,
        }
    }
}

impl From<(OfferReceivedState, ProblemReport, Thread, Reason)> for FinishedHolderState {
    fn from((state, problem_report, thread, reason): (OfferReceivedState, ProblemReport, Thread, Reason)) -> Self {
        trace!("HolderSM: transit state from OfferReceivedState to FinishedHolderState with ProblemReport: {:?}", problem_report);
        trace!("Thread: {:?}", problem_report.thread);
        FinishedHolderState {
            offer: Some(state.offer),
            cred_id: None,
            credential: None,
            status: reason.to_status(problem_report),
            thread,
        }
    }
}