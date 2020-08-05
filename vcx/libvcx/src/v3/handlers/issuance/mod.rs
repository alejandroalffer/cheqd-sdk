pub mod issuer;
pub mod states;
pub mod messages;
pub mod holder;

use error::prelude::*;
use v3::messages::a2a::A2AMessage;
use v3::handlers::issuance::issuer::IssuerSM;
use v3::handlers::issuance::messages::CredentialIssuanceMessage;
use v3::handlers::issuance::holder::HolderSM;
use v3::messages::issuance::credential::Credential;
use v3::messages::issuance::credential_offer::CredentialOffer;
use connection;

// Issuer

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issuer {
    issuer_sm: IssuerSM
}

impl Issuer {
    pub fn create(cred_def_handle: u32, credential_data: &str, source_id: &str, credential_name: &str) -> VcxResult<Issuer> {
        trace!("Issuer::issuer_create_credential >>> cred_def_handle: {:?}, credential_data: {:?}, source_id: {:?}",
               cred_def_handle, secret!(credential_data), source_id);
        debug!("Issuer {}: Creating credential Issuer state object", source_id);

        let cred_def_id = ::credential_def::get_cred_def_id(cred_def_handle)?;
        let rev_reg_id = ::credential_def::get_rev_reg_id(cred_def_handle)?;
        let tails_file = ::credential_def::get_tails_file(cred_def_handle)?;
        let issuer_sm = IssuerSM::new(&cred_def_id, credential_data, rev_reg_id, tails_file, source_id, credential_name);
        Ok(Issuer { issuer_sm })
    }

    pub fn send_credential_offer(&mut self, connection_handle: u32) -> VcxResult<()> {
        debug!("Issuer {}: Sending credential offer", self.get_source_id()?);
        self.step(CredentialIssuanceMessage::CredentialInit(connection_handle))
    }

    pub fn send_credential(&mut self, connection_handle: u32) -> VcxResult<()> {
        debug!("Issuer {}: Sending credential", self.get_source_id()?);
        self.step(CredentialIssuanceMessage::CredentialSend(connection_handle))
    }

    pub fn get_state(&self) -> VcxResult<u32> {
        Ok(self.issuer_sm.state())
    }

    pub fn get_source_id(&self) -> VcxResult<String> {
        Ok(self.issuer_sm.get_source_id())
    }

    pub fn get_credential_offer(&self) -> VcxResult<CredentialOffer> {
        self.issuer_sm.get_credential_offer()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Issuer object state: `offer` not found", self.get_source_id()?)))
    }

    pub fn update_status(&mut self, msg: Option<String>) -> VcxResult<u32> {
        trace!("Issuer {}: update_state >>> msg: {:?}", self.get_source_id()?, secret!(msg));
        debug!("Issuer {}: updating state", self.get_source_id()?);

        match msg {
            Some(msg) => {
                let message: A2AMessage = ::serde_json::from_str(&msg)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson,
                                                      format!("Cannot updated Issuer state with messages: Message deserialization failed with: {:?}", err)))?;

                self.step(message.into())?;
            }
            None => {
                self.issuer_sm = self.issuer_sm.clone().update_state()?;
            }
        };

        let state = self.get_state()?;

        trace!("Issuer::update_state <<< state: {:?}", state);
        Ok(state)
    }

    pub fn step(&mut self, message: CredentialIssuanceMessage) -> VcxResult<()> {
        self.issuer_sm = self.issuer_sm.clone().handle_message(message)?;
        Ok(())
    }
}

// Holder

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Holder {
    holder_sm: HolderSM
}

impl Holder {
    pub fn create(credential_offer: CredentialOffer, source_id: &str) -> VcxResult<Holder> {
        trace!("Holder::holder_create_credential >>> credential_offer: {:?}, source_id: {:?}", credential_offer, source_id);
        debug!("Holder {}: Creating credential Holder state object", source_id);

        let holder_sm = HolderSM::new(credential_offer, source_id.to_string());

        Ok(Holder { holder_sm })
    }

    pub fn send_request(&mut self, connection_handle: u32) -> VcxResult<()> {
        trace!("Holder::send_request >>>");
        debug!("Holder {}: Sending credential request", self.get_source_id());
        self.step(CredentialIssuanceMessage::CredentialRequestSend(connection_handle))
    }

    pub fn send_reject(&mut self, connection_handle: u32, comment: Option<String>) -> VcxResult<()> {
        trace!("Holder::send_reject >>> comment: {:?}", comment);
        debug!("Holder {}: Sending credential reject", self.get_source_id());
        self.step(CredentialIssuanceMessage::CredentialRejectSend((connection_handle, comment)))
    }

    pub fn update_state(&mut self, msg: Option<String>) -> VcxResult<u32> {
        trace!("Holder: update_state >>> msg: {:?}", secret!(msg));
        debug!("Holder {}: Updating state", self.get_source_id());

        match msg {
            Some(msg) => {
                let message: A2AMessage = ::serde_json::from_str(&msg)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson,
                                                      format!("Cannot updated Holder state with messages: Message deserialization failed with: {:?}", err)))?;

                self.step(message.into())?;
            }
            None => {
                self.holder_sm = self.holder_sm.clone().update_state()?;
            }
        };

        let state = self.get_state();

        trace!("Holder::update_state <<< state: {:?}", state);
        Ok(state)
    }

    pub fn get_state(&self) -> u32 {
        self.holder_sm.state()
    }

    pub fn get_source_id(&self) -> String {
        self.holder_sm.get_source_id()
    }

    pub fn get_credential_offer(&self) -> VcxResult<CredentialOffer> {
        trace!("Holder::get_credential_offer >>>");
        debug!("Holder {}: Getting credential offer", self.get_source_id());
        self.holder_sm.get_credential_offer()
    }

    pub fn get_credential(&self) -> VcxResult<(String, Credential)> {
        trace!("Holder::get_credential >>>");
        debug!("Holder {}: Getting credential", self.get_source_id());
        self.holder_sm.get_credential()
    }

    pub fn step(&mut self, message: CredentialIssuanceMessage) -> VcxResult<()> {
        self.holder_sm = self.holder_sm.clone().handle_message(message)?;
        Ok(())
    }

    pub fn get_credential_offer_message(connection_handle: u32, msg_id: &str) -> VcxResult<CredentialOffer> {
        trace!("Holder::get_credential_offer_message >>> connection_handle: {}, msg_id: {}", connection_handle, msg_id);
        debug!("Holder: Getting credential offer {} from the agent", msg_id);

        let message = connection::get_message_by_id(connection_handle, msg_id.to_string())?;

        let credential_offer: CredentialOffer = match message {
            A2AMessage::CredentialOffer(credential_offer) => credential_offer,
            msg => {
                return Err(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse,
                                              format!("Message of different type has been received. Expected: CredentialOffer. Received: {:?}", msg)));
            }
        };

        trace!("Holder: get_credential_offer_message <<< credential_offer: {:?}", secret!(credential_offer));
        Ok(credential_offer)
    }

    pub fn get_credential_offer_messages(conn_handle: u32) -> VcxResult<Vec<CredentialOffer>> {
        trace!("Holder::get_credential_offer_messages >>>");
        debug!("Holder: Getting all credential offers from the agent");

        let messages = connection::get_messages(conn_handle)?;
        let msgs: Vec<CredentialOffer> = messages
            .into_iter()
            .filter_map(|(_, a2a_message)| {
                match a2a_message {
                    A2AMessage::CredentialOffer(credential_offer) => {
                        Some(credential_offer)
                    }
                    _ => None
                }
            })
            .collect();

        trace!("Holder: get_credential_offer_messages <<<");
        Ok(msgs)
    }
}
