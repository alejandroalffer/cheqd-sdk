use api::VcxStateType;

use v3::handlers::connection::messages::DidExchangeMessages;
use v3::messages::a2a::A2AMessage;
use v3::handlers::connection::agent::AgentInfo;
use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::{Response, SignedResponse};
use v3::messages::connection::problem_report::{ProblemReport, ProblemCode};
use v3::messages::trust_ping::ping::Ping;
use v3::messages::trust_ping::ping_response::PingResponse;
use v3::messages::ack::Ack;
use v3::messages::connection::did_doc::DidDoc;
use v3::messages::discovery::query::Query;
use v3::messages::discovery::disclose::{Disclose, ProtocolDescriptor};
use v3::messages::a2a::MessageId;
use v3::messages::a2a::protocol_registry::ProtocolRegistry;

use std::collections::HashMap;

use error::prelude::*;
use v3::utils::pending_message::PendingMessage;
use messages::thread::Thread;
use v3::handlers::connection::types::CompletedConnectionInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidExchangeSM {
    source_id: String,
    agent_info: AgentInfo,
    state: ActorDidExchangeState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorDidExchangeState {
    Inviter(DidExchangeState),
    Invitee(DidExchangeState),
}

/// Transitions of Inviter Connection state
/// Null -> Invited
/// Invited -> Responded, Null
/// Responded -> Complete, Null
/// Completed
///
/// Transitions of Invitee Connection state
/// Null -> Invited
/// Invited -> Requested, Null
/// Requested -> Completed, Null
/// Completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidExchangeState {
    Null(NullState),
    Invited(InvitedState),
    Requested(RequestedState),
    Responded(RespondedState),
    Completed(CompleteState),
}

impl DidExchangeState {
    pub fn code(&self) -> u32 {
        match self {
            DidExchangeState::Null(_) => VcxStateType::VcxStateInitialized as u32,
            DidExchangeState::Invited(_) => VcxStateType::VcxStateOfferSent as u32,
            DidExchangeState::Requested(_) => VcxStateType::VcxStateRequestReceived as u32,
            DidExchangeState::Responded(_) => VcxStateType::VcxStateRequestReceived as u32,
            DidExchangeState::Completed(_) => VcxStateType::VcxStateAccepted as u32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NullState {
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitedState {
    invitation: Invitation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedState {
    request: Request,
    did_doc: DidDoc,
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    response: SignedResponse,
    did_doc: DidDoc,
    prev_agent_info: AgentInfo,
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteState {
    did_doc: DidDoc,
    protocols: Option<Vec<ProtocolDescriptor>>,
    thread: Thread,
}

impl From<(NullState, Invitation)> for InvitedState {
    fn from((_state, invitation): (NullState, Invitation)) -> InvitedState {
        trace!("DidExchangeStateSM: transit state from NullState to InvitedState");
        InvitedState { invitation }
    }
}

impl From<(InvitedState, ProblemReport, Thread)> for NullState {
    fn from((_state, _error, thread): (InvitedState, ProblemReport, Thread)) -> NullState {
        trace!("DidExchangeStateSM: transit state from InvitedState to NullState");
        trace!("Thread: {:?}", thread);
        NullState { thread }
    }
}

impl From<(InvitedState, Request, Thread)> for RequestedState {
    fn from((state, request, thread): (InvitedState, Request, Thread)) -> RequestedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        trace!("Thread: {:?}", thread);
        RequestedState { request, did_doc: DidDoc::from(state.invitation), thread }
    }
}

impl From<(InvitedState, Request, SignedResponse, AgentInfo, Thread)> for RespondedState {
    fn from((_state, request, response, prev_agent_info, thread): (InvitedState, Request, SignedResponse, AgentInfo, Thread)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to RequestedState");
        trace!("Thread: {:?}", thread);
        RespondedState { response, did_doc: request.connection.did_doc, prev_agent_info, thread }
    }
}

impl From<(RespondedState, Ping, Thread)> for RespondedState {
    fn from((state, _ping, thread): (RespondedState, Ping, Thread)) -> RespondedState {
        trace!("DidExchangeStateSM: transit state from RespondedState to RespondedState");
        trace!("Thread: {:?}", thread);
        RespondedState { response: state.response, did_doc: state.did_doc, prev_agent_info: state.prev_agent_info, thread }
    }
}

impl From<(RequestedState, ProblemReport, Thread)> for NullState {
    fn from((_state, _error, thread): (RequestedState, ProblemReport, Thread)) -> NullState {
        trace!("DidExchangeStateSM: transit state from RequestedState to NullState");
        trace!("Thread: {:?}", thread);
        NullState { thread }
    }
}

impl From<(RequestedState, Response, Thread)> for CompleteState {
    fn from((_state, response, thread): (RequestedState, Response, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: response.connection.did_doc, protocols: None, thread }
    }
}

impl From<(RespondedState, ProblemReport, Thread)> for NullState {
    fn from((_state, _error, thread): (RespondedState, ProblemReport, Thread)) -> NullState {
        trace!("DidExchangeStateSM: transit state from RespondedState to NullState");
        trace!("Thread: {:?}", thread);
        NullState { thread }
    }
}

impl From<(RespondedState, Ack, Thread)> for CompleteState {
    fn from((state, _ack, thread): (RespondedState, Ack, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: state.did_doc, protocols: None, thread }
    }
}

impl From<(RespondedState, Ping, Thread)> for CompleteState {
    fn from((state, _ping, thread): (RespondedState, Ping, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: state.did_doc, protocols: None, thread }
    }
}

impl From<(RespondedState, PingResponse, Thread)> for CompleteState {
    fn from((state, _ping_response, thread): (RespondedState, PingResponse, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: state.did_doc, protocols: None, thread }
    }
}

impl From<(CompleteState, Vec<ProtocolDescriptor>)> for CompleteState {
    fn from((state, protocols): (CompleteState, Vec<ProtocolDescriptor>)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from CompleteState to CompleteState");
        CompleteState { did_doc: state.did_doc, protocols: Some(protocols), thread: state.thread }
    }
}

impl InvitedState {
    fn handle_connection_request(&self, request: &Request,
                                 agent_info: &AgentInfo,
                                 thread: &Thread) -> VcxResult<(SignedResponse, AgentInfo)> {
        trace!("InvitedState:handle_connection_request >>> request: {:?}, agent_info: {:?}", request, agent_info);

        request.connection.did_doc.validate()?;

        let prev_agent_info = agent_info.clone();

        // provision a new keys
        let new_agent_info: AgentInfo = agent_info.create_agent()?;

        let response = Response::create()
            .set_did(new_agent_info.pw_did.to_string())
            .set_service_endpoint(new_agent_info.agency_endpoint()?)
            .set_keys(new_agent_info.recipient_keys(), new_agent_info.routing_keys()?)
            .ask_for_ack();

        let signed_response = response.clone()
            .set_thread(thread.to_owned())
            .encode(&prev_agent_info.pw_vk)?;

        new_agent_info.send_message(&signed_response.to_a2a_message(), &request.connection.did_doc)?;

        Ok((signed_response, new_agent_info))
    }
}

impl RequestedState {
    fn handle_connection_response(&self, response: SignedResponse, agent_info: &AgentInfo) -> VcxResult<(Response, Thread)> {
        trace!("RequestedState:handle_connection_response >>> response: {:?}, agent_info: {:?}", response, agent_info);

        let remote_vk: String = self.did_doc.recipient_keys().get(0).cloned()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Cannot handle Response: Remote Verkey not found"))?;

        let response: Response = response.decode(&remote_vk)?;

        if !response.from_thread(&self.request.id.to_string()) {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson,
                                          format!("Received Connection Response from different thread: {:?}. Expected Thread: {:?}", response.thread, self.thread)));
        }

        let thread = self.thread.clone()
            .increment_sender_order()
            .update_received_order(&response.connection.did);

        let message = if response.please_ack.is_some() {
            Ack::create()
                .set_thread(thread.clone())
                .to_a2a_message()
        } else {
            Ping::create()
                .set_thread(thread.clone())
                .to_a2a_message()
        };

        agent_info.send_message(&message, &response.connection.did_doc)?;

        Ok((response, thread))
    }
}

impl RespondedState {
    fn handle_ack(&self, ack: &Ack) -> VcxResult<()> {
        self.thread.check_message_order(&self.did_doc.id, &ack.thread)?;
        Ok(())
    }

    fn handle_ping(&self, ping: &Ping, agent_info: &AgentInfo) -> VcxResult<()> {
        self.thread.check_message_order(&self.did_doc.id, &ping.thread.clone().unwrap_or_default())?;
        _handle_ping(ping, agent_info, &self.did_doc)
    }
}

impl CompleteState {
    fn handle_message(self, message: DidExchangeMessages, agent_info: &AgentInfo) -> VcxResult<DidExchangeState> {
        Ok(match message {
            DidExchangeMessages::SendPing(comment) => {
                self.handle_send_ping(comment, agent_info)?;
                DidExchangeState::Completed(self)
            }
            DidExchangeMessages::PingReceived(ping) => {
                self.handle_ping(&ping, agent_info)?;
                DidExchangeState::Completed(self)
            }
            DidExchangeMessages::PingResponseReceived(_) => {
                DidExchangeState::Completed(self)
            }
            DidExchangeMessages::DiscoverFeatures((query_, comment)) => {
                self.handle_discover_features(query_, comment, agent_info)?;
                DidExchangeState::Completed(self)
            }
            DidExchangeMessages::QueryReceived(query) => {
                self.handle_discovery_query(query, agent_info)?;
                DidExchangeState::Completed(self)
            }
            DidExchangeMessages::DiscloseReceived(disclose) => {
                DidExchangeState::Completed((self, disclose.protocols).into())
            }
            _ => {
                DidExchangeState::Completed(self)
            }
        })
    }

    fn handle_send_ping(&self, comment: Option<String>, agent_info: &AgentInfo) -> VcxResult<()> {
        let ping =
            Ping::create()
                .request_response()
                .set_comment(comment);

        agent_info.send_message(&ping.to_a2a_message(), &self.did_doc).ok();
        Ok(())
    }

    fn handle_ping(&self, ping: &Ping, agent_info: &AgentInfo) -> VcxResult<()> {
        _handle_ping(ping, agent_info, &self.did_doc)
    }

    fn handle_discover_features(&self, query: Option<String>, comment: Option<String>, agent_info: &AgentInfo) -> VcxResult<()> {
        let query_ =
            Query::create()
                .set_query(query)
                .set_comment(comment);

        agent_info.send_message(&query_.to_a2a_message(), &self.did_doc)
    }

    fn handle_discovery_query(&self, query: Query, agent_info: &AgentInfo) -> VcxResult<()> {
        let protocols = ProtocolRegistry::init().get_protocols_for_query(query.query.as_ref().map(String::as_str));

        let disclose = Disclose::create()
            .set_protocols(protocols)
            .set_thread_id(query.id.0.clone());

        agent_info.send_message(&disclose.to_a2a_message(), &self.did_doc)
    }
}

fn _handle_ping(ping: &Ping, agent_info: &AgentInfo, did_doc: &DidDoc) -> VcxResult<()> {
    if ping.response_requested {
        let ping_response = PingResponse::create().set_thread_id(
            &ping.thread.as_ref().and_then(|thread| thread.thid.clone()).unwrap_or(ping.id.0.clone()));
        agent_info.send_message(&ping_response.to_a2a_message(), did_doc)?;
    }
    Ok(())
}

impl DidExchangeSM {
    pub fn new(actor: Actor, source_id: &str) -> Self {
        match actor {
            Actor::Inviter => {
                DidExchangeSM {
                    source_id: source_id.to_string(),
                    state: ActorDidExchangeState::Inviter(
                        DidExchangeState::Null(
                            NullState { thread: Thread::new() }
                        )
                    ),
                    agent_info: AgentInfo::default(),
                }
            }
            Actor::Invitee => {
                DidExchangeSM {
                    source_id: source_id.to_string(),
                    state: ActorDidExchangeState::Invitee(
                        DidExchangeState::Null(
                            NullState { thread: Thread::new() }
                        )
                    ),
                    agent_info: AgentInfo::default(),
                }
            }
        }
    }

    pub fn from(source_id: String, agent_info: AgentInfo, state: ActorDidExchangeState) -> Self {
        DidExchangeSM {
            source_id,
            agent_info,
            state,
        }
    }

    pub fn agent_info(&self) -> &AgentInfo {
        &self.agent_info
    }

    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn state(&self) -> u32 {
        match self.state {
            ActorDidExchangeState::Inviter(ref state) | ActorDidExchangeState::Invitee(ref state) => state.code(),
        }
    }

    pub fn state_object<'a>(&'a self) -> &'a ActorDidExchangeState {
        &self.state
    }

    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, A2AMessage)> {
        trace!("DidExchangeSM::find_message_to_handle >>> messages: {:?}", messages);

        for (uid, message) in messages {
            match self.state {
                ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)) => {
                    match message {
                        request @ A2AMessage::ConnectionRequest(_) => {
                            debug!("Inviter received ConnectionRequest message");
                            return Some((uid, request));
                        }
                        problem_report @ A2AMessage::ConnectionProblemReport(_) => {
                            debug!("Inviter received ProblemReport message");
                            return Some((uid, problem_report));
                        }
                        message @ _ => {
                            debug!("Inviter received unexpected message: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Invitee(DidExchangeState::Requested(_)) => {
                    match message {
                        response @ A2AMessage::ConnectionResponse(_) => {
                            debug!("Invitee received ConnectionResponse message");
                            return Some((uid, response));
                        }
                        problem_report @ A2AMessage::ConnectionProblemReport(_) => {
                            debug!("Invitee received ProblemReport message");
                            return Some((uid, problem_report));
                        }
                        message @ _ => {
                            debug!("Invitee received unexpected message: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Inviter(DidExchangeState::Responded(_)) => {
                    match message {
                        ack @ A2AMessage::Ack(_) => {
                            debug!("Ack message received");
                            return Some((uid, ack));
                        }
                        ping @ A2AMessage::Ping(_) => {
                            debug!("Ping message received");
                            return Some((uid, ping));
                        }
                        ping @ A2AMessage::PingResponse(_) => {
                            debug!("PingResponse message received");
                            return Some((uid, ping));
                        }
                        problem_report @ A2AMessage::ConnectionProblemReport(_) => {
                            debug!("ProblemReport message received");
                            return Some((uid, problem_report));
                        }
                        message @ _ => {
                            debug!("Unexpected message received in Responded state: {:?}", message);
                        }
                    }
                }
                ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)) |
                ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)) => {
                    match message {
                        ping @ A2AMessage::Ping(_) => {
                            debug!("Ping message received");
                            return Some((uid, ping));
                        }
                        ping_response @ A2AMessage::PingResponse(_) => {
                            debug!("PingResponse message received");
                            return Some((uid, ping_response));
                        }
                        query @ A2AMessage::Query(_) => {
                            debug!("Query message received");
                            return Some((uid, query));
                        }
                        disclose @ A2AMessage::Disclose(_) => {
                            debug!("Disclose message received");
                            return Some((uid, disclose));
                        }
                        message @ _ => {
                            debug!("Unexpected message received in Completed state: {:?}", message);
                        }
                    }
                }
                _ => {
                    debug!("Unexpected message received: message: {:?}", message);
                }
            }
        }

        None
    }

    pub fn step(self, message: DidExchangeMessages) -> VcxResult<DidExchangeSM> {
        trace!("DidExchangeStateSM::step >>> message: {:?}", message);

        let DidExchangeSM { source_id, mut agent_info, state } = self;

        let state = match state {
            ActorDidExchangeState::Inviter(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            DidExchangeMessages::Connect() => {
                                agent_info = agent_info.create_agent()?;

                                let invite: Invitation = Invitation::create()
                                    .set_label(source_id.to_string())
                                    .set_service_endpoint(agent_info.agency_endpoint()?)
                                    .set_recipient_keys(agent_info.recipient_keys())
                                    .set_routing_keys(agent_info.routing_keys()?);

                                ActorDidExchangeState::Inviter(DidExchangeState::Invited((state, invite).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Null(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            DidExchangeMessages::ExchangeRequestReceived(request) => {
                                let thread = Thread::new()
                                    .set_thid(request.id.to_string())
                                    .update_received_order(&request.connection.did_doc.id);

                                match state.handle_connection_request(&request, &agent_info, &thread) {
                                    Ok((response, new_agent_info)) => {
                                        let prev_agent_info = agent_info.clone();
                                        agent_info = new_agent_info;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Responded((state, request, response, prev_agent_info, thread).into()))
                                    }
                                    Err(err) => {
                                        let problem_report = ProblemReport::create()
                                            .set_problem_code(ProblemCode::RequestProcessingError)
                                            .set_explain(err.to_string())
                                            .set_thread(thread.clone());

                                        agent_info.send_message(&problem_report.to_a2a_message(), &request.connection.did_doc).ok(); // IS is possible?
                                        ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                let thread = Thread::new()
                                    .set_thid(problem_report.id.to_string());

                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report, thread).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Invited(state))
                            }
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        ActorDidExchangeState::Inviter(DidExchangeState::Requested(state))
                    }
                    DidExchangeState::Responded(state) => {
                        match message {
                            DidExchangeMessages::AckReceived(ack) => {
                                match state.handle_ack(&ack) {
                                    Ok(()) => {
                                        let thread = state.thread.clone()
                                            .update_received_order(&state.did_doc.id);

                                        ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ack, thread).into()))
                                    }
                                    Err(err) => {
                                        let thread = state.thread.clone()
                                            .increment_sender_order()
                                            .update_received_order(&state.did_doc.id);

                                        let problem_report = ProblemReport::create()
                                            .set_problem_code(ProblemCode::RequestProcessingError)
                                            .set_explain(err.to_string())
                                            .set_thread(thread.clone());

                                        agent_info.send_message(&problem_report.to_a2a_message(), &state.did_doc).ok();
                                        ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::PingReceived(ping) => {
                                match state.handle_ping(&ping, &agent_info) {
                                    Ok(()) => {
                                        let thread = state.thread.clone()
                                            .update_received_order(&state.did_doc.id);

                                        ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ping, thread).into()))
                                    }
                                    Err(err) => {
                                        let thread = state.thread.clone()
                                            .increment_sender_order()
                                            .update_received_order(&state.did_doc.id);

                                        let problem_report = ProblemReport::create()
                                            .set_problem_code(ProblemCode::RequestProcessingError)
                                            .set_explain(err.to_string())
                                            .set_thread(thread.clone());

                                        agent_info.send_message(&problem_report.to_a2a_message(), &state.did_doc).ok();
                                        ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                let thread = state.thread.clone()
                                    .update_received_order(&state.did_doc.id);

                                ActorDidExchangeState::Inviter(DidExchangeState::Null((state, problem_report, thread).into()))
                            }
                            DidExchangeMessages::SendPing(comment) => {
                                let thread = state.thread.clone()
                                    .increment_sender_order()
                                    .update_received_order(&state.did_doc.id);

                                let ping =
                                    Ping::create()
                                        .request_response()
                                        .set_comment(comment)
                                        .set_thread(thread.clone());

                                agent_info.send_message(&ping.to_a2a_message(), &state.did_doc).ok();
                                ActorDidExchangeState::Inviter(DidExchangeState::Responded((state, ping, thread).into()))
                            }
                            DidExchangeMessages::PingResponseReceived(ping_response) => {
                                let thread = state.thread.clone()
                                    .update_received_order(&state.did_doc.id);

                                ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ping_response, thread).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Responded(state))
                            }
                        }
                    }
                    DidExchangeState::Completed(state) => {
                        ActorDidExchangeState::Inviter(state.handle_message(message, &agent_info)?)
                    }
                }
            }
            ActorDidExchangeState::Invitee(state) => {
                match state {
                    DidExchangeState::Null(state) => {
                        match message {
                            DidExchangeMessages::InvitationReceived(invitation) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited((state, invitation).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Null(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            DidExchangeMessages::Connect() => {
                                agent_info = agent_info.create_agent()?;

                                let request = Request::create()
                                    .set_label(source_id.to_string())
                                    .set_did(agent_info.pw_did.to_string())
                                    .set_service_endpoint(agent_info.agency_endpoint()?)
                                    .set_keys(agent_info.recipient_keys(), agent_info.routing_keys()?);

                                let thread = Thread::new()
                                    .set_thid(request.id.to_string());

                                agent_info.send_message(&request.to_a2a_message(), &DidDoc::from(state.invitation.clone()))?;
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested((state, request, thread).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited(state))
                            }
                        }
                    }
                    DidExchangeState::Requested(state) => {
                        match message {
                            DidExchangeMessages::ExchangeResponseReceived(response) => {
                                match state.handle_connection_response(response, &agent_info) {
                                    Ok((response, thread)) => {
                                        ActorDidExchangeState::Invitee(DidExchangeState::Completed((state, response, thread).into()))
                                    }
                                    Err(err) => {
                                        let thread = state.thread.clone()
                                            .increment_sender_order()
                                            .update_received_order(&state.did_doc.id);

                                        let problem_report = ProblemReport::create()
                                            .set_problem_code(ProblemCode::ResponseProcessingError)
                                            .set_explain(err.to_string())
                                            .set_thread(thread.clone());

                                        agent_info.send_message(&problem_report.to_a2a_message(), &state.did_doc).ok();
                                        ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                let thread = state.thread.clone()
                                    .update_received_order(&state.did_doc.id);

                                ActorDidExchangeState::Invitee(DidExchangeState::Null((state, problem_report, thread).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested(state))
                            }
                        }
                    }
                    DidExchangeState::Responded(state) => {
                        ActorDidExchangeState::Invitee(DidExchangeState::Responded(state))
                    }
                    DidExchangeState::Completed(state) => {
                        ActorDidExchangeState::Invitee(state.handle_message(message, &agent_info)?)
                    }
                }
            }
        };

        Ok(DidExchangeSM { source_id, agent_info, state })
    }

    pub fn did_doc(&self) -> Option<DidDoc> {
        match self.state {
            ActorDidExchangeState::Inviter(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(ref state) => Some(DidDoc::from(state.invitation.clone())),
                    DidExchangeState::Requested(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Responded(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Completed(ref state) => Some(state.did_doc.clone()),
                },
            ActorDidExchangeState::Invitee(ref state) =>
                match state {
                    DidExchangeState::Null(_) => None,
                    DidExchangeState::Invited(ref state) => Some(DidDoc::from(state.invitation.clone())),
                    DidExchangeState::Requested(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Responded(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Completed(ref state) => Some(state.did_doc.clone()),
                }
        }
    }

    pub fn completed_connection_state_info(&self) -> VcxResult<CompletedConnectionInfo> {
        match self.state {
            ActorDidExchangeState::Inviter(ref state) =>
                match state {
                    DidExchangeState::Completed(ref state) => {
                        Ok(CompletedConnectionInfo {
                            agent: self.agent_info.clone(),
                            remote_did_doc: state.did_doc.clone(),
                        })
                    }
                    _ => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection is not completed yet."))
                },
            ActorDidExchangeState::Invitee(ref state) =>
                match state {
                    DidExchangeState::Completed(ref state) => {
                        Ok(CompletedConnectionInfo {
                            agent: self.agent_info.clone(),
                            remote_did_doc: state.did_doc.clone(),
                        })
                    }
                    _ => Err(VcxError::from_msg(VcxErrorKind::NotReady, "Connection is not completed yet."))
                }
        }
    }

    pub fn get_invitation(&self) -> Option<&Invitation> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Invited(ref state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Invited(ref state)) => Some(&state.invitation),
            _ => None
        }
    }

    pub fn get_protocols(&self) -> Vec<ProtocolDescriptor> {
        ProtocolRegistry::init().protocols()
    }

    pub fn get_remote_protocols(&self) -> Option<Vec<ProtocolDescriptor>> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Completed(ref state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Completed(ref state)) => state.protocols.clone(),
            _ => None
        }
    }

    pub fn remote_did(&self) -> VcxResult<String> {
        self.did_doc()
            .map(|did_doc: DidDoc| did_doc.id.clone())
            .ok_or(VcxError::from_msg(VcxErrorKind::NotReady, "Remote Connection DID is not set"))
    }

    pub fn remote_vk(&self) -> VcxResult<String> {
        self.did_doc()
            .and_then(|did_doc| did_doc.recipient_keys().get(0).cloned())
            .ok_or(VcxError::from_msg(VcxErrorKind::NotReady, "Remote Connection Verkey is not set"))
    }

    pub fn prev_agent_info(&self) -> Option<&AgentInfo> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Responded(ref state)) => Some(&state.prev_agent_info),
            _ => None
        }
    }

    pub fn actor(&self) -> Actor {
        match self.state {
            ActorDidExchangeState::Inviter(_) => Actor::Inviter,
            ActorDidExchangeState::Invitee(_) => Actor::Invitee
        }
    }

    pub fn add_pending_messages(&self, messages: HashMap<MessageId, String>) -> VcxResult<()> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Completed(ref _state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Completed(ref _state)) => {
                for (key, value) in messages {
                    PendingMessage::add(&key.0, &value).ok();
                }
            }
            _ => {}
        };
        Ok(())
    }

    pub fn remove_pending_message(&self, id: MessageId) -> VcxResult<()> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Completed(ref _state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Completed(ref _state)) => {
                if let Some(uid_) = PendingMessage::get(&id.0) {
                    self.agent_info.update_message_status(uid_.to_string()).ok();
                }
            }
            _ => {}
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Actor {
    Inviter,
    Invitee,
}

#[cfg(test)]
pub mod test {
    use super::*;

    use utils::devsetup::SetupAriesMocks;
    use v3::test::source_id;
    use v3::test::setup::AgencyModeSetup;
    use v3::messages::connection::invite::tests::_invitation;
    use v3::messages::connection::request::tests::_request;
    use v3::messages::connection::response::tests::_signed_response;
    use v3::messages::connection::problem_report::tests::_problem_report;
    use v3::messages::trust_ping::ping::tests::_ping;
    use v3::messages::trust_ping::ping_response::tests::_ping_response;
    use v3::messages::ack::tests::_ack as t_ack;
    use v3::messages::discovery::query::tests::_query;
    use v3::messages::discovery::disclose::tests::_disclose;

    pub fn _ack() -> Ack {
        let mut ack = t_ack();
        ack.thread.sender_order = 1;
        ack
    }

    pub mod inviter {
        use super::*;

        pub fn inviter_sm() -> DidExchangeSM {
            DidExchangeSM::new(Actor::Inviter, &source_id())
        }

        impl DidExchangeSM {
            fn to_inviter_invited_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self
            }

            fn to_inviter_responded_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self = self.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                self
            }

            fn to_inviter_completed_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self = self.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                self = self.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                self
            }
        }

        mod new {
            use super::*;

            #[test]
            fn test_inviter_new() {
                let _setup = SetupAriesMocks::init();

                let inviter_sm = inviter_sm();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), inviter_sm.state);
                assert_eq!(source_id(), inviter_sm.source_id());
            }
        }

        mod step {
            use super::*;
            use v3::messages::connection::response::tests::_thread;

            #[test]
            fn test_did_exchange_init() {
                let _setup = AgencyModeSetup::init();

                let did_exchange_sm = inviter_sm();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_connect_message_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_exchange_request_message_from_invited_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_invited_state();
                let request = _request();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(request.clone())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Responded(state)) => {
                        assert_eq!(request.id.to_string(), state.thread.thid.unwrap());
                        assert_eq!(0, state.thread.sender_order);
                        assert_eq!(0, state.thread.received_orders.get(&request.connection.did).cloned().unwrap());
                        Ok(())
                    }
                    other => Err(format!("State expected to be Responded, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_invalid_exchange_request_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_invited_state();

                let mut request = _request();
                request.connection.did_doc = DidDoc::default();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(request)).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_invited_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_invited_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_ack_message_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_responded_state();
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_ping_message_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_responded_state();

                let mut ping = _ping();
                ping.thread = Some(_thread().set_sender_order(1));

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(ping)).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_responded_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_responded_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Responded(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_messages_from_completed_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_completed_state();

                // Send Ping
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::SendPing(None)).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Ping
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(_ping())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Ping Response
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingResponseReceived(_ping_response())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Discovery Features
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::DiscoverFeatures((None, None))).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Query
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::QueryReceived(_query())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Disclose
                assert!(did_exchange_sm.get_remote_protocols().is_none());

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::DiscloseReceived(_disclose())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                assert!(did_exchange_sm.get_remote_protocols().is_some());

                // ignore
                // Ack
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Problem Report
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }
        }

        mod find_message_to_handle {
            use super::*;

            #[test]
            fn test_find_message_to_handle_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm();

                // No messages
                {
                    let messages = map!(
                    "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                    "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                    "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                    "key_4".to_string() => A2AMessage::Ping(_ping()),
                    "key_5".to_string() => A2AMessage::Ack(_ack())
                );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm().to_inviter_invited_state();

                // Connection Request
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_2", uid);
                    assert_match!(A2AMessage::ConnectionRequest(_), message);
                }

                // Connection Problem Report
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::ConnectionProblemReport(_), message);
                }

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_responded_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm().to_inviter_responded_state();

                // Ping
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_1", uid);
                    assert_match!(A2AMessage::Ping(_), message);
                }

                // Ack
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::Ack(_ack()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_2", uid);
                    assert_match!(A2AMessage::Ack(_), message);
                }

                // Connection Problem Report
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionProblemReport(_problem_report())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_2", uid);
                    assert_match!(A2AMessage::ConnectionProblemReport(_), message);
                }

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_completed_state() {
                let _setup = AgencyModeSetup::init();

                let connection = inviter_sm().to_inviter_completed_state();

                // Ping
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::Ping(_ping()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_4", uid);
                    assert_match!(A2AMessage::Ping(_), message);
                }

                // Ping Response
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::PingResponse(_ping_response()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_4", uid);
                    assert_match!(A2AMessage::PingResponse(_), message);
                }

                // Query
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::Query(_query())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::Query(_), message);
                }

                // Disclose
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::Disclose(_disclose())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::Disclose(_), message);
                }
            }
        }

        mod get_state {
            use super::*;

            #[test]
            fn test_get_state() {
                let _setup = SetupAriesMocks::init();

                assert_eq!(VcxStateType::VcxStateInitialized as u32, inviter_sm().state());
                assert_eq!(VcxStateType::VcxStateOfferSent as u32, inviter_sm().to_inviter_invited_state().state());
                assert_eq!(VcxStateType::VcxStateRequestReceived as u32, inviter_sm().to_inviter_responded_state().state());
                assert_eq!(VcxStateType::VcxStateAccepted as u32, inviter_sm().to_inviter_completed_state().state());
            }
        }
    }

    pub mod invitee {
        use super::*;

        use v3::messages::connection::did_doc::tests::_service_endpoint;

        pub fn invitee_sm() -> DidExchangeSM {
            DidExchangeSM::new(Actor::Invitee, &source_id())
        }

        impl DidExchangeSM {
            pub fn to_invitee_invited_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                self
            }

            pub fn to_invitee_requested_state(mut self) -> DidExchangeSM {
                self = self.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self
            }

            pub fn to_invitee_completed_state(mut self) -> DidExchangeSM {
                let key = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL".to_string();
                let invitation = Invitation::default().set_recipient_keys(vec![key.clone()]);

                self = self.step(DidExchangeMessages::InvitationReceived(invitation)).unwrap();
                self = self.step(DidExchangeMessages::Connect()).unwrap();
                self = self.step(DidExchangeMessages::ExchangeResponseReceived(_response(&key))).unwrap();
                self = self.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                self
            }
        }

        fn _response(key: &str) -> SignedResponse {
            Response::default()
                .set_service_endpoint(_service_endpoint())
                .set_keys(vec![key.to_string()], vec![])
                .set_thread_id(&_request().id.0)
                .encode(&key).unwrap()
        }

        mod new {
            use super::*;

            #[test]
            fn test_invitee_new() {
                let _setup = SetupAriesMocks::init();

                let invitee_sm = invitee_sm();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), invitee_sm.state);
                assert_eq!(source_id(), invitee_sm.source_id());
            }
        }

        mod step {
            use super::*;

            #[test]
            fn test_did_exchange_init() {
                let _setup = AgencyModeSetup::init();

                let did_exchange_sm = invitee_sm();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_invite_message_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_message_from_null_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_connect_message_from_invited_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_invited_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Invitee(DidExchangeState::Requested(state)) => {
                        assert_eq!(0, state.thread.sender_order);
                        Ok(())
                    }
                    other => Err(format!("State expected to be Responded, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_invited_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_invited_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Invited(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeRequestReceived(_request())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Invited(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_response_message_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let key = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";

                let mut did_exchange_sm = invitee_sm().to_invitee_requested_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeResponseReceived(_response(key))).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_invalid_response_message_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_requested_state();

                let mut signed_response = _signed_response();
                signed_response.connection_sig.signature = String::from("other");

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ExchangeResponseReceived(signed_response)).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_requested_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Null(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_requested_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Requested(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(_ping())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Requested(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_messages_from_completed_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_completed_state();

                // Send Ping
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::SendPing(None)).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Ping
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingReceived(_ping())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Ping Response
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::PingResponseReceived(_ping_response())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Discovery Features
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::DiscoverFeatures((None, None))).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Query
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::QueryReceived(_query())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Disclose
                assert!(did_exchange_sm.get_remote_protocols().is_none());

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::DiscloseReceived(_disclose())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                assert!(did_exchange_sm.get_remote_protocols().is_some());

                // ignore
                // Ack
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);

                // Problem Report
                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Completed(_)), did_exchange_sm.state);
            }
        }

        mod find_message_to_handle {
            use super::*;

            #[test]
            fn test_find_message_to_handle_from_invited_state() {
                let _setup = AgencyModeSetup::init();

                let connection = invitee_sm().to_invitee_invited_state();

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::Ping(_ping()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_requested_state() {
                let _setup = AgencyModeSetup::init();

                let connection = invitee_sm().to_invitee_requested_state();

                // Connection Response
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_3".to_string() => A2AMessage::ConnectionResponse(_signed_response())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::ConnectionResponse(_), message);
                }

                // Connection Problem Report
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::ConnectionProblemReport(_), message);
                }

                // No messages
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::Ping(_ping()),
                        "key_2".to_string() => A2AMessage::Ack(_ack())
                    );

                    assert!(connection.find_message_to_handle(messages).is_none());
                }
            }

            #[test]
            fn test_find_message_to_handle_from_completed_state() {
                let _setup = AgencyModeSetup::init();

                let connection = invitee_sm().to_invitee_completed_state();

                // Ping
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::Ping(_ping()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_4", uid);
                    assert_match!(A2AMessage::Ping(_), message);
                }

                // Ping Response
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::ConnectionProblemReport(_problem_report()),
                        "key_4".to_string() => A2AMessage::PingResponse(_ping_response()),
                        "key_5".to_string() => A2AMessage::Ack(_ack())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_4", uid);
                    assert_match!(A2AMessage::PingResponse(_), message);
                }

                // Query
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::Query(_query())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::Query(_), message);
                }

                // Disclose
                {
                    let messages = map!(
                        "key_1".to_string() => A2AMessage::ConnectionRequest(_request()),
                        "key_2".to_string() => A2AMessage::ConnectionResponse(_signed_response()),
                        "key_3".to_string() => A2AMessage::Disclose(_disclose())
                    );

                    let (uid, message) = connection.find_message_to_handle(messages).unwrap();
                    assert_eq!("key_3", uid);
                    assert_match!(A2AMessage::Disclose(_), message);
                }
            }
        }

        mod get_state {
            use super::*;

            #[test]
            fn test_get_state() {
                let _setup = SetupAriesMocks::init();

                assert_eq!(VcxStateType::VcxStateInitialized as u32, invitee_sm().state());
                assert_eq!(VcxStateType::VcxStateOfferSent as u32, invitee_sm().to_invitee_invited_state().state());
                assert_eq!(VcxStateType::VcxStateRequestReceived as u32, invitee_sm().to_invitee_requested_state().state());
            }
        }
    }
}

