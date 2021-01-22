use error::prelude::*;
use utils::libindy::anoncreds;
use std::convert::TryInto;

use v3::handlers::proof_presentation::prover::states::ProverSM;
use v3::handlers::proof_presentation::prover::messages::ProverMessages;
use v3::messages::a2a::A2AMessage;
use v3::messages::proof_presentation::presentation_proposal::{PresentationPreview, PresentationProposal};
use v3::messages::proof_presentation::presentation_request::PresentationRequest;
use connection;

use messages::proofs::proof_message::ProofMessage;

use v3::messages::proof_presentation::presentation::Presentation;
use v3::messages::error::ProblemReport;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prover {
    prover_sm: ProverSM
}

impl Prover {
    pub fn create(source_id: &str, presentation_request: PresentationRequest) -> VcxResult<Prover> {
        trace!("Prover::create >>> source_id: {}, presentation_request: {:?}", source_id, secret!(presentation_request));
        debug!("Prover {}: Creating Prover state object", source_id);
        Ok(Prover {
            prover_sm: ProverSM::new(presentation_request, source_id.to_string()),
        })
    }

    pub fn create_proposal(source_id: &str, presentation_proposal: PresentationProposal) -> VcxResult<Prover> {
        trace!("Prover::create_proposal >>> source_id: {}, presentation_proposal: {:?}", source_id, secret!(presentation_proposal));
        debug!("Prover {}: Creating Prover state object", source_id);
        Ok(Prover {
            prover_sm: ProverSM::new_proposal(presentation_proposal, source_id.to_string()),
        })
    }

    pub fn state(&self) -> u32 { self.prover_sm.state() }

    pub fn retrieve_credentials(&self) -> VcxResult<String> {
        trace!("Prover::retrieve_credentials >>>");
        debug!("Prover {}: Retrieving credentials for proof generation", self.get_source_id());

        let presentation_request = self.prover_sm.presentation_request()?.request_presentations_attach.content()?;
        anoncreds::libindy_prover_get_credentials_for_proof_req(&presentation_request)
    }

    pub fn generate_presentation(&mut self, credentials: String, self_attested_attrs: String) -> VcxResult<()> {
        trace!("Prover::generate_presentation >>> credentials: {}, self_attested_attrs: {:?}", secret!(credentials), secret!(self_attested_attrs));
        debug!("Prover {}: Generating presentation", self.get_source_id());

        self.step(ProverMessages::PreparePresentation((credentials, self_attested_attrs)))
    }

    pub fn generate_presentation_msg(&self) -> VcxResult<String> {
        trace!("Prover::generate_presentation_msg >>>");
        debug!("Prover {}: Generating presentation message", self.get_source_id());

        let proof: ProofMessage = self.prover_sm.presentation()?.clone().try_into()?;

        ::serde_json::to_string(&proof)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::SerializationError, format!("Cannot serialize ProofMessage. Err: {:?}", err)))
    }

    pub fn set_presentation(&mut self, presentation: Presentation) -> VcxResult<()> {
        trace!("Prover::set_presentation >>>");
        debug!("Prover {}: Setting presentation", self.get_source_id());
        self.step(ProverMessages::SetPresentation(presentation))
    }

    pub fn send_presentation(&mut self, connection_handle: u32) -> VcxResult<()> {
        trace!("Prover::send_presentation >>>");
        debug!("Prover {}: Sending presentation", self.get_source_id());
        self.step(ProverMessages::SendPresentation(connection_handle))
    }

    pub fn send_proposal(&mut self, connection_handle: u32) -> VcxResult<()> {
        trace!("Prover::send_proposal >>>");
        debug!("Prover {}: Sending proposal", self.get_source_id());
        self.step(ProverMessages::SendProposal(connection_handle))
    }

    pub fn update_state(&mut self, message: Option<&str>) -> VcxResult<()> {
        trace!("Prover::update_state >>> message: {:?}", secret!(message));
        debug!("Prover {}: Updating state", self.get_source_id());

        if !self.prover_sm.has_transitions() { return Ok(()); }

        if let Some(message_) = message {
            return self.update_state_with_message(message_);
        }

        let agent_info = match self.prover_sm.get_agent_info() {
            Some(agent_info) => agent_info.clone(),
            None => {
                warn!("Could not update Prover state: no information about Connection.");
                return Ok(());
            }
        };

        let messages = agent_info.get_messages()?;
        if let Some((uid, message)) = self.prover_sm.find_message_to_handle(messages) {
            self.handle_message(message.into())?;
            agent_info.update_message_status(uid)?;
        };

        Ok(())
    }

    pub fn update_state_with_message(&mut self, message: &str) -> VcxResult<()> {
        trace!("Prover::update_state_with_message >>> message: {:?}", secret!(message));
        debug!("Prover {}: Updating state with message", self.get_source_id());

        let a2a_message: A2AMessage = ::serde_json::from_str(&message)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson,
                                              format!("Cannot updated Prover state with messages: Message deserialization failed with: {:?}", err)))?;

        self.handle_message(a2a_message.into())?;

        Ok(())
    }

    pub fn handle_message(&mut self, message: ProverMessages) -> VcxResult<()> {
        trace!("Prover::handle_message >>> message: {:?}", secret!(message));
        self.step(message)
    }

    pub fn get_presentation_request(connection_handle: u32, msg_id: &str) -> VcxResult<PresentationRequest> {
        trace!("Prover::get_presentation_request >>> connection_handle: {:?}, msg_id: {:?}", connection_handle, msg_id);
        debug!("Prover: Getting presentation request {} from the agent", msg_id);

        let message = connection::get_message_by_id(connection_handle, msg_id.to_string())?;

        let presentation_request: PresentationRequest = match message {
            A2AMessage::PresentationRequest(presentation_request) => presentation_request,
            msg => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse,
                                              format!("Message of different type has been received. Expected: PresentationRequest. Received: {:?}", msg)));
            }
        };

        Ok(presentation_request)
    }

    pub fn get_presentation_request_messages(connection_handle: u32, match_name: Option<&str>) -> VcxResult<Vec<PresentationRequest>> {
        trace!("Prover::get_presentation_request_messages >>> connection_handle: {:?}, match_name: {:?}", connection_handle, secret!(match_name));
        debug!("prover: Getting all presentation requests from the agent");

        let presentation_requests: Vec<PresentationRequest> =
            connection::get_messages(connection_handle)?
                .into_iter()
                .filter_map(|(_, message)| {
                    match message {
                        A2AMessage::PresentationRequest(presentation_request) => {
                            Some(presentation_request)
                        }
                        _ => None,
                    }
                })
                .collect();

        Ok(presentation_requests)
    }

    pub fn get_source_id(&self) -> String { self.prover_sm.source_id() }

    pub fn step(&mut self, message: ProverMessages) -> VcxResult<()> {
        self.prover_sm = self.prover_sm.clone().step(message)?;
        Ok(())
    }

    pub fn decline_presentation_request(&mut self, connection_handle: u32, reason: Option<String>, proposal: Option<String>) -> VcxResult<()> {
        trace!("Prover::decline_presentation_request >>> connection_handle: {}, reason: {:?}, proposal: {:?}", connection_handle, secret!(reason), secret!(proposal));
        debug!("Prover {}: Declining presentation request", self.get_source_id());

        match (reason, proposal) {
            (Some(reason), None) => {
                self.step(ProverMessages::RejectPresentationRequest((connection_handle, reason)))
            }
            (None, Some(proposal)) => {
                let presentation_preview: PresentationPreview = serde_json::from_str(&proposal)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot parse Presentation Preview from JSON string. Err: {:?}", err)))?;

                self.step(ProverMessages::ProposePresentation((connection_handle, presentation_preview)))
            }
            (None, None) => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidOption, "Either `reason` or `proposal` parameter must be specified."));
            }
            (Some(_), Some(_)) => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidOption, "Only one of `reason` or `proposal` parameters must be specified."));
            }
        }
    }

    pub fn get_problem_report_message(&self) -> VcxResult<String> {
        trace!("Prover::get_problem_report_message >>>");
        debug!("Prover {}: Getting problem report message", self.get_source_id());

        let problem_report: Option<&ProblemReport> = self.prover_sm.problem_report();
        Ok(json!(&problem_report).to_string())
    }
}