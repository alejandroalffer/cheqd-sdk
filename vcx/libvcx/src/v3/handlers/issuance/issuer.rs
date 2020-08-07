use api::VcxStateType;
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::states::{IssuerState, InitialState, RequestReceivedState};
use v3::messages::a2a::A2AMessage;
use v3::messages::issuance::credential_offer::CredentialOffer;
use v3::messages::issuance::credential::Credential;
use v3::messages::error::{ProblemReport, ProblemReportCodes};
use v3::messages::mime_type::MimeType;
use v3::messages::status::Status;
use messages::thread::Thread;

use issuer_credential::encode_attributes;

use utils::libindy::anoncreds::{self, libindy_issuer_create_credential_offer};
use error::{VcxResult, VcxError, VcxErrorKind};

use std::collections::HashMap;
use v3::handlers::connection::agent::AgentInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuerSM {
    state: IssuerState,
    source_id: String,
}

impl IssuerSM {
    pub fn new(cred_def_id: &str, credential_data: &str, rev_reg_id: Option<String>,
               tails_file: Option<String>, source_id: &str, credential_name: &str) -> Self {
        IssuerSM {
            state: IssuerState::Initial(InitialState::new(cred_def_id,
                                                          credential_data,
                                                          rev_reg_id,
                                                          tails_file,
                                                          Some(credential_name.to_string()))),
            source_id: source_id.to_string(),
        }
    }

    pub fn get_source_id(&self) -> String {
        self.source_id.clone()
    }

    pub fn step(state: IssuerState, source_id: String) -> Self {
        IssuerSM {
            state,
            source_id,
        }
    }

    pub fn update_state(self) -> VcxResult<Self> {
        trace!("Issuer::update_state >>> ", );

        if self.is_terminal_state() { return Ok(self); }

        let agent = match self.get_agent_info() {
            Some(agent_info) => agent_info.clone(),
            None => {
                warn!("Could not update Issuer state: no information about Connection.");
                return Ok(self);
            }
        };

        let messages = agent.get_messages()?;

        match self.find_message_to_handle(messages) {
            Some((uid, msg)) => {
                let state = self.handle_message(msg.into())?;
                agent.update_message_status(uid)?;
                Ok(state)
            }
            None => Ok(self)
        }
    }

    fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, A2AMessage)> {
        trace!("Issuer::find_message_to_handle >>> messages: {:?}", secret!(messages));
        debug!("Issuer: Finding message to update state");

        for (uid, message) in messages {
            match self.state {
                IssuerState::Initial(_) => {
                    // do not process messages
                }
                IssuerState::OfferPrepared(ref state) => {
                    match message {
                        A2AMessage::CredentialRequest(credential_request) => {
                            if credential_request.from_thread(&state.offer.id.to_string()) {
                                debug!("Issuer: CredentialRequest message received");
                                return Some((uid, A2AMessage::CredentialRequest(credential_request)));
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) |
                        A2AMessage::CredentialReject(problem_report) => {
                            if problem_report.from_thread(&state.offer.id.to_string()) {
                                debug!("Issuer: CredentialReject message received");
                                return Some((uid, A2AMessage::CommonProblemReport(problem_report)));
                            }
                        }
                        message => {
                            warn!("Issuer: Unexpected message received in OfferSent state: {:?}", message);
                        }
                    }
                }
                IssuerState::OfferSent(ref state) => {
                    match message {
                        A2AMessage::CredentialRequest(credential) => {
                            if credential.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                debug!("Issuer: CredentialRequest message received");
                                return Some((uid, A2AMessage::CredentialRequest(credential)));
                            }
                        }
                        A2AMessage::CredentialProposal(credential_proposal) => {
                            if let Some(ref thread) = credential_proposal.thread {
                                debug!("Issuer: CredentialProposal message received");
                                if thread.is_reply(&state.thread.thid.clone().unwrap_or_default()) {
                                    return Some((uid, A2AMessage::CredentialProposal(credential_proposal)));
                                }
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) |
                        A2AMessage::CredentialReject(problem_report) => {
                            if problem_report.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                debug!("Issuer: CredentialReject message received");
                                return Some((uid, A2AMessage::CommonProblemReport(problem_report)));
                            }
                        }
                        message => {
                            warn!("Issuer: Unexpected message received in OfferSent state: {:?}", message);
                        }
                    }
                }
                IssuerState::RequestReceived(_) => {
                    // do not process messages
                }
                IssuerState::CredentialSent(ref state) => {
                    match message {
                        A2AMessage::Ack(ack) | A2AMessage::CredentialAck(ack) => {
                            if ack.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                return Some((uid, A2AMessage::CredentialAck(ack)));
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) |
                        A2AMessage::CredentialReject(problem_report) => {
                            if problem_report.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                return Some((uid, A2AMessage::CommonProblemReport(problem_report)));
                            }
                        }
                        message => {
                            warn!("Issuer: Unexpected message received in CredentialSent state: {:?}", message);
                        }
                    }
                }
                IssuerState::Finished(_) => {
                    // do not process messages
                }
            };
        }
        debug!("Issuer: no message to update state");
        None
    }

    pub fn state(&self) -> u32 {
        match self.state {
            IssuerState::Initial(_) => VcxStateType::VcxStateInitialized as u32,
            IssuerState::OfferSent(_) => VcxStateType::VcxStateOfferSent as u32,
            IssuerState::OfferPrepared(_) => VcxStateType::VcxStateOfferSent as u32,
            IssuerState::RequestReceived(_) => VcxStateType::VcxStateRequestReceived as u32,
            IssuerState::CredentialSent(_) => VcxStateType::VcxStateAccepted as u32,
            IssuerState::Finished(ref status) => {
                match status.status {
                    Status::Success => VcxStateType::VcxStateAccepted as u32,
                    _ => VcxStateType::VcxStateNone as u32,
                }
            }
        }
    }

    pub fn handle_message(self, cim: CredentialIssuanceMessage) -> VcxResult<IssuerSM> {
        trace!("Issuer::handle_message >>> cim: {:?}", secret!(cim));
        debug!("Issuer: Updating state");

        let IssuerSM { state, source_id } = self;
        let state = match state {
            IssuerState::Initial(state_data) => match cim {
                CredentialIssuanceMessage::SendCredentialOffer(connection_handle) => {
                    let connection = ::connection::get_completed_connection(connection_handle)?;
                    let mut cred_offer = state_data.generate_credential_offer()?;

                    let thread = Thread::new()
                        .set_thid(cred_offer.id.to_string())
                        .set_opt_pthid(connection.data.thread.pthid.clone());

                    cred_offer = cred_offer.set_thread(thread.clone());

                    connection.data.send_message(&cred_offer.to_a2a_message(), &connection.agent)?;
                    IssuerState::OfferSent((state_data, cred_offer, connection, thread).into())
                }
                CredentialIssuanceMessage::PrepareCredentialOffer() => {
                    let cred_offer = state_data.generate_credential_offer()?;
                    IssuerState::OfferPrepared((state_data, cred_offer).into())
                }
                _ => {
                    warn!("Credential Issuance can only start on issuer side with init");
                    IssuerState::Initial(state_data)
                }
            }
            IssuerState::OfferPrepared(state_data) => match cim {
                CredentialIssuanceMessage::SetConnection(connection_handle) => {
                    let connection = ::connection::get_completed_connection(connection_handle)?;
                    IssuerState::OfferPrepared((state_data, connection).into())
                }
                CredentialIssuanceMessage::SendCredentialOffer(connection_handle) => {
                    let connection = ::connection::get_completed_connection(connection_handle)?;

                    let thread = Thread::new()
                        .set_thid(state_data.offer.id.to_string())
                        .set_opt_pthid(connection.data.thread.pthid.clone());

                    connection.data.send_message(&state_data.offer.to_a2a_message(), &connection.agent)?;
                    IssuerState::OfferSent((state_data, connection, thread).into())
                }
                CredentialIssuanceMessage::CredentialRequest(request) => {
                    let connection = state_data.connection.clone()
                        .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Issuer object state: `connection` not found", source_id)))?;

                    let thread = request.thread.clone()
                        .update_received_order(&connection.data.did_doc.id);

                    IssuerState::RequestReceived((state_data, request, connection, thread).into())
                }
                CredentialIssuanceMessage::ProblemReport(problem_report) => {
                    let connection = state_data.connection.as_ref()
                        .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Issuer object state: `connection` not found", source_id)))?;

                    let thread = problem_report.thread.clone()
                        .update_received_order(&connection.data.did_doc.id);

                    IssuerState::Finished((state_data, problem_report, thread).into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Request, Proposal and Problem Report");
                    IssuerState::OfferPrepared(state_data)
                }
            }
            IssuerState::OfferSent(state_data) => match cim {
                CredentialIssuanceMessage::CredentialRequest(request) => {
                    let thread = state_data.thread.clone()
                        .update_received_order(&state_data.connection.data.did_doc.id);

                    IssuerState::RequestReceived((state_data, request, thread).into())
                }
                CredentialIssuanceMessage::CredentialProposal(_) => {
                    let thread = state_data.thread.clone()
                        .increment_sender_order()
                        .update_received_order(&state_data.connection.data.did_doc.id);

                    let problem_report = ProblemReport::create()
                        .set_description(ProblemReportCodes::Unimplemented)
                        .set_comment(String::from("credential-proposal message is not supported"))
                        .set_thread(thread.clone());

                    state_data.connection.data.send_message(&A2AMessage::CredentialReject(problem_report.clone()), &state_data.connection.agent)?;
                    IssuerState::Finished((state_data, problem_report, thread).into())
                }
                CredentialIssuanceMessage::ProblemReport(problem_report) => {
                    let thread = state_data.thread.clone()
                        .update_received_order(&state_data.connection.data.did_doc.id);

                    IssuerState::Finished((state_data, problem_report, thread).into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Request, Proposal and Problem Report");
                    IssuerState::OfferSent(state_data)
                }
            },
            IssuerState::RequestReceived(state_data) => match cim {
                CredentialIssuanceMessage::CredentialSend(connection_handle) => {
                    let connection = ::connection::get_completed_connection(connection_handle)?;

                    let thread = state_data.thread.clone()
                        .increment_sender_order()
                        .update_received_order(&state_data.connection.data.did_doc.id);

                    match state_data.create_credential() {
                        Ok(credential_msg) => {
                            let credential_msg = credential_msg
                                .set_thread(thread.clone());

                            connection.data.send_message(&credential_msg.to_a2a_message(), &connection.agent)?;
                            IssuerState::Finished((state_data, thread).into())
                        }
                        Err(err) => {
                            let problem_report = ProblemReport::create()
                                .set_description(ProblemReportCodes::InvalidCredentialRequest)
                                .set_comment(format!("error occurred: {:?}", err))
                                .set_thread(thread.clone());

                            state_data.connection.data.send_message(&A2AMessage::CredentialReject(problem_report.clone()), &connection.agent)?;
                            IssuerState::Finished((state_data, problem_report, thread).into())
                        }
                    }
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only CredentialSend");
                    IssuerState::RequestReceived(state_data)
                }
            }
            IssuerState::CredentialSent(state_data) => match cim {
                CredentialIssuanceMessage::ProblemReport(_problem_report) => {
                    info!("Interaction closed with failure");
                    let thread = state_data.thread.clone()
                        .update_received_order(&state_data.connection.data.did_doc.id);

                    IssuerState::Finished((state_data, thread).into())
                }
                CredentialIssuanceMessage::CredentialAck(_ack) => {
                    info!("Interaction closed with success");
                    let thread = state_data.thread.clone()
                        .update_received_order(&state_data.connection.data.did_doc.id);

                    IssuerState::Finished((state_data, thread).into())
                }
                _ => {
                    warn!("In this state Credential Issuance can accept only Ack and Problem Report");
                    IssuerState::CredentialSent(state_data)
                }
            }
            IssuerState::Finished(state_data) => {
                warn!("Exchange is finished, no messages can be sent or received");
                IssuerState::Finished(state_data)
            }
        };

        trace!("Issuer::handle_message <<< state: {:?}", secret!(state));
        Ok(IssuerSM::step(state, source_id))
    }

    pub fn is_terminal_state(&self) -> bool {
        match self.state {
            IssuerState::Finished(_) => true,
            _ => false
        }
    }

    pub fn get_agent_info(&self) -> Option<&AgentInfo> {
        match self.state {
            IssuerState::OfferSent(ref state) => Some(&state.connection.agent),
            IssuerState::OfferPrepared(ref state) => state.connection.as_ref().map(|connection| &connection.agent),
            IssuerState::RequestReceived(ref state) => Some(&state.connection.agent),
            IssuerState::CredentialSent(ref state) => Some(&state.connection.agent),
            IssuerState::Initial(_) => None,
            IssuerState::Finished(_) => None,
        }
    }

    pub fn get_credential_offer(&self) -> Option<CredentialOffer> {
        match self.state {
            IssuerState::Initial(_) => None,
            IssuerState::OfferPrepared(ref state) => Some(state.offer.clone()),
            IssuerState::OfferSent(ref state) => Some(state.offer.clone()),
            IssuerState::RequestReceived(ref state) => Some(state.offer.clone()),
            IssuerState::CredentialSent(ref state) => Some(state.offer.clone()),
            IssuerState::Finished(ref state) => state.offer.clone(),
        }
    }
}

impl InitialState {
    fn generate_credential_offer(&self) -> VcxResult<CredentialOffer> {
        trace!("Issuer::generate_credential_offer >>>");

        let cred_offer = libindy_issuer_create_credential_offer(&self.cred_def_id)?;
        let cred_offer = CredentialOffer::create()
            .set_comment(Some(format!("{} is offering you a credential: {}",
                                      ::settings::get_config_value(::settings::CONFIG_INSTITUTION_NAME)?,
                                      self.credential_name.clone().unwrap_or_default()
            )))
            .set_offers_attach(&cred_offer)?;
        let cred_offer = self.append_credential_preview(cred_offer)?;

        trace!("Issuer::generate_credential_offer <<< cred_offer: {:?}", cred_offer);

        Ok(cred_offer)
    }

    fn append_credential_preview(&self, cred_offer_msg: CredentialOffer) -> VcxResult<CredentialOffer> {
        trace!("Issuer::InitialState::append_credential_preview >>> cred_offer_msg: {:?}", secret!(cred_offer_msg));

        let cred_values: serde_json::Value = serde_json::from_str(&self.credential_json)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidAttributesStructure,
                                              format!("Cannot parse Credential Preview from JSON string. Err: {:?}", err)))?;

        let values_map = cred_values.as_object()
            .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidAttributesStructure,
                                              "Invalid Credential Preview Json".to_string()))?;

        let mut new_offer = cred_offer_msg;
        for item in values_map.iter() {
            let (key, value) = item;
            new_offer = new_offer.add_credential_preview_data(
                key,
                value.as_str()
                    .ok_or_else(|| VcxError::from_msg(VcxErrorKind::InvalidJson,
                                                      "Invalid Credential Preview Json".to_string()))?,
                MimeType::Plain,
            )?;
        }

        trace!("Issuer::InitialState::append_credential_preview <<<");
        Ok(new_offer)
    }
}

impl RequestReceivedState {
    fn create_credential(&self) -> VcxResult<Credential> {
        trace!("Issuer::RequestReceivedState::create_credential >>>");

        self.thread.check_message_order(&self.connection.data.did_doc.id, &self.request.thread)?;

        let request = &self.request.requests_attach.content()?;

        let cred_data = encode_attributes(&self.cred_data)?;

        let (credential, _, _) = anoncreds::libindy_issuer_create_credential(&self.offer.offers_attach.content()?,
                                                                             &request,
                                                                             &cred_data,
                                                                             self.rev_reg_id.clone(),
                                                                             self.tails_file.clone())?;
        let credential = Credential::create()
            .set_credential(credential)?;

        trace!("Issuer::RequestReceivedState::create_credential <<<");
        Ok(credential)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use utils::devsetup::SetupAriesMocks;
    use v3::handlers::connection::tests::mock_connection;
    use v3::test::source_id;
    use v3::messages::issuance::test::{_ack, _problem_report};
    use v3::messages::issuance::credential::tests::_credential;
    use v3::messages::issuance::credential_request::tests::_credential_request;
    use v3::messages::issuance::credential_proposal::tests::_credential_proposal;
    use v3::messages::issuance::credential_offer::tests::_credential_offer;

    fn _issuer_sm() -> IssuerSM {
        IssuerSM::new("test", &json!({"name": "alice"}).to_string(), None, None, &source_id(), "test")
    }

    impl IssuerSM {
        fn to_offer_sent_state(mut self) -> IssuerSM {
            self = self.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            self
        }

        fn to_request_received_state(mut self) -> IssuerSM {
            self = self.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            self = self.handle_message(CredentialIssuanceMessage::CredentialRequest(_credential_request())).unwrap();
            self
        }

        fn to_finished_state(mut self) -> IssuerSM {
            self = self.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            self = self.handle_message(CredentialIssuanceMessage::CredentialRequest(_credential_request())).unwrap();
            self = self.handle_message(CredentialIssuanceMessage::CredentialSend(mock_connection())).unwrap();
            self
        }
    }

    mod new {
        use super::*;

        #[test]
        fn test_issuer_new() {
            let _setup = SetupAriesMocks::init();

            let issuer_sm = _issuer_sm();

            assert_match!(IssuerState::Initial(_), issuer_sm.state);
            assert_eq!(source_id(), issuer_sm.get_source_id());
        }
    }

    mod handle_message {
        use super::*;
        use v3::messages::issuance::credential_request::CredentialRequest;

        #[test]
        fn test_issuer_init() {
            let _setup = SetupAriesMocks::init();

            let issuer_sm = _issuer_sm();

            assert_match!(IssuerState::Initial(_), issuer_sm.state);
        }

        #[test]
        fn test_issuer_handle_credential_init_message_from_initial_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();

            assert_match!(IssuerState::OfferSent(_), issuer_sm.state);
        }

        #[test]
        fn test_issuer_handle_other_messages_from_initial_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::Credential(_credential())).unwrap();
            assert_match!(IssuerState::Initial(_), issuer_sm.state);

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialRequest(_credential_request())).unwrap();
            assert_match!(IssuerState::Initial(_), issuer_sm.state);
        }

        #[test]
        fn test_issuer_handle_credential_request_message_from_offer_sent_state() -> Result<(), String> {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();

            let credential_request = _credential_request();

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialRequest(credential_request.clone())).unwrap();

            match issuer_sm.state {
                IssuerState::RequestReceived(state) => {
                    assert_eq!(credential_request.thread.thid, state.thread.thid);
                    assert_eq!(0, state.thread.sender_order);
                    Ok(())
                }
                other => Err(format!("State expected to be RequestReceived, but: {:?}", other))
            }
        }

        #[test]
        fn test_issuer_handle_credential_proposal_message_from_offer_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialProposal(_credential_proposal())).unwrap();

            assert_match!(IssuerState::Finished(_), issuer_sm.state);
            assert_eq!(VcxStateType::VcxStateNone as u32, issuer_sm.state());
        }

        #[test]
        fn test_issuer_handle_problem_report_message_from_offer_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::ProblemReport(_problem_report())).unwrap();

            assert_match!(IssuerState::Finished(_), issuer_sm.state);
            assert_eq!(VcxStateType::VcxStateNone as u32, issuer_sm.state());
        }

        #[test]
        fn test_issuer_handle_other_messages_from_offer_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::Credential(_credential())).unwrap();

            assert_match!(IssuerState::OfferSent(_), issuer_sm.state);
        }

        #[test]
        fn test_issuer_handle_credential_send_message_from_request_received_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialRequest(_credential_request())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialSend(mock_connection())).unwrap();

            assert_match!(IssuerState::Finished(_), issuer_sm.state);
            assert_eq!(VcxStateType::VcxStateAccepted as u32, issuer_sm.state());
        }

        #[test]
        fn test_issuer_handle_credential_send_message_from_request_received_state_with_invalid_request() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialRequest(CredentialRequest::create())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialSend(mock_connection())).unwrap();

            assert_match!(IssuerState::Finished(_), issuer_sm.state);
            assert_eq!(VcxStateType::VcxStateNone as u32, issuer_sm.state());
        }

        #[test]
        fn test_issuer_handle_other_messages_from_request_received_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialRequest(_credential_request())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialSend(mock_connection())).unwrap();

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialSend(mock_connection())).unwrap();
            assert_match!(IssuerState::Finished(_), issuer_sm.state);

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialAck(_ack())).unwrap();
            assert_match!(IssuerState::Finished(_), issuer_sm.state);
        }

        // TRANSITIONS TO/FROM CREDENTIAL SENT STATE AREN'T POSSIBLE NOW

        #[test]
        fn test_issuer_handle_messages_from_finished_state() {
            let _setup = SetupAriesMocks::init();

            let mut issuer_sm = _issuer_sm();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialRequest(_credential_request())).unwrap();
            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialSend(mock_connection())).unwrap();

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::SendCredentialOffer(mock_connection())).unwrap();
            assert_match!(IssuerState::Finished(_), issuer_sm.state);

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::CredentialRequest(_credential_request())).unwrap();
            assert_match!(IssuerState::Finished(_), issuer_sm.state);

            issuer_sm = issuer_sm.handle_message(CredentialIssuanceMessage::Credential(_credential())).unwrap();
            assert_match!(IssuerState::Finished(_), issuer_sm.state);
        }
    }

    mod find_message_to_handle {
        use super::*;

        #[test]
        fn test_issuer_find_message_to_handle_from_initial_state() {
            let _setup = SetupAriesMocks::init();

            let issuer = _issuer_sm();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::CredentialRequest(_credential_request()),
                    "key_3".to_string() => A2AMessage::CredentialProposal(_credential_proposal()),
                    "key_4".to_string() => A2AMessage::Credential(_credential()),
                    "key_5".to_string() => A2AMessage::CredentialAck(_ack()),
                    "key_6".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(issuer.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_issuer_find_message_to_handle_from_offer_sent_state() {
            let _setup = SetupAriesMocks::init();

            let issuer = _issuer_sm().to_offer_sent_state();

            // CredentialRequest
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::Credential(_credential()),
                    "key_3".to_string() => A2AMessage::CredentialRequest(_credential_request())
                );

                let (uid, message) = issuer.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::CredentialRequest(_), message);
            }

            // CredentialProposal
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::CredentialAck(_ack()),
                    "key_3".to_string() => A2AMessage::Credential(_credential()),
                    "key_4".to_string() => A2AMessage::CredentialProposal(_credential_proposal())
                );

                let (uid, message) = issuer.find_message_to_handle(messages).unwrap();
                assert_eq!("key_4", uid);
                assert_match!(A2AMessage::CredentialProposal(_), message);
            }

            // Problem Report
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::CredentialAck(_ack()),
                    "key_3".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                let (uid, message) = issuer.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::CommonProblemReport(_), message);
            }

            // Credential Reject
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::CredentialAck(_ack()),
                    "key_3".to_string() => A2AMessage::CredentialReject(_problem_report())
                );

                let (uid, message) = issuer.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::CommonProblemReport(_), message);
            }

            // No messages for different Thread ID
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer().set_thread_id("")),
                    "key_2".to_string() => A2AMessage::CredentialRequest(_credential_request().set_thread_id("")),
                    "key_3".to_string() => A2AMessage::CredentialProposal(_credential_proposal().set_thread_id("")),
                    "key_4".to_string() => A2AMessage::Credential(_credential().set_thread_id("")),
                    "key_5".to_string() => A2AMessage::CredentialAck(_ack().set_thread_id("")),
                    "key_6".to_string() => A2AMessage::CommonProblemReport(_problem_report().set_thread_id(""))
                );

                assert!(issuer.find_message_to_handle(messages).is_none());
            }

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::CredentialAck(_ack())
                );

                assert!(issuer.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_issuer_find_message_to_handle_from_request_state() {
            let _setup = SetupAriesMocks::init();

            let issuer = _issuer_sm().to_finished_state();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::CredentialRequest(_credential_request()),
                    "key_3".to_string() => A2AMessage::CredentialProposal(_credential_proposal()),
                    "key_4".to_string() => A2AMessage::Credential(_credential()),
                    "key_5".to_string() => A2AMessage::CredentialAck(_ack()),
                    "key_6".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(issuer.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_issuer_find_message_to_handle_from_credential_sent_state() {
            let _setup = SetupAriesMocks::init();

            let issuer = _issuer_sm().to_finished_state();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::CredentialOffer(_credential_offer()),
                    "key_2".to_string() => A2AMessage::CredentialRequest(_credential_request()),
                    "key_3".to_string() => A2AMessage::CredentialProposal(_credential_proposal()),
                    "key_4".to_string() => A2AMessage::Credential(_credential()),
                    "key_5".to_string() => A2AMessage::CredentialAck(_ack()),
                    "key_6".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(issuer.find_message_to_handle(messages).is_none());
            }
        }
    }

    mod get_state {
        use super::*;

        #[test]
        fn test_get_state() {
            let _setup = SetupAriesMocks::init();

            assert_eq!(VcxStateType::VcxStateInitialized as u32, _issuer_sm().state());
            assert_eq!(VcxStateType::VcxStateOfferSent as u32, _issuer_sm().to_offer_sent_state().state());
            assert_eq!(VcxStateType::VcxStateRequestReceived as u32, _issuer_sm().to_request_received_state().state());
            assert_eq!(VcxStateType::VcxStateAccepted as u32, _issuer_sm().to_finished_state().state());
        }
    }
}