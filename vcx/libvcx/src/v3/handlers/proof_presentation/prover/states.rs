use api::VcxStateType;

use v3::handlers::proof_presentation::prover::messages::ProverMessages;
use v3::messages::a2a::A2AMessage;
use v3::messages::proof_presentation::presentation_request::PresentationRequest;
use v3::messages::proof_presentation::presentation_proposal::{PresentationProposal, PresentationPreview};
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::proof_presentation::presentation_ack::PresentationAck;
use v3::messages::error::{ProblemReport, ProblemReportCodes, Reason};
use v3::messages::status::Status;

use std::collections::HashMap;
use disclosed_proof::DisclosedProof;

use error::prelude::*;
use messages::thread::Thread;
use v3::messages::ack::Ack;
use v3::handlers::connection::types::CompletedConnection;
use v3::handlers::connection::agent::AgentInfo;
use v3::handlers::connection::connection::Connection;

/// A state machine that tracks the evolution of states for a Prover during
/// the Present Proof protocol.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProverSM {
    source_id: String,
    state: ProverState,
}

impl ProverSM {
    pub fn new(presentation_request: PresentationRequest, source_id: String) -> ProverSM {
        ProverSM {
            source_id,
            state: ProverState::RequestReceived(
                RequestReceivedState {
                    thread: Thread::new().set_thid(presentation_request.id.to_string()),
                    presentation_request,
                    presentation_proposal: None
                }
            ),
        }
    }

    pub fn new_proposal(presentation_proposal: PresentationProposal, source_id: String) -> ProverSM {
        ProverSM {
            source_id,
            state: ProverState::ProposalPrepared(
                ProposalPreparedState {
                    presentation_proposal,
                    thread: Thread::new()
                }
            )
        }
    }
}

// Possible Transitions:
//
// Initial -> PresentationPrepared, PresentationPreparationFailedState, Finished
// PresentationPrepared -> PresentationSent, Finished
// PresentationPreparationFailedState -> Finished
// PresentationSent -> Finished
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProverState {
    RequestReceived(RequestReceivedState),
    PresentationPrepared(PresentationPreparedState),
    PresentationPreparationFailed(PresentationPreparationFailedState),
    ProposalPrepared(ProposalPreparedState),
    PresentationSent(PresentationSentState),
    ProposalSent(ProposalSentState),
    Finished(FinishedState),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestReceivedState {
    presentation_request: PresentationRequest,
    presentation_proposal: Option<PresentationProposal>,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationPreparedState {
    presentation_request: PresentationRequest,
    presentation: Presentation,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationPreparationFailedState {
    presentation_request: PresentationRequest,
    problem_report: ProblemReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_kind: Option<VcxErrorKind>,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationSentState {
    presentation_request: PresentationRequest,
    presentation: Presentation,
    connection: CompletedConnection,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProposalPreparedState {
    presentation_proposal: PresentationProposal,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProposalSentState {
    presentation_proposal: PresentationProposal,
    presentation_request: Option<PresentationRequest>,
    connection: CompletedConnection,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    presentation_request: PresentationRequest,
    presentation: Presentation,
    status: Status,
    #[serde(default)]
    thread: Thread,
}

impl From<(ProposalSentState, PresentationRequest, Thread)> for RequestReceivedState {
    fn from((state, presentation_request, thread): (ProposalSentState, PresentationRequest, Thread)) -> Self {
        trace!("ProverSM transit state from ProposalSentState to RequestReceivedState");
        trace!("Thread: {:?}", thread);
        RequestReceivedState {
            presentation_request,
            presentation_proposal: Some(state.presentation_proposal),
            thread,
        }
    }
}

impl From<(RequestReceivedState, Presentation, Thread)> for PresentationPreparedState {
    fn from((state, presentation, thread): (RequestReceivedState, Presentation, Thread)) -> Self {
        trace!("ProverSM transit state from RequestReceivedState to PresentationPreparedState");
        trace!("Thread: {:?}", thread);
        PresentationPreparedState {
            presentation_request: state.presentation_request,
            thread,
            presentation,
        }
    }
}

impl From<(RequestReceivedState, ProblemReport, VcxErrorKind, Thread)> for PresentationPreparationFailedState {
    fn from((state, problem_report, error_kind, thread): (RequestReceivedState, ProblemReport, VcxErrorKind, Thread)) -> Self {
        trace!("ProverSM transit state from RequestReceivedState to PresentationPreparationFailedState with ProblemReport: {:?}", problem_report);
        trace!("Thread: {:?}", thread);
        PresentationPreparationFailedState {
            presentation_request: state.presentation_request,
            thread,
            problem_report,
            error_kind: Some(error_kind),
        }
    }
}

impl From<(RequestReceivedState, CompletedConnection, PresentationProposal, Thread)> for ProposalSentState {
    fn from((state, connection, presentation_proposal, thread): (RequestReceivedState, CompletedConnection, PresentationProposal, Thread)) -> Self {
        trace!("ProverSM transit state from RequestReceivedState to ProposalSentState");
        trace!("Thread: {:?}", thread);
        ProposalSentState {
            presentation_proposal,
            presentation_request: Some(state.presentation_request),
            connection,
            thread,
        }
    }
}

impl From<(ProposalPreparedState, CompletedConnection, PresentationProposal, Thread)> for ProposalSentState {
    fn from((_state, connection, presentation_proposal, thread): (ProposalPreparedState, CompletedConnection, PresentationProposal, Thread)) -> Self {
        trace!("ProverSM transit state from ProposalPreparedState to ProposalSentState");
        trace!("Thread: {:?}", thread);
        ProposalSentState {
            presentation_proposal,
            presentation_request: None,
            connection,
            thread,
        }
    }
}

impl From<(RequestReceivedState, Thread, ProblemReport, Reason)> for FinishedState {
    fn from((state, thread, problem_report, reason): (RequestReceivedState, Thread, ProblemReport, Reason)) -> Self {
        trace!("ProverSM transit state from RequestReceivedState to FinishedState with DeclineProof message");
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: Default::default(),
            status: reason.to_status(problem_report),
            thread,
        }
    }
}

impl From<(RequestReceivedState, Thread, PresentationProposal, Reason)> for FinishedState {
    fn from((state, thread, _presentation_proposal, _reason): (RequestReceivedState, Thread, PresentationProposal, Reason)) -> Self {
        trace!("ProverSM transit state from RequestReceivedState to FinishedState with PresentationProposal message");
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: Default::default(),
            status: Status::Rejected,
            thread,
        }
    }
}

impl From<(PresentationPreparedState, CompletedConnection, Presentation, Thread)> for PresentationSentState {
    fn from((state, connection, presentation, thread): (PresentationPreparedState, CompletedConnection, Presentation, Thread)) -> Self {
        trace!("ProverSM transit state from PresentationPreparedState to PresentationSentState");
        trace!("Thread: {:?}", thread);
        PresentationSentState {
            presentation_request: state.presentation_request,
            presentation,
            connection,
            thread,
        }
    }
}

impl From<(PresentationPreparedState, Thread)> for FinishedState {
    fn from((state, thread): (PresentationPreparedState, Thread)) -> Self {
        trace!("ProverSM transit state from PresentationPreparedState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: state.presentation,
            status: Status::Success,
            thread,
        }
    }
}

impl From<(PresentationPreparedState, Thread, ProblemReport, Reason)> for FinishedState {
    fn from((state, thread, problem_report, reason): (PresentationPreparedState, Thread, ProblemReport, Reason)) -> Self {
        trace!("ProverSM transit state from PresentationPreparedState to FinishedState with DeclineProof message");
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: Default::default(),
            status: reason.to_status(problem_report),
            thread,
        }
    }
}

impl From<(PresentationPreparedState, Thread, PresentationProposal, Reason)> for FinishedState {
    fn from((state, thread, _presentation_proposal, _reason): (PresentationPreparedState, Thread, PresentationProposal, Reason)) -> Self {
        trace!("ProverSM transit state from PresentationPreparedState to FinishedState with DeclineProof message");
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: Default::default(),
            status: Status::Rejected,
            thread,
        }
    }
}

impl From<(PresentationPreparationFailedState, Thread)> for FinishedState {
    fn from((state, thread): (PresentationPreparationFailedState, Thread)) -> Self {
        trace!("ProverSM transit state from PresentationPreparationFailedState to FinishedState with ProblemReport: {:?}", state.problem_report);
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: Presentation::create(),
            status: Status::Failed(state.problem_report),
            thread,
        }
    }
}

impl From<(PresentationSentState, PresentationAck, Thread)> for FinishedState {
    fn from((state, _ack, thread): (PresentationSentState, PresentationAck, Thread)) -> Self {
        trace!("ProverSM transit state from PresentationSentState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: state.presentation,
            status: Status::Success,
            thread,
        }
    }
}

impl From<(PresentationSentState, ProblemReport, Thread, Reason)> for FinishedState {
    fn from((state, problem_report, thread, reason): (PresentationSentState, ProblemReport, Thread, Reason)) -> Self {
        trace!("ProverSM transit state from PresentationSentState to FinishedState with ProblemReport: {:?}", problem_report);
        trace!("Thread: {:?}", problem_report.thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: state.presentation,
            status: reason.to_status(problem_report),
            thread,
        }
    }
}

impl RequestReceivedState {
    fn build_presentation(&self, credentials: &str, self_attested_attrs: &str) -> VcxResult<String> {
        DisclosedProof::generate_indy_proof(credentials,
                                            self_attested_attrs,
                                            &self.presentation_request.request_presentations_attach.content()?)
    }
}

impl PresentationSentState {
    fn handle_ack(&self, ack: &Ack) -> VcxResult<()> {
        trace!("PresentationSentState::handle_ack >>> ack: {:?}", secret!(ack));
        debug!("prover handling received presentation ack message");
        self.thread.check_message_order(&self.connection.data.did_doc.id, &ack.thread)?;
        Ok(())
    }
}


impl ProverSM {
    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, A2AMessage)> {
        trace!("Prover::find_message_to_handle >>> messages: {:?}", secret!(messages));
        debug!("Prover: Finding message to update state");

        for (uid, message) in messages {
            match self.state {
                ProverState::RequestReceived(_) => {
                    match message {
                        A2AMessage::PresentationRequest(_) => {
                            // ignore it here??
                        }
                        message => {
                            warn!("Prover: Unexpected message received in Initiated state: {:?}", message);
                        }
                    }
                }
                ProverState::PresentationPrepared(_) => {
                    // do not process messages
                }
                ProverState::PresentationPreparationFailed(_) => {
                    // do not process messages
                }
                ProverState::PresentationSent(ref state) => {
                    match message {
                        A2AMessage::Ack(ack) | A2AMessage::PresentationAck(ack) => {
                            if ack.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                debug!("Prover: Ack message received");
                                return Some((uid, A2AMessage::PresentationAck(ack)));
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) |
                        A2AMessage::PresentationReject(problem_report) => {
                            if problem_report.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                debug!("Prover: PresentationReject message received");
                                return Some((uid, A2AMessage::CommonProblemReport(problem_report)));
                            }
                        }
                        message => {
                            warn!("Prover: Unexpected message received in PresentationSent state: {:?}", message);
                        }
                    }
                }
                ProverState::ProposalPrepared(_) => {
                    // do not process messages
                }
                ProverState::ProposalSent(_) => {
                    match message {
                        A2AMessage::PresentationRequest(request) => {
                            return Some((uid, A2AMessage::PresentationRequest(request)))
                        }
                        message => {
                            warn!("Prover: Unexpected message received in PresentationSent state: {:?}", message);
                        }
                    }
                }
                ProverState::Finished(_) => {
                    // do not process messages
                }
            };
        }
        debug!("Prover: no message to update state");
        None
    }

    pub fn step(self, message: ProverMessages) -> VcxResult<ProverSM> {
        trace!("ProverSM::step >>> message: {:?}", secret!(message));
        debug!("Prover: Updating state");

        let ProverSM { source_id, state } = self;

        let state = match state {
            ProverState::RequestReceived(state) => {
                let thread = state.thread.clone();

                match message {
                    ProverMessages::SetPresentation(presentation) => {
                        let presentation = presentation.set_thread(thread.clone());
                        ProverState::PresentationPrepared((state, presentation, thread).into())
                    }
                    ProverMessages::PreparePresentation((credentials, self_attested_attrs)) => {
                        match state.build_presentation(&credentials, &self_attested_attrs) {
                            Ok(presentation) => {
                                let presentation = Presentation::create()
                                    .set_comment(state.presentation_request.comment.clone())
                                    .ask_for_ack()
                                    .set_thread(thread.clone())
                                    .set_presentations_attach(presentation)?;
                                ProverState::PresentationPrepared((state, presentation, thread).into())
                            }
                            Err(err) => {
                                let problem_report =
                                    ProblemReport::create()
                                        .set_description(ProblemReportCodes::InvalidPresentationRequest)
                                        .set_comment(err.to_string())
                                        .set_thread(thread.clone());
                                ProverState::PresentationPreparationFailed((state, problem_report, err.kind(), thread).into())
                            }
                        }
                    }
                    ProverMessages::RejectPresentationRequest((connection_handle, reason)) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;
                        let thread = thread.clone().update_received_order(&connection.data.did_doc.id);
                        let problem_report = Self::_handle_reject_presentation_request(&connection, &reason, &state.presentation_request, &thread)?;
                        ProverState::Finished((state, thread, problem_report, Reason::Reject).into())
                    }
                    ProverMessages::ProposePresentation((connection_handle, preview)) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;
                        let thread = thread.clone().update_received_order(&connection.data.did_doc.id);
                        let presentation_proposal = Self::_handle_presentation_proposal(&connection, preview, &state.presentation_request, &thread)?;
                        ProverState::ProposalSent((state, connection, presentation_proposal, thread).into())
                    }
                    message_ => {
                        warn!("Prover: Unexpected action to update state {:?}", message_);
                        ProverState::RequestReceived(state)
                    }
                }
            }
            ProverState::PresentationPrepared(state) => {
                match message {
                    ProverMessages::SendPresentation(connection_handle) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;
                        let thread = state.thread.clone()
                            .update_received_order(&connection.data.did_doc.id)
                            .set_opt_pthid(connection.data.thread.pthid.clone());

                        let presentation = state.presentation.clone()
                            .set_thread(thread.clone());

                        match state.presentation_request.service.clone() {
                            None => {
                                connection.data.send_message(&presentation.to_a2a_message(), &connection.agent)?;
                                ProverState::PresentationSent((state, connection, presentation, thread).into())
                            }
                            Some(service) => {
                                Connection::send_message_to_self_endpoint(&presentation.to_a2a_message(), &service.into())?;
                                ProverState::Finished((state, thread).into())
                            }
                        }
                    }
                    ProverMessages::RejectPresentationRequest((connection_handle, reason)) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;
                        let thread = state.thread.clone().update_received_order(&connection.data.did_doc.id);
                        let problem_report = Self::_handle_reject_presentation_request(&connection, &reason, &state.presentation_request, &thread)?;
                        ProverState::Finished((state, thread, problem_report, Reason::Reject).into())
                    }
                    ProverMessages::ProposePresentation((connection_handle, preview)) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;
                        let thread = state.thread.clone().update_received_order(&connection.data.did_doc.id);
                        let presentation_proposal = Self::_handle_presentation_proposal(&connection, preview, &state.presentation_request, &thread)?;
                        ProverState::Finished((state, thread, presentation_proposal, Reason::Reject).into())
                    }
                    message_ => {
                        warn!("Prover: Unexpected action to update state {:?}", message_);
                        ProverState::PresentationPrepared(state)
                    }
                }
            }
            ProverState::PresentationPreparationFailed(state) => {
                match message {
                    ProverMessages::SendPresentation(connection_handle) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;
                        let thread = state.thread.clone()
                            .update_received_order(&connection.data.did_doc.id);
                        let error_kind = state.error_kind
                            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Prover object state: `error_kind` not found", source_id)))?;

                        let problem_report = A2AMessage::PresentationReject(
                            state.problem_report.clone()
                                .set_thread(thread.clone())
                        );

                        match state.presentation_request.service.clone() {
                            None => {
                                connection.data.send_message(&problem_report, &connection.agent)?;
                            }
                            Some(service) => {
                                Connection::send_message_to_self_endpoint(&problem_report, &service.into())?;
                            }
                        }
                        return Err(VcxError::from_msg(error_kind, state.problem_report.comment.unwrap_or_default()));
                    }
                    message_ => {
                        warn!("Prover: Unexpected action to update state {:?}", message_);
                        ProverState::PresentationPreparationFailed(state)
                    }
                }
            }
            ProverState::PresentationSent(state) => {
                match message {
                    ProverMessages::PresentationAckReceived(ack) => {
                        let mut thread = state.thread.clone()
                            .update_received_order(&state.connection.data.did_doc.id);

                        match state.handle_ack(&ack) {
                            Ok(()) => {
                                ProverState::Finished((state, ack, thread).into())
                            }
                            Err(err) => {
                                thread = thread.increment_sender_order();

                                let problem_report = ProblemReport::create()
                                    .set_description(ProblemReportCodes::Other(String::from("invalid-message-state")))
                                    .set_comment(format!("error occurred: {:?}", err))
                                    .set_thread(thread.clone());

                                state.connection.data.send_message(&A2AMessage::PresentationReject(problem_report.clone()), &state.connection.agent)?;
                                return Err(err);
                            }
                        }
                    }
                    ProverMessages::PresentationRejectReceived(problem_report) => {
                        let thread = state.thread.clone()
                            .update_received_order(&state.connection.data.did_doc.id);

                        ProverState::Finished((state, problem_report, thread, Reason::Fail).into())
                    }
                    ProverMessages::RejectPresentationRequest(_) => {
                        return Err(VcxError::from_msg(VcxErrorKind::InvalidState, "Presentation is already sent"));
                    }
                    message_ => {
                        warn!("Prover: Unexpected action to update state {:?}", message_);
                        ProverState::PresentationSent(state)
                    }
                }
            }
            ProverState::Finished(state) => ProverState::Finished(state),
            ProverState::ProposalPrepared(state) => {
                match message {
                    ProverMessages::SendProposal(connection_handle) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;
                        let thread = state.thread.clone().update_received_order(&connection.data.did_doc.id);
                        let presentation_proposal = state.presentation_proposal.clone();

                        connection.data.send_message(&presentation_proposal.to_a2a_message(), &connection.agent)?;

                        ProverState::ProposalSent((state, connection, presentation_proposal, thread).into())
                    }
                    message_ => {
                        warn!("Prover: Unexpected action to update state {:?}", message_);
                        ProverState::ProposalPrepared(state)
                    }

                }
            }
            ProverState::ProposalSent(state) => {
                match message {
                    ProverMessages::PresentationRequestReceived(presentation_request) => {
                        let thread = state.thread.clone()
                            .update_received_order(&state.connection.data.did_doc.id)
                            .set_opt_pthid(state.connection.data.thread.pthid.clone());

                        ProverState::RequestReceived((state, presentation_request, thread).into())
                    }
                    message_ => {
                        warn!("Prover: Unexpected action to update state {:?}", message_);
                        ProverState::ProposalSent(state)
                    }

                }
            }
        };

        trace!("Prover::step <<< state: {:?}", secret!(state));
        Ok(ProverSM { source_id, state })
    }

    fn _handle_reject_presentation_request(connection: &CompletedConnection, reason: &str, presentation_request: &PresentationRequest, thread: &Thread) -> VcxResult<ProblemReport> {
        trace!("ProverSM::_handle_reject_presentation_request >>> reason: {:?}, presentation_request: {:?}", secret!(reason), secret!(presentation_request));
        debug!("Prover: Rejecting presentation request");

        let problem_report = ProblemReport::create()
            .set_description(ProblemReportCodes::PresentationRejected)
            .set_comment(reason.to_string())
            .set_thread(thread.clone());

        match presentation_request.service.clone() {
            None => connection.data.send_message(&A2AMessage::PresentationReject(problem_report.clone()), &connection.agent)?,
            Some(service) => Connection::send_message_to_self_endpoint(&A2AMessage::PresentationReject(problem_report.clone()), &service.into())?
        }

        trace!("ProverSM::_handle_reject_presentation_request <<<");
        Ok(problem_report)
    }

    fn _handle_presentation_proposal(connection: &CompletedConnection, preview: PresentationPreview, presentation_request: &PresentationRequest, thread: &Thread) -> VcxResult<PresentationProposal> {
        trace!("ProverSM::_handle_presentation_proposal >>> preview: {:?}, presentation_request: {:?}", secret!(preview), secret!(presentation_request));
        debug!("Prover: Preparing presentation proposal");

        let proposal = PresentationProposal::create()
            .set_presentation_preview(preview)
            .set_thread_id(&thread.thid.clone().unwrap_or(presentation_request.id.to_string()));

        match presentation_request.service.clone() {
            None => connection.data.send_message(&proposal.to_a2a_message(), &connection.agent)?,
            Some(service) => Connection::send_message_to_self_endpoint(&proposal.to_a2a_message(), &service.into())?
        }

        trace!("ProverSM::_handle_presentation_proposal <<<");
        Ok(proposal)
    }

    pub fn source_id(&self) -> String { self.source_id.clone() }

    pub fn state(&self) -> u32 {
        match self.state {
            ProverState::RequestReceived(_) => VcxStateType::VcxStateRequestReceived as u32,
            ProverState::PresentationPrepared(_) => VcxStateType::VcxStateRequestReceived as u32,
            ProverState::PresentationPreparationFailed(_) => VcxStateType::VcxStateRequestReceived as u32,
            ProverState::PresentationSent(_) => VcxStateType::VcxStateOfferSent as u32,
            ProverState::Finished(ref status) => {
                match status.status {
                    Status::Success => VcxStateType::VcxStateAccepted as u32,
                    Status::Rejected => VcxStateType::VcxStateRejected as u32,
                    _ => VcxStateType::VcxStateNone as u32,
                }
            }
            ProverState::ProposalPrepared(_) =>  VcxStateType::VcxStateInitialized as u32,
            ProverState::ProposalSent(_) => VcxStateType::VcxStateOfferSent as u32,
        }
    }

    pub fn has_transitions(&self) -> bool {
        match self.state {
            ProverState::RequestReceived(_) => false,
            ProverState::PresentationPrepared(_) => true,
            ProverState::PresentationPreparationFailed(_) => true,
            ProverState::PresentationSent(_) => true,
            ProverState::Finished(_) => false,
            ProverState::ProposalPrepared(_) => false,
            ProverState::ProposalSent(_) => true
        }
    }

    pub fn get_agent_info(&self) -> Option<&AgentInfo> {
        match self.state {
            ProverState::RequestReceived(_) => None,
            ProverState::PresentationPrepared(_) => None,
            ProverState::PresentationPreparationFailed(_) => None,
            ProverState::PresentationSent(ref state) => Some(&state.connection.agent),
            ProverState::Finished(_) => None,
            ProverState::ProposalPrepared(_) => None,
            ProverState::ProposalSent(ref state) => Some(&state.connection.agent)
        }
    }

    pub fn presentation_request(&self) -> VcxResult<&PresentationRequest> {
        match self.state {
            ProverState::RequestReceived(ref state) => Ok(&state.presentation_request),
            ProverState::PresentationPrepared(ref state) => Ok(&state.presentation_request),
            ProverState::PresentationPreparationFailed(ref state) => Ok(&state.presentation_request),
            ProverState::PresentationSent(ref state) => Ok(&state.presentation_request),
            ProverState::Finished(ref state) => Ok(&state.presentation_request),
            ProverState::ProposalPrepared(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                      format!("Prover object {} in state {} not ready to get Presentation message", self.source_id, self.state()))),
            ProverState::ProposalSent(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                      format!("Prover object {} in state {} not ready to get Presentation message", self.source_id, self.state()))),
        }
    }

    pub fn presentation(&self) -> VcxResult<&Presentation> {
        match self.state {
            ProverState::RequestReceived(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                      format!("Prover object {} in state {} not ready to get Presentation message", self.source_id, self.state()))),
            ProverState::PresentationPrepared(ref state) => Ok(&state.presentation),
            ProverState::PresentationPreparationFailed(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Presentation preparation failed")),
            ProverState::PresentationSent(ref state) => Ok(&state.presentation),
            ProverState::Finished(ref state) => Ok(&state.presentation),
            ProverState::ProposalPrepared(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                      format!("Prover object {} in state {} not ready to get Presentation message", self.source_id, self.state()))),
            ProverState::ProposalSent(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                      format!("Prover object {} in state {} not ready to get Presentation message", self.source_id, self.state()))),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use utils::devsetup::SetupAriesMocks;
    use v3::handlers::connection::tests::mock_connection;
    use v3::test::source_id;
    use v3::messages::proof_presentation::test::{_ack, _problem_report};
    use v3::messages::proof_presentation::presentation_request::tests::{_presentation_request, _presentation_request_with_service};
    use v3::messages::proof_presentation::presentation::tests::_presentation;
    use v3::messages::proof_presentation::presentation_proposal::tests::{_presentation_proposal, _presentation_preview};

    pub fn _prover_sm() -> ProverSM {
        ProverSM::new(_presentation_request(), source_id())
    }

    impl ProverSM {
        fn to_presentation_prepared_state(mut self) -> ProverSM {
            self = self.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            self
        }

        fn to_presentation_sent_state(mut self) -> ProverSM {
            self = self.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            self = self.step(ProverMessages::SendPresentation(mock_connection())).unwrap();
            self
        }

        fn to_finished_state(mut self) -> ProverSM {
            self = self.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            self = self.step(ProverMessages::SendPresentation(mock_connection())).unwrap();
            self = self.step(ProverMessages::PresentationAckReceived(_ack())).unwrap();
            self
        }
    }

    fn _credentials() -> String {
        json!({
            "attrs":{
            "attribute_0":{
                "credential":{
                    "cred_info":{
                        "attrs":{"name": "alice"},
                        "cred_def_id": "V4SGRU86Z58d6TV7PBUe6f:3:CL:419:tag",
                        "referent": "a1991de8-8317-43fd-98b3-63bac40b9e8b",
                        "schema_id": "V4SGRU86Z58d6TV7PBUe6f:2:QcimrRShWQniqlHUtIDddYP0n:1.0"
                        }
                    }
                }
            }
        }).to_string()
    }

    fn _self_attested() -> String {
        json!({}).to_string()
    }

    mod new {
        use super::*;

        #[test]
        fn test_prover_new() {
            let _setup = SetupAriesMocks::init();

            let prover_sm = _prover_sm();

            assert_match!(ProverState::RequestReceived(_), prover_sm.state);
            assert_eq!(source_id(), prover_sm.source_id());
        }
    }

    mod step {
        use super::*;

        #[test]
        fn test_prover_init() {
            let _setup = SetupAriesMocks::init();

            let prover_sm = _prover_sm();
            assert_match!(ProverState::RequestReceived(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_prepare_presentation_message_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();

            assert_match!(ProverState::PresentationPrepared(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_prepare_presentation_message_from_initiated_state_for_invalid_credentials() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation(("invalid".to_string(), _self_attested()))).unwrap();

            assert_match!(ProverState::PresentationPreparationFailed(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_reject_presentation_request_message_from_initiated_state() -> Result<(), String> {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::RejectPresentationRequest((mock_connection(), String::from("reject request")))).unwrap();

            assert_match!(ProverState::Finished(_), prover_sm.state);
            match prover_sm.state {
                ProverState::Finished(state) => {
                    assert_eq!(3, state.status.code());
                    Ok(())
                }
                other => Err(format!("State expected to be Finished, but: {:?}", other))
            }
        }

        #[test]
        fn test_prover_handle_propose_presentation_message_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::ProposePresentation((mock_connection(), _presentation_preview()))).unwrap();

            assert_match!(ProverState::Finished(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_other_messages_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();

            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();
            assert_match!(ProverState::RequestReceived(_), prover_sm.state);

            prover_sm = prover_sm.step(ProverMessages::PresentationAckReceived(_ack())).unwrap();
            assert_match!(ProverState::RequestReceived(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_send_presentation_message_from_presentation_prepared_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();

            assert_match!(ProverState::PresentationSent(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_send_presentation_message_from_presentation_prepared_state_for_presentation_request_contains_service_decorator() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = ProverSM::new(_presentation_request_with_service(), source_id());
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();

            assert_match!(ProverState::Finished(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_other_messages_from_presentation_prepared_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm().to_presentation_prepared_state();

            prover_sm = prover_sm.step(ProverMessages::PresentationRejectReceived(_problem_report())).unwrap();
            assert_match!(ProverState::PresentationPrepared(_), prover_sm.state);

            prover_sm = prover_sm.step(ProverMessages::PresentationAckReceived(_ack())).unwrap();
            assert_match!(ProverState::PresentationPrepared(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_reject_presentation_request_message_from_presentation_prepared_state() -> Result<(), String> {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm().to_presentation_prepared_state();
            prover_sm = prover_sm.step(ProverMessages::RejectPresentationRequest((mock_connection(), String::from("reject request")))).unwrap();

            assert_match!(ProverState::Finished(_), prover_sm.state);
            match prover_sm.state {
                ProverState::Finished(state) => {
                    assert_eq!(3, state.status.code());
                    Ok(())
                }
                other => Err(format!("State expected to be Finished, but: {:?}", other))
            }
        }

        #[test]
        fn test_prover_handle_propose_presentation_message_from_presentation_prepared_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm().to_presentation_prepared_state();
            prover_sm = prover_sm.step(ProverMessages::ProposePresentation((mock_connection(), _presentation_preview()))).unwrap();

            assert_match!(ProverState::Finished(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_send_presentation_message_from_presentation_preparation_failed_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation(("invalid".to_string(), _self_attested()))).unwrap();
            assert_match!(ProverState::PresentationPreparationFailed(_), prover_sm.state);

            prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap_err();
        }

        #[test]
        fn test_prover_handle_other_messages_from_presentation_preparation_failed_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation(("invalid".to_string(), _self_attested()))).unwrap();

            prover_sm = prover_sm.step(ProverMessages::PresentationRejectReceived(_problem_report())).unwrap();
            assert_match!(ProverState::PresentationPreparationFailed(_), prover_sm.state);

            prover_sm = prover_sm.step(ProverMessages::PresentationAckReceived(_ack())).unwrap();
            assert_match!(ProverState::PresentationPreparationFailed(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_ack_message_from_presentation_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();
            prover_sm = prover_sm.step(ProverMessages::PresentationAckReceived(_ack())).unwrap();

            assert_match!(ProverState::Finished(_), prover_sm.state);
            assert_eq!(VcxStateType::VcxStateAccepted as u32, prover_sm.state());
        }

        #[test]
        fn test_prover_handle_reject_presentation_request_message_from_presentation_sent_state() {
            let _setup = SetupAriesMocks::init();

            let prover_sm = _prover_sm().to_presentation_sent_state();
            let err = prover_sm.step(ProverMessages::RejectPresentationRequest((mock_connection(), String::from("reject")))).unwrap_err();
            assert_eq!(VcxErrorKind::InvalidState, err.kind());
        }

        #[test]
        fn test_prover_handle_presentation_reject_message_from_presentation_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();
            prover_sm = prover_sm.step(ProverMessages::PresentationRejectReceived(_problem_report())).unwrap();

            assert_match!(ProverState::Finished(_), prover_sm.state);
            assert_eq!(VcxStateType::VcxStateNone as u32, prover_sm.state());
        }

        #[test]
        fn test_prover_handle_other_messages_from_presentation_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();

            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            assert_match!(ProverState::PresentationSent(_), prover_sm.state);

            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();
            assert_match!(ProverState::PresentationSent(_), prover_sm.state);
        }

        #[test]
        fn test_prover_handle_messages_from_finished_state() {
            let _setup = SetupAriesMocks::init();

            let mut prover_sm = _prover_sm();
            prover_sm = prover_sm.step(ProverMessages::PreparePresentation((_credentials(), _self_attested()))).unwrap();
            prover_sm = prover_sm.step(ProverMessages::SendPresentation(mock_connection())).unwrap();
            prover_sm = prover_sm.step(ProverMessages::PresentationAckReceived(_ack())).unwrap();

            prover_sm = prover_sm.step(ProverMessages::PresentationAckReceived(_ack())).unwrap();
            assert_match!(ProverState::Finished(_), prover_sm.state);

            prover_sm = prover_sm.step(ProverMessages::PresentationRejectReceived(_problem_report())).unwrap();
            assert_match!(ProverState::Finished(_), prover_sm.state);
        }
    }

    mod find_message_to_handle {
        use super::*;

        #[test]
        fn test_prover_find_message_to_handle_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let prover = _prover_sm();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_4".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_5".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(prover.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_prover_find_message_to_handle_from_presentation_prepared_state() {
            let _setup = SetupAriesMocks::init();

            let prover = _prover_sm().to_presentation_prepared_state();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_4".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_5".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(prover.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_prover_find_message_to_handle_from_presentation_sent_state() {
            let _setup = SetupAriesMocks::init();

            let prover = _prover_sm().to_presentation_sent_state();

            // Ack
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationAck(_ack())
                );

                let (uid, message) = prover.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::PresentationAck(_), message);
            }

            // Problem Report
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_3".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                let (uid, message) = prover.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::CommonProblemReport(_), message);
            }

            // Presentation Reject
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_3".to_string() => A2AMessage::PresentationReject(_problem_report())
                );

                let (uid, message) = prover.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::CommonProblemReport(_), message);
            }

            // No messages for different Thread ID
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal().set_thread_id("")),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation().set_thread_id("")),
                    "key_3".to_string() => A2AMessage::PresentationAck(_ack().set_thread_id("")),
                    "key_4".to_string() => A2AMessage::CommonProblemReport(_problem_report().set_thread_id(""))
                );

                assert!(prover.find_message_to_handle(messages).is_none());
            }

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::PresentationRequest(_presentation_request())
                );

                assert!(prover.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_prover_find_message_to_handle_from_finished_state() {
            let _setup = SetupAriesMocks::init();

            let prover = _prover_sm().to_finished_state();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_4".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_5".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(prover.find_message_to_handle(messages).is_none());
            }
        }
    }

    mod get_state {
        use super::*;

        #[test]
        fn test_get_state() {
            let _setup = SetupAriesMocks::init();

            assert_eq!(VcxStateType::VcxStateRequestReceived as u32, _prover_sm().state());
            assert_eq!(VcxStateType::VcxStateRequestReceived as u32, _prover_sm().to_presentation_prepared_state().state());
            assert_eq!(VcxStateType::VcxStateOfferSent as u32, _prover_sm().to_presentation_sent_state().state());
            assert_eq!(VcxStateType::VcxStateAccepted as u32, _prover_sm().to_finished_state().state());
        }
    }
}