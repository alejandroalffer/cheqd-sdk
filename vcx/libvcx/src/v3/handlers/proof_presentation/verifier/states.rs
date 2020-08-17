use api::VcxStateType;

use v3::handlers::proof_presentation::verifier::messages::VerifierMessages;
use v3::messages::a2a::A2AMessage;
use v3::messages::proof_presentation::presentation_request::{PresentationRequest, PresentationRequestData};
use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::proof_presentation::presentation_ack::PresentationAck;
use v3::messages::error::{ProblemReport, ProblemReportCodes};
use v3::messages::status::Status;
use proof::Proof;

use std::collections::HashMap;
use error::prelude::*;
use v3::handlers::connection::types::CompletedConnection;
use messages::thread::Thread;
use v3::handlers::connection::agent::AgentInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifierSM {
    source_id: String,
    state: VerifierState,
}

impl VerifierSM {
    pub fn new(presentation_request: PresentationRequestData, source_id: String) -> VerifierSM {
        VerifierSM {
            source_id,
            state: VerifierState::Initiated(
                InitialState { presentation_request_data: presentation_request }
            ),
        }
    }
}

// Possible Transitions:
//
// Initial -> PresentationRequestSent
// SendPresentationRequest -> Finished
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerifierState {
    Initiated(InitialState),
//    PresentationRequestPrepared(PresentationRequestPreparedState),
    PresentationRequestSent(PresentationRequestSentState),
    Finished(FinishedState),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialState {
    presentation_request_data: PresentationRequestData
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationRequestPreparedState {
    presentation_request: PresentationRequest,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationRequestSentState {
    presentation_request: PresentationRequest,
    connection: CompletedConnection,
    #[serde(default)]
    thread: Thread,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    presentation_request: PresentationRequest,
    presentation: Option<Presentation>,
    status: Status,
    #[serde(default)]
    thread: Thread,
}

impl From<(InitialState, PresentationRequest, CompletedConnection, Thread)> for PresentationRequestSentState {
    fn from((_state, presentation_request, connection, thread): (InitialState, PresentationRequest, CompletedConnection, Thread)) -> Self {
        trace!("VerifierSM transit state from InitialState to PresentationRequestSentState");
        trace!("Thread: {:?}", thread);
        PresentationRequestSentState {
            connection,
            presentation_request,
            thread,
        }
    }
}

impl From<(PresentationRequestSentState, Presentation, Thread)> for FinishedState {
    fn from((state, presentation, thread): (PresentationRequestSentState, Presentation, Thread)) -> Self {
        trace!("VerifierSM transit state from PresentationRequestSentState to FinishedState");
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: Some(presentation),
            status: Status::Success,
            thread,
        }
    }
}

impl From<(PresentationRequestSentState, ProblemReport, Thread)> for FinishedState {
    fn from((state, problem_report, thread): (PresentationRequestSentState, ProblemReport, Thread)) -> Self {
        trace!("VerifierSM transit state from PresentationRequestSentState to FinishedState with ProblemReport: {:?}", problem_report);
        trace!("Thread: {:?}", thread);
        FinishedState {
            presentation_request: state.presentation_request,
            presentation: None,
            status: Status::Failed(problem_report),
            thread,
        }
    }
}

impl PresentationRequestSentState {
    fn verify_presentation(&self, presentation: &Presentation, thread: &Thread) -> VcxResult<Thread> {
        trace!("PresentationRequestSentState::verify_presentation >>> presentation: {:?}", secret!(presentation));
        debug!("verifier verifying received presentation");

        let mut thread = thread.clone();

        let valid = Proof::validate_indy_proof(&presentation.presentations_attach.content()?,
                                               &self.presentation_request.request_presentations_attach.content()?)?;

        if !valid {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidProof, "Presentation verification failed"));
        }

        if presentation.please_ack.is_some() {
            thread = thread.increment_sender_order();

            let ack = PresentationAck::create()
                .set_thread(thread.clone());

            self.connection.data.send_message(&A2AMessage::PresentationAck(ack), &self.connection.agent)?;
        }

        trace!("PresentationRequestSentState::verify_presentation <<<");
        Ok(thread)
    }
}

impl VerifierSM {
    pub fn find_message_to_handle(&self, messages: HashMap<String, A2AMessage>) -> Option<(String, A2AMessage)> {
        trace!("VerifierSM::find_message_to_handle >>> messages: {:?}", secret!(messages));
        debug!("Verifier: Finding message to update state");

        for (uid, message) in messages {
            match self.state {
                VerifierState::Initiated(_) => {
                    // do not process message
                }
                VerifierState::PresentationRequestSent(ref state) => {
                    match message {
                        A2AMessage::Presentation(presentation) => {
                            if presentation.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                debug!("Verifier: Presentation message received");
                                return Some((uid, A2AMessage::Presentation(presentation)));
                            }
                        }
                        A2AMessage::PresentationProposal(proposal) => {
                            match proposal.thread.as_ref() {
                                Some(thread) if thread.is_reply(&state.thread.thid.clone().unwrap_or_default()) => {
                                    debug!("Verifier: PresentationProposal message received");
                                    return Some((uid, A2AMessage::PresentationProposal(proposal)));
                                }
                                _ => return None
                            }
                        }
                        A2AMessage::CommonProblemReport(problem_report) |
                        A2AMessage::PresentationReject(problem_report) => {
                            if problem_report.from_thread(&state.thread.thid.clone().unwrap_or_default()) {
                                debug!("Verifier: PresentationReject message received");
                                return Some((uid, A2AMessage::CommonProblemReport(problem_report)));
                            }
                        }
                        _ => {}
                    }
                }
                VerifierState::Finished(_) => {
                    // do not process message
                }
            };
        }
        debug!("verifier: no message to update state");
        None
    }

    pub fn step(self, message: VerifierMessages) -> VcxResult<VerifierSM> {
        trace!("VerifierSM::step >>> message: {:?}", secret!(message));
        debug!("verifier updating state");

        let VerifierSM { source_id, state } = self;

        let state = match state {
            VerifierState::Initiated(state) => {
                match message {
                    VerifierMessages::SendPresentationRequest(connection_handle) => {
                        let connection = ::connection::get_completed_connection(connection_handle)?;

                        let presentation_request: PresentationRequestData =
                            state.presentation_request_data.clone()
                                .set_format_version_for_did(&connection.agent.pw_did, &connection.data.did_doc.id)?;

                        let presentation_request =
                            PresentationRequest::create()
                                .set_comment(presentation_request.name.clone())
                                .set_request_presentations_attach(&presentation_request)?;

                        let thread = Thread::new()
                            .set_thid(presentation_request.id.to_string())
                            .set_opt_pthid(connection.data.thread.pthid.clone());

                        connection.data.send_message(&presentation_request.to_a2a_message(), &connection.agent)?;
                        VerifierState::PresentationRequestSent((state, presentation_request, connection, thread).into())
                    }
                    _ => {
                        VerifierState::Initiated(state)
                    }
                }
            }
            VerifierState::PresentationRequestSent(state) => {
                match message {
                    VerifierMessages::PresentationReceived(presentation) => {
                        let mut thread = state.thread.clone()
                            .update_received_order(&state.connection.data.did_doc.id);

                        match state.verify_presentation(&presentation, &thread) {
                            Ok(thread) => {
                                VerifierState::Finished((state, presentation, thread).into())
                            }
                            Err(err) => {
                                thread = thread.increment_sender_order();

                                let problem_report =
                                    ProblemReport::create()
                                        .set_description(ProblemReportCodes::InvalidPresentation)
                                        .set_comment(format!("error occurred: {:?}", err))
                                        .set_thread(thread.clone());

                                state.connection.data.send_message(&problem_report.to_a2a_message(), &state.connection.agent)?;
                                VerifierState::Finished((state, problem_report, thread).into())
                            }
                        }
                    }
                    VerifierMessages::PresentationRejectReceived(problem_report) => {
                        let thread = state.thread.clone()
                            .update_received_order(&state.connection.data.did_doc.id);

                        VerifierState::Finished((state, problem_report, thread).into())
                    }
                    VerifierMessages::PresentationProposalReceived(_) => { // TODO: handle Presentation Proposal
                        let thread = state.thread.clone()
                            .increment_sender_order()
                            .update_received_order(&state.connection.data.did_doc.id);

                        let problem_report =
                            ProblemReport::create()
                                .set_description(ProblemReportCodes::Unimplemented)
                                .set_comment(String::from("presentation-proposal message is not supported"))
                                .set_thread(thread.clone());

                        state.connection.data.send_message(&problem_report.to_a2a_message(), &state.connection.agent)?;
                        VerifierState::Finished((state, problem_report, thread).into())
                    }
                    _ => {
                        VerifierState::PresentationRequestSent(state)
                    }
                }
            }
            VerifierState::Finished(state) => VerifierState::Finished(state)
        };

        Ok(VerifierSM { source_id, state })
    }

    pub fn source_id(&self) -> String { self.source_id.clone() }

    pub fn state(&self) -> u32 {
        match self.state {
            VerifierState::Initiated(_) => VcxStateType::VcxStateInitialized as u32,
            VerifierState::PresentationRequestSent(_) => VcxStateType::VcxStateOfferSent as u32,
            VerifierState::Finished(ref status) => {
                match status.status {
                    Status::Success => VcxStateType::VcxStateAccepted as u32,
                    _ => VcxStateType::VcxStateNone as u32,
                }
            }
        }
    }

    pub fn presentation_status(&self) -> u32 {
        match self.state {
            VerifierState::Finished(ref state) => state.status.code(),
            _ => Status::Undefined.code()
        }
    }

    pub fn has_transitions(&self) -> bool {
        match self.state {
            VerifierState::Initiated(_) => false,
            VerifierState::PresentationRequestSent(_) => true,
            VerifierState::Finished(_) => false,
        }
    }

    pub fn get_agent_info(&self) -> Option<&AgentInfo> {
        match self.state {
            VerifierState::Initiated(_) => None,
            VerifierState::PresentationRequestSent(ref state) => Some(&state.connection.agent),
            VerifierState::Finished(_) => None,
        }
    }

    pub fn presentation_request_data(&self) -> VcxResult<&PresentationRequestData> {
        match self.state {
            VerifierState::Initiated(ref state) => Ok(&state.presentation_request_data),
            VerifierState::PresentationRequestSent(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                                format!("Verifier object {} in state {} not ready to get Presentation Request Data message", self.source_id, self.state()))),
            VerifierState::Finished(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                 format!("Verifier object {} in state {} not ready to get Presentation Request Data message", self.source_id, self.state()))),
        }
    }

    pub fn presentation_request(&self) -> VcxResult<PresentationRequest> {
        match self.state {
            VerifierState::Initiated(ref state) =>
                Err(VcxError::from_msg(VcxErrorKind::InvalidState, "Could not get Presentation Request message. VerifierSM is not in appropriate state.")),
            VerifierState::PresentationRequestSent(ref state) => Ok(state.presentation_request.clone()),
            VerifierState::Finished(ref state) => Ok(state.presentation_request.clone()),
        }
    }

    pub fn presentation(&self) -> VcxResult<Presentation> {
        match self.state {
            VerifierState::Initiated(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                  format!("Verifier object {} in state {} not ready to get Presentation message", self.source_id, self.state()))),
            VerifierState::PresentationRequestSent(_) => Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                                                                format!("Verifier object {} in state {} not ready to get Presentation message", self.source_id, self.state()))),
            VerifierState::Finished(ref state) => {
                state.presentation.clone()
                    .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Verifier object state: `presentation` not found", self.source_id)))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use utils::devsetup::SetupAriesMocks;
    use v3::handlers::connection::tests::mock_connection;
    use v3::test::source_id;
    use v3::messages::proof_presentation::presentation_request::tests::_presentation_request;
    use v3::messages::proof_presentation::presentation_request::tests::_presentation_request_data;
    use v3::messages::proof_presentation::presentation::tests::_presentation;
    use v3::messages::proof_presentation::presentation_proposal::tests::_presentation_proposal;
    use v3::messages::proof_presentation::test::{_ack, _problem_report};

    pub fn _verifier_sm() -> VerifierSM {
        VerifierSM::new(_presentation_request_data(), source_id())
    }

    impl VerifierSM {
        fn to_presentation_request_sent_state(mut self) -> VerifierSM {
            self = self.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            self
        }

        fn to_finished_state(mut self) -> VerifierSM {
            self = self.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            self = self.step(VerifierMessages::PresentationReceived(_presentation())).unwrap();
            self
        }
    }

    mod new {
        use super::*;

        #[test]
        fn test_verifier_new() {
            let _setup = SetupAriesMocks::init();

            let verifier_sm = _verifier_sm();

            assert_match!(VerifierState::Initiated(_), verifier_sm.state);
            assert_eq!(source_id(), verifier_sm.source_id());
        }
    }

    mod step {
        use super::*;

        #[test]
        fn test_verifier_init() {
            let _setup = SetupAriesMocks::init();

            let verifier_sm = _verifier_sm();
            assert_match!(VerifierState::Initiated(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_send_presentation_request_message_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();

            assert_match!(VerifierState::PresentationRequestSent(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_other_messages_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();

            verifier_sm = verifier_sm.step(VerifierMessages::PresentationRejectReceived(_problem_report())).unwrap();
            assert_match!(VerifierState::Initiated(_), verifier_sm.state);

            verifier_sm = verifier_sm.step(VerifierMessages::PresentationReceived(_presentation())).unwrap();
            assert_match!(VerifierState::Initiated(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_verify_presentation_message_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::PresentationReceived(_presentation())).unwrap();

            assert_match!(VerifierState::Finished(_), verifier_sm.state);
            assert_eq!(VcxStateType::VcxStateAccepted as u32, verifier_sm.state());
        }

        //    #[test]
        //    fn test_prover_handle_verify_presentation_message_from_presentation_request_sent_state_for_invalid_presentation() {
        //        let _setup = Setup::init();
        //
        //        let mut verifier_sm = _verifier_sm();
        //        verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
        //        verifier_sm = verifier_sm.step(VerifierMessages::VerifyPresentation(_presentation())).unwrap();
        //
        //        assert_match!(VerifierState::Finished(_), verifier_sm.state);
        //        assert_eq!(Status::Failed(_problem_report()).code(), verifier_sm.presentation_status());
        //    }

        #[test]
        fn test_prover_handle_presentation_proposal_message_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::PresentationProposalReceived(_presentation_proposal())).unwrap();

            assert_match!(VerifierState::Finished(_), verifier_sm.state);
            assert_eq!(VcxStateType::VcxStateNone as u32, verifier_sm.state());
        }

        #[test]
        fn test_prover_handle_presentation_reject_message_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::PresentationRejectReceived(_problem_report())).unwrap();

            assert_match!(VerifierState::Finished(_), verifier_sm.state);
            assert_eq!(VcxStateType::VcxStateNone as u32, verifier_sm.state());
        }

        #[test]
        fn test_prover_handle_other_messages_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();

            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            assert_match!(VerifierState::PresentationRequestSent(_), verifier_sm.state);
        }

        #[test]
        fn test_prover_handle_messages_from_presentation_finished_state() {
            let _setup = SetupAriesMocks::init();

            let mut verifier_sm = _verifier_sm();
            verifier_sm = verifier_sm.step(VerifierMessages::SendPresentationRequest(mock_connection())).unwrap();
            verifier_sm = verifier_sm.step(VerifierMessages::PresentationReceived(_presentation())).unwrap();

            verifier_sm = verifier_sm.step(VerifierMessages::PresentationRejectReceived(_problem_report())).unwrap();
            assert_match!(VerifierState::Finished(_), verifier_sm.state);

            verifier_sm = verifier_sm.step(VerifierMessages::PresentationProposalReceived(_presentation_proposal())).unwrap();
            assert_match!(VerifierState::Finished(_), verifier_sm.state);
        }
    }

    mod find_message_to_handle {
        use super::*;

        #[test]
        fn test_verifier_find_message_to_handle_from_initiated_state() {
            let _setup = SetupAriesMocks::init();

            let verifier = _verifier_sm();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_4".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_5".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(verifier.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_verifier_find_message_to_handle_from_presentation_request_sent_state() {
            let _setup = SetupAriesMocks::init();

            let verifier = _verifier_sm().to_presentation_request_sent_state();

            // Presentation
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationAck(_ack())
                );

                let (uid, message) = verifier.find_message_to_handle(messages).unwrap();
                assert_eq!("key_2", uid);
                assert_match!(A2AMessage::Presentation(_), message);
            }

            // Presentation Proposal
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_2".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_3".to_string() => A2AMessage::PresentationAck(_ack())
                );

                let (uid, message) = verifier.find_message_to_handle(messages).unwrap();
                assert_eq!("key_2", uid);
                assert_match!(A2AMessage::PresentationProposal(_), message);
            }

            // Problem Report
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_2".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_3".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                let (uid, message) = verifier.find_message_to_handle(messages).unwrap();
                assert_eq!("key_3", uid);
                assert_match!(A2AMessage::CommonProblemReport(_), message);
            }

            // Presentation Reject
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_2".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_3".to_string() => A2AMessage::PresentationReject(_problem_report())
                );

                let (uid, message) = verifier.find_message_to_handle(messages).unwrap();
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

                assert!(verifier.find_message_to_handle(messages).is_none());
            }

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationRequest(_presentation_request())
                );

                assert!(verifier.find_message_to_handle(messages).is_none());
            }
        }

        #[test]
        fn test_verifier_find_message_to_handle_from_finished_state() {
            let _setup = SetupAriesMocks::init();

            let verifier = _verifier_sm().to_finished_state();

            // No messages
            {
                let messages = map!(
                    "key_1".to_string() => A2AMessage::PresentationProposal(_presentation_proposal()),
                    "key_2".to_string() => A2AMessage::Presentation(_presentation()),
                    "key_3".to_string() => A2AMessage::PresentationRequest(_presentation_request()),
                    "key_4".to_string() => A2AMessage::PresentationAck(_ack()),
                    "key_5".to_string() => A2AMessage::CommonProblemReport(_problem_report())
                );

                assert!(verifier.find_message_to_handle(messages).is_none());
            }
        }
    }

    mod get_state {
        use super::*;

        #[test]
        fn test_get_state() {
            let _setup = SetupAriesMocks::init();

            assert_eq!(VcxStateType::VcxStateInitialized as u32, _verifier_sm().state());
            assert_eq!(VcxStateType::VcxStateOfferSent as u32, _verifier_sm().to_presentation_request_sent_state().state());
            assert_eq!(VcxStateType::VcxStateAccepted as u32, _verifier_sm().to_finished_state().state());
        }
    }
}