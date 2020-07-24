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
use v3::messages::connection::did_doc::{DidDoc, Service, SERVICE_ID};
use v3::messages::discovery::query::Query;
use v3::messages::discovery::disclose::{Disclose, ProtocolDescriptor};
use v3::messages::a2a::protocol_registry::ProtocolRegistry;
use v3::messages::outofband::invitation::Invitation as OutofbandInvitation;
use v3::messages::outofband::handshake_reuse::HandshakeReuse;
use v3::messages::outofband::handshake_reuse_accepted::HandshakeReuseAccepted;
use v3::handlers::connection::types::{CompletedConnection, OutofbandMeta, Invitations};

use std::collections::HashMap;

use error::prelude::*;
use messages::thread::Thread;
use settings;

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
/// Initialized -> Invited
/// Invited -> Responded, Failed
/// Responded -> Complete, Failed
/// Completed
/// Failed
///
/// Transitions of Invitee Connection state
/// Initialized -> Invited
/// Invited -> Requested, Failed
/// Requested -> Completed, Failed
/// Completed
/// Failed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidExchangeState {
    Initialized(InitializedState),
    Invited(InvitedState),
    Requested(RequestedState),
    Responded(RespondedState),
    Completed(CompleteState),
    Failed(FailedState),
}

impl DidExchangeState {
    pub fn code(&self) -> u32 {
        match self {
            DidExchangeState::Initialized(_) => VcxStateType::VcxStateInitialized as u32,
            DidExchangeState::Invited(_) => VcxStateType::VcxStateOfferSent as u32,
            DidExchangeState::Requested(_) => VcxStateType::VcxStateRequestReceived as u32,
            DidExchangeState::Responded(_) => VcxStateType::VcxStateRequestReceived as u32,
            DidExchangeState::Completed(_) => VcxStateType::VcxStateAccepted as u32,
            DidExchangeState::Failed(_) => VcxStateType::VcxStateNone as u32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializedState {
    #[serde(skip_serializing_if = "Option::is_none")]
    outofband_meta: Option<OutofbandMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitedState {
    invitation: Invitations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedState {
    request: Request,
    did_doc: DidDoc,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondedState {
    response: SignedResponse,
    did_doc: DidDoc,
    prev_agent_info: AgentInfo,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteState {
    pub did_doc: DidDoc,
    pub protocols: Option<Vec<ProtocolDescriptor>>,
    #[serde(default)]
    pub thread: Thread,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outofband_invite: Option<OutofbandInvitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedState {
    error: Option<ProblemReport>,
    #[serde(default)]
    thread: Thread,
}

impl From<(InitializedState, Invitation)> for InvitedState {
    fn from((_state, invitation): (InitializedState, Invitation)) -> InvitedState {
        trace!("DidExchangeStateSM: transit state from InitializedState to InvitedState with ConnectionInvitation");
        InvitedState { invitation: Invitations::ConnectionInvitation(invitation) }
    }
}

impl From<(InitializedState, OutofbandInvitation)> for InvitedState {
    fn from((_state, invitation): (InitializedState, OutofbandInvitation)) -> InvitedState {
        trace!("DidExchangeStateSM: transit state from InitializedState to InvitedState with OutofbandInvitation");
        InvitedState { invitation: Invitations::OutofbandInvitation(invitation) }
    }
}

impl From<(InitializedState, OutofbandInvitation)> for CompleteState {
    fn from((_state, invitation): (InitializedState, OutofbandInvitation)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from InitializedState to CompleteState with Out-of-Band Invitation");
        let thread = Thread::new()
            .set_pthid(invitation.id.to_string());

        CompleteState {
            did_doc: DidDoc::from(invitation.clone()),
            protocols: None,
            thread,
            outofband_invite: Some(invitation),
        }
    }
}

impl From<(InvitedState, ProblemReport, Thread)> for FailedState {
    fn from((_state, error, thread): (InvitedState, ProblemReport, Thread)) -> FailedState {
        trace!("DidExchangeStateSM: transit state from InvitedState to FailedState with ProblemReport message: {:?}", error);
        trace!("Thread: {:?}", thread);
        FailedState { error: Some(error), thread }
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

impl From<(RequestedState, ProblemReport, Thread)> for FailedState {
    fn from((_state, error, thread): (RequestedState, ProblemReport, Thread)) -> FailedState {
        trace!("DidExchangeStateSM: transit state from RequestedState to FailedState with ProblemReport: {:?}", error);
        trace!("Thread: {:?}", thread);
        FailedState { error: Some(error), thread }
    }
}

impl From<(RequestedState, Response, Thread)> for CompleteState {
    fn from((_state, response, thread): (RequestedState, Response, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RequestedState to RespondedState");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: response.connection.did_doc, protocols: None, thread, outofband_invite: None }
    }
}

impl From<(RespondedState, ProblemReport, Thread)> for FailedState {
    fn from((_state, error, thread): (RespondedState, ProblemReport, Thread)) -> FailedState {
        trace!("DidExchangeStateSM: transit state from RespondedState to FailedState with ProblemReport message: {:?}", error);
        trace!("Thread: {:?}", thread);
        FailedState { error: Some(error), thread }
    }
}

impl From<(RespondedState, Ack, Thread)> for CompleteState {
    fn from((state, _ack, thread): (RespondedState, Ack, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState with Ack");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: state.did_doc, protocols: None, thread, outofband_invite: None }
    }
}

impl From<(RespondedState, Ping, Thread)> for CompleteState {
    fn from((state, _ping, thread): (RespondedState, Ping, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState with Ping");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: state.did_doc, protocols: None, thread, outofband_invite: None }
    }
}

impl From<(RespondedState, PingResponse, Thread)> for CompleteState {
    fn from((state, _ping_response, thread): (RespondedState, PingResponse, Thread)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from RespondedState to CompleteState with PingResponse");
        trace!("Thread: {:?}", thread);
        CompleteState { did_doc: state.did_doc, protocols: None, thread, outofband_invite: None }
    }
}

impl From<(CompleteState, Vec<ProtocolDescriptor>)> for CompleteState {
    fn from((state, protocols): (CompleteState, Vec<ProtocolDescriptor>)) -> CompleteState {
        trace!("DidExchangeStateSM: transit state from CompleteState to CompleteState");
        CompleteState { did_doc: state.did_doc, protocols: Some(protocols), thread: state.thread, outofband_invite: None }
    }
}

impl InitializedState {
    fn prepare_invitation(self, source_id: &str, agent_info: &AgentInfo) -> VcxResult<ActorDidExchangeState> {
        trace!("InvitedState:prepare_invitation >>> source_id: {:?}", source_id);

        let label = settings::get_config_value(settings::CONFIG_INSTITUTION_NAME).unwrap_or(source_id.to_string());

        let state = match self.outofband_meta.clone() {
            None => {
                let invite: Invitation = Invitation::create()
                    .set_label(label)
                    .set_service_endpoint(agent_info.agency_endpoint()?)
                    .set_recipient_keys(agent_info.recipient_keys())
                    .set_routing_keys(agent_info.routing_keys()?);

                ActorDidExchangeState::Inviter(DidExchangeState::Invited((self, invite).into()))
            }
            Some(outofband_meta) => {
                let invite: OutofbandInvitation = OutofbandInvitation::create()
                    .set_label(label)
                    .set_opt_goal_code(outofband_meta.goal_code)
                    .set_opt_goal(outofband_meta.goal)
                    .set_handshake(outofband_meta.handshake)
                    .set_service(
                        Service::create()
                            .set_id(SERVICE_ID.to_string())
                            .set_service_endpoint(agent_info.agency_endpoint()?)
                            .set_recipient_keys(agent_info.recipient_keys())
                            .set_routing_keys(agent_info.routing_keys()?)
                    )
                    .set_opt_request_attach(outofband_meta.request_attach)?;

                if outofband_meta.handshake {
                    ActorDidExchangeState::Inviter(DidExchangeState::Invited((self, invite).into()))
                } else {
                    ActorDidExchangeState::Inviter(DidExchangeState::Completed((self, invite).into()))
                }
            }
        };

        Ok(state)
    }
}

impl InvitedState {
    fn handle_connection_request(&self, request: &Request,
                                 agent_info: &AgentInfo) -> VcxResult<(SignedResponse, AgentInfo, Thread)> {
        trace!("InvitedState:handle_connection_request >>> request: {:?}, agent_info: {:?}", request, agent_info);

        request.connection.did_doc.validate()?;

        let prev_agent_info = agent_info.clone();

        // provision a new pairwise agent
        let new_agent_info: AgentInfo = agent_info.create_agent()?;

        let thread = Thread::new()
            .set_thid(request.id.to_string())
            .update_received_order(&request.connection.did_doc.id);

        let response = Response::create()
            .set_did(new_agent_info.pw_did.to_string())
            .set_service_endpoint(new_agent_info.agency_endpoint()?)
            .set_keys(new_agent_info.recipient_keys(), new_agent_info.routing_keys()?)
            .ask_for_ack();

        let signed_response = response.clone()
            .set_thread(thread.clone())
            .encode(&prev_agent_info.pw_vk)?;

        new_agent_info.send_message(&signed_response.to_a2a_message(), &request.connection.did_doc)?;

        Ok((signed_response, new_agent_info, thread))
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
                .set_thread(Thread::from_parent(&thread))
                .to_a2a_message()
        };

        agent_info.send_message(&message, &response.connection.did_doc)?;

        Ok((response, thread))
    }
}

impl RespondedState {
    fn handle_ack(&self, ack: &Ack) -> VcxResult<Thread> {
        self.thread.check_message_order(&self.did_doc.id, &ack.thread)?;
        let thread = self.thread.clone().update_received_order(&self.did_doc.id);
        Ok(thread)
    }

    fn handle_ping(&self, ping: &Ping, agent_info: &AgentInfo) -> VcxResult<Thread> {
        self.thread.check_message_order(&self.did_doc.id, &ping.thread.clone().unwrap_or_default())?;
        _handle_ping(ping, agent_info, &self.did_doc)?;
        let thread = self.thread.clone().update_received_order(&self.did_doc.id);
        Ok(thread)
    }

    fn handle_ping_response(&self, ping_response: &PingResponse) -> VcxResult<Thread> {
        self.thread.check_message_order(&self.did_doc.id, &ping_response.thread)?;
        let thread = self.thread.clone().update_received_order(&self.did_doc.id);
        Ok(thread)
    }

    fn send_problem_report(&self, agent_info: &AgentInfo, err: String) -> VcxResult<(ProblemReport, Thread)> {
        let thread = self.thread.clone()
            .increment_sender_order()
            .update_received_order(&self.did_doc.id);

        let problem_report = ProblemReport::create()
            .set_explain(err)
            .set_thread(thread.clone());

        agent_info.send_message(&problem_report.to_a2a_message(), &self.did_doc).ok();

        Ok((problem_report, thread))
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
            DidExchangeMessages::SendHandshakeReuse(invitation) => {
                self.handle_send_reuse(invitation, agent_info)?;
                DidExchangeState::Completed(self)
            }
            DidExchangeMessages::HandshakeReuseReceived(handshake_reuse) => {
                self.handle_reuse(handshake_reuse, agent_info)?;
                DidExchangeState::Completed(self)
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

    fn handle_send_reuse(&self, invitation: OutofbandInvitation, agent_info: &AgentInfo) -> VcxResult<()> {
        let handshake_reuse = HandshakeReuse::create();

        let thread = Thread::new()
            .set_thid(handshake_reuse.id.to_string())
            .set_pthid(invitation.id.to_string());

        let handshake_reuse = handshake_reuse
            .set_thread(thread);

        agent_info.send_message(&handshake_reuse.to_a2a_message(), &self.did_doc).ok();
        Ok(())
    }

    fn handle_reuse(&self, handshake_reuse: HandshakeReuse, agent_info: &AgentInfo) -> VcxResult<()> {
        let thread = handshake_reuse.thread.clone()
            .update_received_order(&self.did_doc.id);

        let handshake_reuse_accepted = HandshakeReuseAccepted::create()
            .set_thread(thread);

        agent_info.send_message(&handshake_reuse_accepted.to_a2a_message(), &self.did_doc).ok();
        Ok(())
    }

    pub fn send_message(&self, message: &A2AMessage, agent_info: &AgentInfo) -> VcxResult<()> {
        self.warn_if_onetime_connection();
        agent_info.send_message(message, &self.did_doc)
    }

    pub fn warn_if_onetime_connection(&self) {
        if self.outofband_invite.is_some() {
            warn!("You are using one-time connection. The other side of communication might have erased it already")
        }
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
    pub fn new(actor: Actor, source_id: &str, meta: Option<OutofbandMeta>) -> Self {
        match actor {
            Actor::Inviter => {
                DidExchangeSM {
                    source_id: source_id.to_string(),
                    state: ActorDidExchangeState::Inviter(
                        DidExchangeState::Initialized(
                            InitializedState { outofband_meta: meta }
                        )
                    ),
                    agent_info: AgentInfo::default(),
                }
            }
            Actor::Invitee => {
                DidExchangeSM {
                    source_id: source_id.to_string(),
                    state: ActorDidExchangeState::Invitee(
                        DidExchangeState::Initialized(
                            InitializedState { outofband_meta: None }
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
                    DidExchangeState::Initialized(state) => {
                        match message {
                            DidExchangeMessages::Connect() => {
                                agent_info = agent_info.create_agent()?;
                                state.prepare_invitation(&source_id, &agent_info)?
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Initialized(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            DidExchangeMessages::ExchangeRequestReceived(request) => {
                                match state.handle_connection_request(&request, &agent_info) {
                                    Ok((response, new_agent_info, thread)) => {
                                        let prev_agent_info = agent_info.clone();
                                        agent_info = new_agent_info;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Responded((state, request, response, prev_agent_info, thread).into()))
                                    }
                                    Err(err) => {
                                        let thread = Thread::new()
                                            .set_thid(request.id.to_string())
                                            .update_received_order(&request.connection.did_doc.id);

                                        let problem_report = ProblemReport::create()
                                            .set_problem_code(ProblemCode::RequestProcessingError)
                                            .set_explain(err.to_string())
                                            .set_thread(thread.clone());

                                        agent_info.send_message(&problem_report.to_a2a_message(), &request.connection.did_doc).ok(); // IS is possible?
                                        ActorDidExchangeState::Inviter(DidExchangeState::Failed((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                let thread = Thread::new()
                                    .set_thid(problem_report.id.to_string());

                                ActorDidExchangeState::Inviter(DidExchangeState::Failed((state, problem_report, thread).into()))
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
                                    Ok(thread) => {
                                        ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ack, thread).into()))
                                    }
                                    Err(err) => {
                                        let (problem_report, thread) = state.send_problem_report(&agent_info, err.to_string())?;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Failed((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::PingReceived(ping) => {
                                match state.handle_ping(&ping, &agent_info) {
                                    Ok(thread) => {
                                        ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ping, thread).into()))
                                    }
                                    Err(err) => {
                                        let (problem_report, thread) = state.send_problem_report(&agent_info, err.to_string())?;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Failed((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                let thread = state.thread.clone()
                                    .update_received_order(&state.did_doc.id);

                                ActorDidExchangeState::Inviter(DidExchangeState::Failed((state, problem_report, thread).into()))
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
                                match state.handle_ping_response(&ping_response) {
                                    Ok(thread) => {
                                        ActorDidExchangeState::Inviter(DidExchangeState::Completed((state, ping_response, thread).into()))
                                    }
                                    Err(err) => {
                                        let (problem_report, thread) = state.send_problem_report(&agent_info, err.to_string())?;
                                        ActorDidExchangeState::Inviter(DidExchangeState::Failed((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            _ => {
                                ActorDidExchangeState::Inviter(DidExchangeState::Responded(state))
                            }
                        }
                    }
                    DidExchangeState::Failed(state) => {
                        ActorDidExchangeState::Inviter(DidExchangeState::Failed(state))
                    }
                    DidExchangeState::Completed(state) => {
                        ActorDidExchangeState::Inviter(state.handle_message(message, &agent_info)?)
                    }
                }
            }
            ActorDidExchangeState::Invitee(state) => {
                match state {
                    DidExchangeState::Initialized(state) => {
                        match message {
                            DidExchangeMessages::InvitationReceived(invitation) => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Invited((state, invitation).into()))
                            }
                            DidExchangeMessages::OutofbandInvitationReceived(invitation) => {
                                if invitation.handshake_protocols.len() > 0 {
                                    ActorDidExchangeState::Invitee(DidExchangeState::Invited((state, invitation).into()))
                                } else {
                                    ActorDidExchangeState::Invitee(DidExchangeState::Completed((state, invitation).into()))
                                }
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Initialized(state))
                            }
                        }
                    }
                    DidExchangeState::Invited(state) => {
                        match message {
                            DidExchangeMessages::Connect() => {
                                agent_info = agent_info.create_agent()?;

                                let label = settings::get_config_value(settings::CONFIG_INSTITUTION_NAME).unwrap_or(source_id.to_string());

                                let request = Request::create()
                                    .set_label(label)
                                    .set_did(agent_info.pw_did.to_string())
                                    .set_service_endpoint(agent_info.agency_endpoint()?)
                                    .set_keys(agent_info.recipient_keys(), agent_info.routing_keys()?);

                                let thread = Thread::new()
                                    .set_thid(request.id.to_string())
                                    .set_opt_pthid(state.invitation.pthid());

                                agent_info.send_message(&request.to_a2a_message(), &DidDoc::from(state.invitation.clone()))?;
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested((state, request, thread).into()))
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                let thread = problem_report.thread.clone();
                                ActorDidExchangeState::Invitee(DidExchangeState::Failed((state, problem_report, thread).into()))
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
                                        ActorDidExchangeState::Invitee(DidExchangeState::Failed((state, problem_report, thread).into()))
                                    }
                                }
                            }
                            DidExchangeMessages::ProblemReportReceived(problem_report) => {
                                let thread = state.thread.clone()
                                    .update_received_order(&state.did_doc.id);

                                ActorDidExchangeState::Invitee(DidExchangeState::Failed((state, problem_report, thread).into()))
                            }
                            _ => {
                                ActorDidExchangeState::Invitee(DidExchangeState::Requested(state))
                            }
                        }
                    }
                    DidExchangeState::Responded(state) => {
                        ActorDidExchangeState::Invitee(DidExchangeState::Responded(state))
                    }
                    DidExchangeState::Failed(state) => {
                        ActorDidExchangeState::Invitee(DidExchangeState::Failed(state))
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
            ActorDidExchangeState::Invitee(ref state) | ActorDidExchangeState::Inviter(ref state) =>
                match state {
                    DidExchangeState::Failed(_) => None,
                    DidExchangeState::Initialized(_) => None,
                    DidExchangeState::Invited(ref state) => Some(DidDoc::from(state.invitation.clone())),
                    DidExchangeState::Requested(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Responded(ref state) => Some(state.did_doc.clone()),
                    DidExchangeState::Completed(ref state) => Some(state.did_doc.clone()),
                }
        }
    }

    pub fn completed_connection(&self) -> Option<CompletedConnection> {
        match self.state {
            ActorDidExchangeState::Invitee(ref state) | ActorDidExchangeState::Inviter(ref state) =>
                match state {
                    DidExchangeState::Completed(ref state) => Some(CompletedConnection {
                        agent: self.agent_info.clone(),
                        data: state.clone(),
                    }),
                    _ => None
                },
        }
    }

    pub fn get_invitation(&self) -> Option<Invitations> {
        match self.state {
            ActorDidExchangeState::Inviter(DidExchangeState::Invited(ref state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Invited(ref state)) => Some(state.invitation.clone()),
            ActorDidExchangeState::Inviter(DidExchangeState::Completed(ref state)) |
            ActorDidExchangeState::Invitee(DidExchangeState::Completed(ref state)) => {
                state.outofband_invite.as_ref().map(|invite|Invitations::OutofbandInvitation(invite.clone()))
            }
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
    use v3::messages::outofband::invitation::tests::{_invitation as _outofband_invitation, _invitation_no_handshake as _outofband_invitation_no_handshake};

    pub fn _ack() -> Ack {
        let mut ack = t_ack();
        ack.thread.sender_order = 1;
        ack
    }

    pub fn _outofband_meta() -> OutofbandMeta {
        OutofbandMeta {
            goal_code: None,
            goal: Some(String::from("Test Meta")),
            handshake: true,
            request_attach: None,
        }
    }

    pub mod inviter {
        use super::*;

        pub fn inviter_sm() -> DidExchangeSM {
            DidExchangeSM::new(Actor::Inviter, &source_id(), None)
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

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Initialized(_)), inviter_sm.state);
                assert_eq!(source_id(), inviter_sm.source_id());
            }
        }

        mod step {
            use super::*;
            use v3::messages::connection::response::tests::_thread;

            #[test]
            fn test_did_exchange_init() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let did_exchange_sm = DidExchangeSM::new(Actor::Inviter, &source_id(), None);

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Initialized(state)) => {
                        assert!(state.outofband_meta.is_none());
                        Ok(())
                    }
                    other => Err(format!("State expected to be Initialized, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_init_outofband() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let did_exchange_sm = DidExchangeSM::new(Actor::Inviter, &source_id(), Some(_outofband_meta()));

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Initialized(state)) => {
                        assert!(state.outofband_meta.is_some());
                        Ok(())
                    }
                    other => Err(format!("State expected to be Initialized, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_connect_message_from_initialized_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Invited(state)) => {
                        match state.invitation {
                            Invitations::ConnectionInvitation(_) => Ok(()),
                            Invitations::OutofbandInvitation(other) => Err(format!("Invitation expected to be in `Connections` protocol format, but: {:?}", other))
                        }
                    }
                    other => Err(format!("State expected to be Initialized, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_connect_message_from_initialized_outofband_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = DidExchangeSM::new(Actor::Inviter, &source_id(), Some(_outofband_meta()));

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Invited(state)) => {
                        match state.invitation {
                            Invitations::OutofbandInvitation(_) => Ok(()),
                            Invitations::ConnectionInvitation(other) => Err(format!("Invitation expected to be in `Out-of-Band` protocol format, but: {:?}", other))
                        }
                    }
                    other => Err(format!("State expected to be Initialized, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_connect_message_from_initialized_outofband_state_with_no_handshake() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let outofband_meta = OutofbandMeta {
                    goal_code: None,
                    goal: Some(String::from("Test Meta")),
                    handshake: false,
                    request_attach: None,
                };

                let mut did_exchange_sm = DidExchangeSM::new(Actor::Inviter, &source_id(), Some(outofband_meta));

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Completed(_)) => Ok(()),
                    other => Err(format!("State expected to be Completed, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_other_messages_from_initialized_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Initialized(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();
                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Initialized(_)), did_exchange_sm.state);
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

                assert_match!(ActorDidExchangeState::Inviter(DidExchangeState::Failed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_invited_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_invited_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Failed(state)) => {
                        assert!(state.error.is_some(), "Expected `error` value to be set");
                        Ok(())
                    }
                    _ => Err(String::from("Unexpected Inviter state"))
                }
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
            fn test_did_exchange_handle_problem_report_message_from_responded_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = inviter_sm().to_inviter_responded_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Inviter(DidExchangeState::Failed(state)) => {
                        assert!(state.error.is_some(), "Expected `error` value to be set");
                        Ok(())
                    }
                    _ => Err(String::from("Unexpected Inviter state"))
                }
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
            DidExchangeSM::new(Actor::Invitee, &source_id(), None)
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

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Initialized(_)), invitee_sm.state);
                assert_eq!(source_id(), invitee_sm.source_id());
            }
        }

        mod step {
            use super::*;

            #[test]
            fn test_did_exchange_init() {
                let _setup = AgencyModeSetup::init();

                let did_exchange_sm = invitee_sm();

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Initialized(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_invite_message_from_initialized_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::InvitationReceived(_invitation())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Invitee(DidExchangeState::Invited(state)) => {
                        assert_eq!(None, state.invitation.pthid());
                        Ok(())
                    }
                    other => Err(format!("State expected to be Invited, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_outofband_invite_message_from_initialized_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();
                let invitation = _outofband_invitation();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::OutofbandInvitationReceived(invitation.clone())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Invitee(DidExchangeState::Invited(state)) => {
                        assert_eq!(invitation.id.to_string(), state.invitation.pthid().unwrap());
                        Ok(())
                    }
                    other => Err(format!("State expected to be Invited, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_outofband_invite_without_handshake_message_from_initialized_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();
                let invitation = _outofband_invitation_no_handshake();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::OutofbandInvitationReceived(invitation.clone())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Invitee(DidExchangeState::Completed(state)) => {
                        assert!(state.outofband_invite.is_some());
                        assert_eq!(invitation.id.to_string(), state.thread.pthid.unwrap());
                        Ok(())
                    }
                    other => Err(format!("State expected to be Invited, but: {:?}", other))
                }
            }

            #[test]
            fn test_did_exchange_handle_other_message_from_initialized_state() {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::Connect()).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Initialized(_)), did_exchange_sm.state);

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::AckReceived(_ack())).unwrap();
                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Initialized(_)), did_exchange_sm.state);
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
            fn test_did_exchange_handle_problem_report_message_from_invited_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_invited_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Invitee(DidExchangeState::Failed(state)) => {
                        assert!(state.error.is_some(), "Expected `error` value to be set");
                        Ok(())
                    }
                    state => Err(format!("Unexpected Invitee state: {:?}", state))
                }
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

                assert_match!(ActorDidExchangeState::Invitee(DidExchangeState::Failed(_)), did_exchange_sm.state);
            }

            #[test]
            fn test_did_exchange_handle_problem_report_message_from_requested_state() -> Result<(), String> {
                let _setup = AgencyModeSetup::init();

                let mut did_exchange_sm = invitee_sm().to_invitee_requested_state();

                did_exchange_sm = did_exchange_sm.step(DidExchangeMessages::ProblemReportReceived(_problem_report())).unwrap();

                match did_exchange_sm.state {
                    ActorDidExchangeState::Invitee(DidExchangeState::Failed(state)) => {
                        assert!(state.error.is_some(), "Expected `error` value to be set");
                        Ok(())
                    }
                    _ => Err(String::from("Unexpected Invitee state"))
                }
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

