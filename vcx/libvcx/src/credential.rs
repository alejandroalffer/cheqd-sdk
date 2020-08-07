use serde_json;
use serde_json::Value;
use std::convert::TryInto;

use error::prelude::*;
use object_cache::ObjectCache;
use api::VcxStateType;
use issuer_credential::PaymentInfo;
use messages::issuance::credential::CredentialMessage;
use messages::issuance::credential_request::CredentialRequest;
use messages::{
    self,
    GeneralMessage,
    RemoteMessageType,
    payload::{
        Payloads,
        PayloadKinds,
    },
    thread::Thread,
    get_message::{
        get_ref_msg,
        get_connection_messages,
    },
};
use connection;
use utils::libindy::anoncreds::{libindy_prover_create_credential_req, libindy_prover_store_credential, get_cred_def_json, libindy_prover_delete_credential, prover_get_credential};
use utils::libindy::payments::{pay_a_payee, PaymentTxn};
use utils::{error, constants};

use utils::httpclient::AgencyMock;

use v3::{
    messages::issuance::credential_offer::CredentialOffer as CredentialOfferV3,
    handlers::issuance::Holder,
};
use utils::agent_info::{get_agent_info, get_agent_attr, MyAgentInfo};
use messages::issuance::credential_offer::{set_cred_offer_ref_message, parse_json_offer, CredentialOffer};
use v3::messages::proof_presentation::presentation_proposal::{PresentationProposal, PresentationPreview};

lazy_static! {
    static ref HANDLE_MAP: ObjectCache<Credentials>  = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "version", content = "data")]
enum Credentials {
    #[serde(rename = "3.0")]
    Pending(Credential),
    #[serde(rename = "1.0")]
    V1(Credential),
    #[serde(rename = "2.0")]
    V3(Holder),
}

impl Default for Credential {
    fn default() -> Credential
    {
        Credential {
            source_id: String::new(),
            state: VcxStateType::VcxStateNone,
            credential_name: None,
            credential_request: None,
            credential_offer: None,
            msg_uid: None,
            cred_id: None,
            credential: None,
            payment_info: None,
            payment_txn: None,
            my_did: None,
            my_vk: None,
            their_did: None,
            their_vk: None,
            agent_did: None,
            agent_vk: None,
            thread: Some(Thread::new()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Credential {
    source_id: String,
    state: VcxStateType,
    credential_name: Option<String>,
    credential_request: Option<CredentialRequest>,
    credential_offer: Option<CredentialOffer>,
    msg_uid: Option<String>,
    // the following 6 are pulled from the connection object
    agent_did: Option<String>,
    agent_vk: Option<String>,
    my_did: Option<String>,
    my_vk: Option<String>,
    their_did: Option<String>,
    their_vk: Option<String>,
    credential: Option<String>,
    cred_id: Option<String>,
    payment_info: Option<PaymentInfo>,
    payment_txn: Option<PaymentTxn>,
    thread: Option<Thread>,
}

impl Credential {
    fn create(source_id: &str) -> Credential {
        let mut credential: Credential = Default::default();

        credential.state = VcxStateType::VcxStateInitialized;
        credential.set_source_id(source_id);

        credential
    }

    fn create_with_offer(source_id: &str, offer: &str) -> VcxResult<Credential> {
        trace!("create_with_offer >>> source_id: {}, offer: {}", source_id, secret!(&offer));
        debug!("Credential {}: Creating with offer", source_id);

        let mut credential = Credential::create(source_id);

        let (offer, payment_info) = parse_json_offer(offer)?;

        credential.credential_offer = Some(offer);
        credential.payment_info = payment_info;
        credential.state = VcxStateType::VcxStateRequestReceived;

        trace!("create_with_offer <<<");

        Ok(credential)
    }

    pub fn build_credential_request(&self, my_did: &str, their_did: &str) -> VcxResult<CredentialRequest> {
        trace!("Credential::build_request >>> my_did: {}, their_did: {}", secret!(my_did), secret!(their_did));
        debug!("Credential {}: Building credential request", self.source_id);

        if self.state != VcxStateType::VcxStateRequestReceived {
            return Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                          format!("Credential object {} in state {} not ready to send credential request", self.source_id, self.state as u32)));
        }

        let credential_offer = self.credential_offer.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Credential object state: `credential_offer` not found", self.source_id)))?;

        let (req, req_meta, cred_def_id, _) = Credential::create_credential_request(&credential_offer.cred_def_id,
                                                                                    &my_did,
                                                                                    &credential_offer.libindy_offer)?;

        let credential = CredentialRequest {
            libindy_cred_req: req,
            libindy_cred_req_meta: req_meta,
            cred_def_id,
            tid: String::new(),
            to_did: String::from(their_did),
            from_did: String::from(my_did),
            mid: String::new(),
            version: String::from("0.1"),
            msg_ref_id: None,
        };

        trace!("Credential::build_credential_request <<<");
        Ok(credential)
    }

    pub fn create_credential_request(cred_def_id: &str, prover_did: &str, cred_offer: &str) -> VcxResult<(String, String, String, String)> {
        trace!("Credential::create_credential_request >>> cred_def_id: {}, prover_did: {}, cred_offer: {}", secret!(cred_def_id), secret!(prover_did), secret!(cred_offer));

        let (cred_def_id, cred_def_json) = get_cred_def_json(&cred_def_id)?;

        let (cred_req, cred_req_meta) =  libindy_prover_create_credential_req(&prover_did,
                                             &cred_offer,
                                             &cred_def_json)
            .map_err(|err| err.extend("Cannot create credential request"))?;

        trace!("Credential::create_credential_request <<< cred_req: {}, cred_req_meta: {}", secret!(cred_req), secret!(cred_req_meta));

        Ok((cred_req, cred_req_meta, cred_def_id, cred_def_json))
    }

    fn generate_request_msg(&mut self, my_pw_did: &str, their_pw_did: &str) -> VcxResult<String> {
        trace!("Credential::generate_request_msg >>> my_did: {}, their_did: {}", secret!(my_pw_did), secret!(their_pw_did));
        debug!("Credential {}: Generating credential request", self.source_id);

        let cred_req: CredentialRequest = self.build_credential_request(my_pw_did, their_pw_did)?;

        let cred_req_json = json!(cred_req).to_string();

        self.credential_request = Some(cred_req);

        if self.payment_info.is_some() {
            let (payment_txn, _) = self.submit_payment()?;
            self.payment_txn = Some(payment_txn);
        }

        trace!("Credential::generate_request_msg <<< cred_req_json: {}", secret!(cred_req_json));

        Ok(cred_req_json)
    }

    fn send_request(&mut self, connection_handle: u32) -> VcxResult<u32> {
        trace!("Credential::send_request >>> connection_handle: {}", connection_handle);
        debug!("Credential {}: Sending credential request", self.source_id);

        let my_agent = get_agent_info()?.pw_info(connection_handle)?;
        apply_agent_info(self, &my_agent);

        let cred_req_json = self.generate_request_msg(&my_agent.my_pw_did()?, &my_agent.their_pw_did()?)?;

        let offer_msg_id = self.credential_offer
            .as_ref()
            .and_then(|offer| offer.msg_ref_id.clone())
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, "Invalid Credential Offer: `msg_ref_id` not found"))?;

        let response =
            messages::send_message()
                .to(&my_agent.my_pw_did()?)?
                .to_vk(&my_agent.my_pw_vk()?)?
                .msg_type(&RemoteMessageType::CredReq)?
                .agent_did(&my_agent.pw_agent_did()?)?
                .agent_vk(&my_agent.pw_agent_vk()?)?
                .version(my_agent.version.clone())?
                .edge_agent_payload(
                    &my_agent.my_pw_vk()?,
                    &my_agent.their_pw_vk()?,
                    &cred_req_json,
                    PayloadKinds::CredReq,
                    self.thread.clone(),
                )?
                .ref_msg_id(Some(offer_msg_id.to_string()))?
                .send_secure()
                .map_err(|err| err.extend("Cannot not send credential request"))?;

        self.msg_uid = Some(response.get_msg_uid()?);
        self.state = VcxStateType::VcxStateOfferSent;

        trace!("Credential::send_request <<<");
        Ok(error::SUCCESS.code_num)
    }

    fn _check_msg(&mut self, message: Option<String>) -> VcxResult<()> {
        trace!("Credential::_check_msg >>> message: {:?}", secret!(message));

        let credential = match message {
            None => {
                let msg_uid = self.msg_uid
                    .as_ref()
                    .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, "Invalid Credential object state: `msg_uid` not found"))?;

                let (_, payload) = get_ref_msg(msg_uid,
                                               &get_agent_attr(&self.my_did)?,
                                               &get_agent_attr(&self.my_vk)?,
                                               &get_agent_attr(&self.agent_did)?,
                                               &get_agent_attr(&self.agent_vk)?)?;
                let (credential, thread) = Payloads::decrypt(&get_agent_attr(&self.my_vk)?, &payload)?;

                if let Some(_) = thread {
                    let their_did = get_agent_attr(&self.their_vk)?;

                    self.thread
                        .as_mut()
                        .map(|thread| thread.increment_receiver(&their_did));
                };
                credential
            }
            Some(ref message) => message.clone(),
        };

        let credential_msg: CredentialMessage = serde_json::from_str(&credential)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredential, format!("Cannot parse Credential message from JSON string. Err: {:?}", err)))?;

        let cred_offer: &CredentialOffer = self.credential_offer.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Invalid Credential object state:`credential_offer` not found"))?;

        credential_msg.ensure_match_offer(&cred_offer)?;

        let cred_req: &CredentialRequest = self.credential_request.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, "Invalid Credential object state:`credential_request` not found"))?;

        let (_, cred_def_json) = get_cred_def_json(&cred_req.cred_def_id)?;

        self.credential = Some(credential);
        self.cred_id = Some(libindy_prover_store_credential(None,
                                                            &cred_req.libindy_cred_req_meta,
                                                            &credential_msg.libindy_cred,
                                                            &cred_def_json,
                                                            match credential_msg.rev_reg_def_json.len() {
                                                                0 => None,
                                                                _ => Some(&credential_msg.rev_reg_def_json),
                                                            })?);

        self.state = VcxStateType::VcxStateAccepted;

        trace!("Credential::_check_msg <<<");

        Ok(())
    }

    fn update_state(&mut self, message: Option<String>) -> VcxResult<u32> {
        trace!("Credential::update_state >>> message: {:?}", secret!(message));
        debug!("Credential {}: Updating state", self.source_id);

        match self.state {
            VcxStateType::VcxStateOfferSent => {
                //Check for messages
                let _ = self._check_msg(message);
            }
            VcxStateType::VcxStateAccepted => {
                //Check for revocation
            }
            _ => {
                // NOOP there is nothing the check for a changed state
            }
        };

        let state = self.get_state();

        trace!("Credential::update_state <<< state: {:?}", state);

        Ok(state)
    }

    fn get_state(&self) -> u32 {
        trace!("Credential::get_state >>>");

        let state = self.state as u32;

        debug!("Credential {} is in state {}", self.source_id, self.state as u32);
        trace!("Credential::get_state <<< state: {:?}", state);
        state
    }

    fn get_credential(&self) -> VcxResult<String> {
        trace!("Credential::get_credential >>>");
        debug!("Credential {}: Getting credential message", self.source_id);

        if self.state != VcxStateType::VcxStateAccepted {
            return Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                          format!("Credential object {} in state {} not ready to get Credential message", self.source_id, self.state as u32)))
        }

        let credential = self.credential.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState,
                                      format!("Invalid {} Credential object state: `credential` not found", self.source_id)))?;
        let credential = self.to_cred_string(&credential);

        trace!("Credential::get_credential <<< credential: {:?}", credential);

        Ok(credential)
    }

    fn get_credential_offer(&self) -> VcxResult<String> {
        trace!("Credential::get_credential_offer >>>");
        debug!("Credential {}: Getting credential offer message", self.source_id);

        if self.state != VcxStateType::VcxStateRequestReceived {
            return Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                          format!("Credential object {} in state {} not ready to get Credential Offer message", self.source_id, self.state as u32)))
        }

        let credential_offer = self.credential_offer.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState,
                                      format!("Invalid {} Credential object state: `credential_offer` not found", self.source_id)))?;

        let credential_offer_json = serde_json::to_value(credential_offer)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::SerializationError, format!("Cannot serialize CredentialOffer. Err: {}", err)))?;

        let credential_offer_json = self.to_cred_offer_string(credential_offer_json);

        trace!("Credential::get_credential_offer <<< credential_offer: {:?}", secret!(credential_offer_json));

        Ok(credential_offer_json)
    }

    fn get_credential_id(&self) -> String {
        self.cred_id.clone().unwrap_or_default()
    }

    fn set_payment_info(&self, json: &mut serde_json::Map<String, Value>) {
        if let Some(ref payment_info) = self.payment_info {
            json.insert("price".to_string(), Value::String(payment_info.price.to_string()));
            json.insert("payment_address".to_string(), Value::String(payment_info.payment_addr.to_string()));
        };
    }

    fn to_cred_string(&self, cred: &str) -> String {
        let mut json = serde_json::Map::new();
        json.insert("credential_id".to_string(), Value::String(self.get_credential_id()));
        json.insert("credential".to_string(), Value::String(cred.to_string()));
        self.set_payment_info(&mut json);
        serde_json::Value::from(json).to_string()
    }

    fn to_cred_offer_string(&self, cred_offer: Value) -> String {
        let mut json = serde_json::Map::new();
        json.insert("credential_offer".to_string(), cred_offer);
        self.set_payment_info(&mut json);
        serde_json::Value::from(json).to_string()
    }

    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }

    fn get_source_id(&self) -> String { self.source_id.to_string() }

    fn get_payment_txn(&self) -> VcxResult<PaymentTxn> {
        trace!("Credential::get_payment_txn >>>");
        debug!("Credential {}: Getting payment transaction", self.source_id);

        match (&self.payment_txn, &self.payment_info) {
            (Some(ref payment_txn), Some(_)) => Ok(payment_txn.clone()),
            _ => Err(VcxError::from(VcxErrorKind::NoPaymentInformation))
        }
    }

    fn is_payment_required(&self) -> bool {
        self.payment_info.is_some()
    }

    fn submit_payment(&self) -> VcxResult<(PaymentTxn, String)> {
        debug!("Credential {}: Submitting payment for premium credentia", self.source_id);

        match &self.payment_info {
            &Some(ref pi) => {
                let address = &pi.get_address();
                let price = pi.get_price();
                let (payment_txn, receipt) = pay_a_payee(price, address)?;
                Ok((payment_txn, receipt))
            }
            &None => Err(VcxError::from(VcxErrorKind::NoPaymentInformation)),
        }
    }

    fn get_payment_info(&self) -> VcxResult<Option<PaymentInfo>> {
        trace!("Credential::get_payment_info >>>");
        Ok(self.payment_info.clone())
    }

    #[cfg(test)]
    fn from_str(data: &str) -> VcxResult<Credential> {
        use messages::ObjectWithVersion;
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<Credential>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize Credential"))
    }

    fn set_state(&mut self, state: VcxStateType) {
        self.state = state;
    }

    fn delete_credential(&self) -> VcxResult<()> {
        debug!("Credential {}: Deleting credential", self.source_id);

        if self.state != VcxStateType::VcxStateAccepted {
            return Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                          format!("Credential object {} in state {} not ready to delete credential", self.source_id, self.state as u32)));
        }

        let cred_id = self.cred_id.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Credential object state: `cred_id` not found", self.source_id)))?;

        libindy_prover_delete_credential(cred_id)
    }

    fn get_presentation_proposal(&self) -> VcxResult<PresentationProposal> {
        trace!("Credential::get_presentation_proposal_msg >>>");
        debug!("Credential {}: Building presentation proposal", self.source_id);

        if self.state != VcxStateType::VcxStateAccepted {
            return Err(VcxError::from_msg(VcxErrorKind::NotReady,
                                          format!("Credential object {} in state {} not ready to prepare presentation proposal message", self.source_id, self.state as u32)));
        }

        let cred_id = self.cred_id.as_ref()
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState, format!("Invalid {} Credential object state: `cred_id` not found", self.source_id)))?;

        let credential = prover_get_credential(&cred_id)?;

        let presentation_proposal = PresentationProposal::default()
            .set_presentation_preview(PresentationPreview::for_credential(&credential));

        trace!("Credential::get_presentation_proposal_msg <<< presentation_proposal: {:?}", presentation_proposal);
        Ok(presentation_proposal)
    }
}

//********************************************
//         HANDLE FUNCTIONS
//********************************************
fn handle_err(err: VcxError) -> VcxError {
    if err.kind() == VcxErrorKind::InvalidHandle {
        VcxError::from(VcxErrorKind::InvalidCredentialHandle)
    } else {
        err
    }
}

fn apply_agent_info(cred: &mut Credential, agent_info: &MyAgentInfo) {
    cred.my_did = agent_info.my_pw_did.clone();
    cred.my_vk = agent_info.my_pw_vk.clone();
    cred.their_did = agent_info.their_pw_did.clone();
    cred.their_vk = agent_info.their_pw_vk.clone();
    cred.agent_did = agent_info.pw_agent_did.clone();
    cred.agent_vk = agent_info.pw_agent_vk.clone();
}

fn create_pending_credential(source_id: &str, offer: &str) -> VcxResult<Credentials> {
    trace!("create_pending_credential >>> source_id: {}, offer: {}", source_id, secret!(&offer));

    let credential = Credential::create_with_offer(source_id, offer)?;

    Ok(Credentials::Pending(credential))
}

fn create_credential_v1(source_id: &str, offer: &str) -> VcxResult<Credentials> {
    trace!("create_credential_v1 >>> source_id: {}, offer: {}", source_id, secret!(&offer));
    debug!("creating V1 credential {}", source_id);

    let credential = Credential::create_with_offer(source_id, offer)?;

    trace!("create_credential_v1 <<<");

    Ok(Credentials::V1(credential))
}

fn create_credential_v3(source_id: &str, offer: &str) -> VcxResult<Option<Credentials>> {
    trace!("create_credential_v3 >>> source_id: {}, offer: {}", source_id, secret!(&offer));
    debug!("creating V3 credential {}", source_id);

    let offer_message = ::serde_json::from_str::<serde_json::Value>(offer)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer,
                                          format!("Cannot parse CredentialOffer from `offer` JSON string. Err: {:?}", err)))?;

    let offer_message = match offer_message {
        serde_json::Value::Array(mut offer) => { // legacy offer format
            offer.pop()
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidCredentialOffer, "Cannot parse CredentialOffer from `offer` JSON string. Err: Array is empty"))?
        }
        offer => offer //aries offer format
    };

    // Received offer of aries format
    if let Ok(cred_offer) = serde_json::from_value::<CredentialOfferV3>(offer_message) {
        let holder = Holder::create(cred_offer, source_id)?;
        return Ok(Some(Credentials::V3(holder)));
    }

    trace!("create_credential_v3 <<<");
    Ok(None)
}

pub fn credential_create_with_offer(source_id: &str, offer: &str) -> VcxResult<u32> {
    trace!("credential_create_with_offer >>> source_id: {}, offer: {}", source_id, secret!(&offer));
    debug!("creating credential {}", source_id);

    let credential =
        match create_credential_v3(source_id, &offer)? {
            Some(credential) => credential,
            None => {
                create_pending_credential(source_id, offer)?
            }
        };

    let handle = HANDLE_MAP.add(credential)?;

    debug!("inserting credential {} into handle map", source_id);
    Ok(handle)
}

pub fn accept_credential_offer(source_id: &str, offer: &str, connection_handle: u32) -> VcxResult<(u32, String)> {
    trace!("accept_credential_offer >>> source_id: {}, offer: {}, connection_handle: {}", source_id, secret!(&offer), connection_handle);
    debug!("creating credential {}", source_id);

    let credential_handle = credential_create_with_offer(source_id, offer)?;
    send_credential_request(credential_handle, connection_handle)?;
    let credential_serialized = to_string(credential_handle)?;

    trace!("accept_credential_offer <<<");
    Ok((credential_handle, credential_serialized))
}

pub fn credential_create_with_msgid(source_id: &str, connection_handle: u32, msg_id: &str) -> VcxResult<(u32, String)> {
    trace!("credential_create_with_msgid >>> source_id: {}, connection_handle: {}, msg_id: {}", source_id, connection_handle, secret!(&msg_id));
    debug!("creating credential {} with message {}", source_id, msg_id);

    let offer = get_credential_offer_msg(connection_handle, &msg_id)?;

    let credential = if connection::is_v3_connection(connection_handle)? {
        create_credential_v3(source_id, &offer)?
            .ok_or(VcxError::from_msg(VcxErrorKind::InvalidConnectionHandle, format!("Connection can not be used for Proprietary Issuance protocol")))?
    } else {
        create_credential_v1(source_id, &offer)?
    };

    let handle = HANDLE_MAP.add(credential)?;

    debug!("inserting credential {} into handle map", source_id);
    Ok((handle, offer))
}

pub fn update_state(handle: u32, message: Option<String>) -> VcxResult<u32> {
    HANDLE_MAP.get_mut(handle, |obj| {
        match obj {
            Credentials::Pending(ref mut obj) => {
                obj.update_state(message.clone())
            }
            Credentials::V1(ref mut obj) => {
                obj.update_state(message.clone())
            }
            Credentials::V3(ref mut obj) => {
                obj.update_state(message.clone())
            }
        }
    }).map_err(handle_err)
}

pub fn get_credential(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |credential| {
        match credential {
            Credentials::Pending(ref credential) => {
                credential.get_credential()
            }
            Credentials::V1(ref credential) => {
                credential.get_credential()
            }
            Credentials::V3(ref credential) => {
                let (cred_id, credential) = credential.get_credential()?;
                let credential: CredentialMessage = credential.try_into()?;

                let mut json = serde_json::Map::new();
                json.insert("credential_id".to_string(), Value::String(cred_id));
                json.insert("credential".to_string(), Value::String(json!(credential).to_string()));
                return Ok(serde_json::Value::from(json).to_string());
            }
        }
    }).map_err(handle_err)
}

pub fn delete_credential(handle: u32) -> VcxResult<()> {
    HANDLE_MAP.get(handle, |credential| {
        match credential {
            Credentials::Pending(_) => {
                warn!("Cannot delete credential for Pending object");
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Cannot delete credential for Pending object: Credential is not received yet"))
            }
            Credentials::V1(ref credential) => {
                credential.delete_credential()
            }
            Credentials::V3(ref credential) => {
                credential.delete_credential()
            }
        }
    })
        .map_err(handle_err)
        .and(release(handle))
}

pub fn get_payment_txn(handle: u32) -> VcxResult<PaymentTxn> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => obj.get_payment_txn(),
            Credentials::V1(ref obj) => obj.get_payment_txn(),
            Credentials::V3(_) => Err(VcxError::from_msg(VcxErrorKind::ActionNotSupported, "Aries Credential type doesn't support this action: `get_payment_txn`."))
        }
    }).map_err(handle_err)
}

pub fn get_credential_offer(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => {
                obj.get_credential_offer()
            }
            Credentials::V1(ref obj) => {
                obj.get_credential_offer()
            }
            Credentials::V3(ref obj) => {
                let cred_offer = obj.get_credential_offer()?;
                let cred_offer: CredentialOffer = cred_offer.try_into()?;

                let cred_offer = json!({
                    "credential_offer": cred_offer
                });
                return Ok(cred_offer.to_string());
            }
        }
    }).map_err(handle_err)
}

pub fn get_credential_id(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => Ok(obj.get_credential_id()),
            Credentials::V1(ref obj) => Ok(obj.get_credential_id()),
            Credentials::V3(_) => Err(VcxError::from_msg(VcxErrorKind::ActionNotSupported, "Aries Credential type doesn't support this action: `get_credential_id`."))
        }
    }).map_err(handle_err)
}

pub fn get_state(handle: u32) -> VcxResult<u32> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => Ok(obj.get_state()),
            Credentials::V1(ref obj) => Ok(obj.get_state()),
            Credentials::V3(ref obj) => Ok(obj.get_state()),
        }
    }).map_err(handle_err)
}

pub fn generate_credential_request_msg(handle: u32, my_pw_did: &str, their_pw_did: &str) -> VcxResult<String> {
    HANDLE_MAP.get_mut(handle, |obj| {
        match obj {
            Credentials::Pending(ref mut obj) => {
                let req = obj.generate_request_msg(my_pw_did, their_pw_did)?;
                obj.set_state(VcxStateType::VcxStateOfferSent);
                Ok(req)
            }
            Credentials::V1(ref mut obj) => {
                let req = obj.generate_request_msg(my_pw_did, their_pw_did)?;
                obj.set_state(VcxStateType::VcxStateOfferSent);
                Ok(req)
            }
            Credentials::V3(_) => Err(VcxError::from_msg(VcxErrorKind::ActionNotSupported, "Aries Credential type doesn't support this action: `generate_credential_request_msg`."))
        }
    }).map_err(handle_err)
}

pub fn send_credential_request(handle: u32, connection_handle: u32) -> VcxResult<u32> {
    HANDLE_MAP.get_mut(handle, |credential| {
        let new_credential = match credential {
            Credentials::Pending(ref mut obj) => {
                // if Aries connection is established --> Convert PendingCredential object to Aries credential
                if ::connection::is_v3_connection(connection_handle)? {
                    let credential_offer = obj.credential_offer.clone()
                        .ok_or(VcxError::from_msg(VcxErrorKind::NotReady,
                                                  format!("Credential object {} in state {} not ready to get Credential Offer message", obj.source_id, obj.state as u32)))?;

                    let mut holder = Holder::create(credential_offer.try_into()?, &obj.get_source_id())?;
                    holder.send_request(connection_handle)?;

                    Credentials::V3(holder)
                } else {  // else --> Convert PendingCredential object to Proprietary credential
                    obj.send_request(connection_handle)?;
                    Credentials::V1(obj.clone())
                }
            }
            Credentials::V1(ref mut obj) => {
                obj.send_request(connection_handle)?;
                Credentials::V1(obj.clone())
            }
            Credentials::V3(ref mut obj) => {
                obj.send_request(connection_handle)?;
                Credentials::V3(obj.clone())
            }
        };
        *credential = new_credential;
        Ok(error::SUCCESS.code_num)
    }).map_err(handle_err)
}

fn get_credential_offer_msg(connection_handle: u32, msg_id: &str) -> VcxResult<String> {
    trace!("get_credential_offer_msg >>> connection_handle: {}, msg_id: {}", connection_handle, msg_id);
    debug!("getting credential offer message with id {}", msg_id);

    if connection::is_v3_connection(connection_handle)? {
        let credential_offer = Holder::get_credential_offer_message(connection_handle, msg_id)?;

        return Ok(json!(credential_offer).to_string());
    }

    let my_agent = get_agent_info()?.pw_info(connection_handle)?;

    AgencyMock::set_next_response(constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec());

    let message = get_connection_messages(&my_agent.my_pw_did()?,
                                          &my_agent.my_pw_vk()?,
                                          &my_agent.pw_agent_did()?,
                                          &my_agent.pw_agent_vk()?,
                                          Some(vec![msg_id.to_string()]),
                                          None,
                                          &my_agent.version()?)
        .map_err(|err| err.extend("Cannot get connection messages"))?;

    if message[0].msg_type != RemoteMessageType::CredOffer {
        return Err(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse,
                                      format!("Agency response contains the Message of different type. Expected: CredOffer. Received: {:?}", message[0].msg_type)));
    }

    let payload = message
        .get(0)
        .and_then(|msg| msg.payload.as_ref())
        .ok_or(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, "Received Message does not contain `payload`"))?;

    let (offer, thread) = Payloads::decrypt(&my_agent.my_pw_vk()?, payload)?;
    let payload = set_cred_offer_ref_message(&offer, thread, &message[0].uid)?;

    let offers = serde_json::to_string_pretty(&payload)
        .map_err(|err|
            VcxError::from_msg(VcxErrorKind::SerializationError,
                               format!("Cannot serialize CredentialOffer. Err: {}", err),
            ))?;

    trace!("get_credential_offer_msg <<< offers: {}", secret!(offers));
    Ok(offers)
}

pub fn get_credential_offer_messages(connection_handle: u32) -> VcxResult<String> {
    trace!("Credential::get_credential_offer_messages >>> connection_handle: {}", connection_handle);
    debug!("getting all credential offer messages from connection {}", connection::get_source_id(connection_handle).unwrap_or_default());

    if connection::is_v3_connection(connection_handle)? {
        let credential_offers = Holder::get_credential_offer_messages(connection_handle)?;
        let msgs: Vec<Vec<::serde_json::Value>> = credential_offers
            .into_iter()
            .map(|credential_offer| credential_offer.try_into())
            .collect::<VcxResult<Vec<CredentialOffer>>>()?
            .into_iter()
            .map(|msg| vec![json!(msg)])
            .collect();

        return serde_json::to_string(&msgs).
            map_err(|err| {
                VcxError::from_msg(VcxErrorKind::SerializationError, format!("Cannot serialize CredentialOffers. Err: {:?}", err))
            });
    }

    let my_agent = get_agent_info()?.pw_info(connection_handle)?;

    AgencyMock::set_next_response(constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec());

    let payload = get_connection_messages(&my_agent.my_pw_did()?,
                                          &my_agent.my_pw_vk()?,
                                          &my_agent.pw_agent_did()?,
                                          &my_agent.pw_agent_vk()?,
                                          None,
                                          None,
                                          &my_agent.version()?)
        .map_err(|err| err.extend("Cannot get messages"))?;

    let mut messages = Vec::new();

    for msg in payload {
        if msg.msg_type == RemoteMessageType::CredOffer {
            let payload = msg.payload
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, "Received Message does not contain `payload`"))?;

            let (offer, thread) = Payloads::decrypt(&my_agent.my_pw_vk()?, &payload)?;
            let payload = set_cred_offer_ref_message(&offer, thread, &msg.uid)?;

            messages.push(payload);
        }
    }

    let offers = serde_json::to_string_pretty(&messages)
        .or(Err(VcxError::from(VcxErrorKind::SerializationError)))?;

    trace!("get_credential_offer_messages >>> offers: {}", secret!(offers));
    Ok(offers)
}

pub fn release(handle: u32) -> VcxResult<()> {
    HANDLE_MAP.release(handle).map_err(handle_err)
}

pub fn release_all() {
    HANDLE_MAP.drain().ok();
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        serde_json::to_string(obj)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::SerializationError, format!("Cannot serialize Credential object. Err: {:?}", err)))
    }).map_err(handle_err)
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => Ok(obj.get_source_id()),
            Credentials::V1(ref obj) => Ok(obj.get_source_id()),
            Credentials::V3(ref obj) => Ok(obj.get_source_id()),
        }
    }).map_err(handle_err)
}

pub fn from_string(credential_data: &str) -> VcxResult<u32> {
    let credential: Credentials = serde_json::from_str(credential_data)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot Credential state object from JSON string. Err: {:?}", err)))?;

    HANDLE_MAP.add(credential)
}

pub fn is_payment_required(handle: u32) -> VcxResult<bool> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => Ok(obj.is_payment_required()),
            Credentials::V1(ref obj) => Ok(obj.is_payment_required()),
            Credentials::V3(_) => Ok(false),
        }
    }).map_err(handle_err)
}

pub fn submit_payment(handle: u32) -> VcxResult<(PaymentTxn, String)> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => obj.submit_payment(),
            Credentials::V1(ref obj) => obj.submit_payment(),
            Credentials::V3(_) => Err(VcxError::from_msg(VcxErrorKind::ActionNotSupported, "Aries Credential type doesn't support this action: `submit_payment`."))
        }
    }).map_err(handle_err)
}

pub fn get_payment_information(handle: u32) -> VcxResult<Option<PaymentInfo>> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(ref obj) => obj.get_payment_info(),
            Credentials::V1(ref obj) => obj.get_payment_info(),
            Credentials::V3(_) => Ok(None),
        }
    }).map_err(handle_err)
}

pub fn reject(handle: u32, connection_handle: u32, comment: Option<String>) -> VcxResult<()> {
    HANDLE_MAP.get_mut(handle, |credential| {
        let new_credential = match credential {
            Credentials::Pending(ref mut obj) => {
                // if Aries connection is established --> Convert PendingCredential object to Aries credential
                if ::connection::is_v3_connection(connection_handle)? {
                    let credential_offer = obj.credential_offer.clone()
                        .ok_or(VcxError::from_msg(VcxErrorKind::InvalidState,
                                                  format!("Credential object {} in state {} not ready to get Credential Offer message", obj.source_id, obj.state as u32)))?;

                    let mut holder = Holder::create(credential_offer.try_into()?, &obj.get_source_id())?;
                    holder.send_reject(connection_handle, comment.clone())?;

                    Credentials::V3(holder)
                } else {  // else --> error
                    return Err(VcxError::from_msg(VcxErrorKind::ActionNotSupported, "Proprietary Credential type doesn't support this action: `reject_credential`."));
                }
            }
            Credentials::V1(_) => {
                return Err(VcxError::from_msg(VcxErrorKind::ActionNotSupported, "Proprietary Credential type doesn't support this action: `reject_credential`."));
            }
            Credentials::V3(ref mut obj) => {
                obj.send_reject(connection_handle, comment.clone())?;
                Credentials::V3(obj.clone())
            }
        };
        *credential = new_credential;
        Ok(())
    }).map_err(handle_err)
}

pub fn get_presentation_proposal_msg(handle: u32) -> VcxResult<String> {
    HANDLE_MAP.get(handle, |obj| {
        match obj {
            Credentials::Pending(_) => {
                warn!("Cannot prepare presentation proposal for Pending Credential object");
                Err(VcxError::from_msg(VcxErrorKind::NotReady, "Cannot prepare presentation proposal for Pending object: Credential is not received yet"))
            }
            Credentials::V1(ref obj) => obj.get_presentation_proposal(),
            Credentials::V3(ref obj) => obj.get_presentation_proposal(),
        }
    })
        .map(|presentation_proposal| presentation_proposal.to_string())
        .map_err(handle_err)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use api::VcxStateType;
    use utils::devsetup::*;

    pub const BAD_CREDENTIAL_OFFER: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","claim": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    use utils::constants::{DEFAULT_SERIALIZED_CREDENTIAL,
                           DEFAULT_SERIALIZED_CREDENTIAL_PAYMENT_REQUIRED};
    use utils::libindy::payments::{build_test_address, get_wallet_token_info};

    pub fn create_credential(offer: &str) -> Credential {
        let mut credential = Credential::create("source_id");
        let (offer, payment_info) = parse_json_offer(offer).unwrap();
        credential.credential_offer = Some(offer);
        credential.payment_info = payment_info;
        credential.state = VcxStateType::VcxStateRequestReceived;
        apply_agent_info(&mut credential, &get_agent_info().unwrap());
        credential
    }

    fn create_credential_with_price(price: u64) -> Credential {
        let mut cred: Credential = Credential::from_str(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        cred.payment_info = Some(PaymentInfo {
            payment_required: "one-time".to_string(),
            payment_addr: build_test_address("OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j"),
            price,
        });
        cred
    }

    fn _get_offer(handle: u32) -> String {
        let offers = get_credential_offer_messages(handle).unwrap();
        let offers: serde_json::Value = serde_json::from_str(&offers).unwrap();
        let offer = serde_json::to_string(&offers[0]).unwrap();
        offer
    }

    #[test]
    fn test_credential_defaults() {
        let _setup = SetupDefaults::init();

        let credential = Credential::default();
        assert_eq!(credential.build_credential_request("test1", "test2").unwrap_err().kind(), VcxErrorKind::NotReady);
    }

    #[test]
    fn test_credential_create_with_offer() {
        let _setup = SetupDefaults::init();

        let handle = credential_create_with_offer("test_credential_create_with_offer", constants::CREDENTIAL_OFFER_JSON).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_credential_create_with_bad_offer() {
        let _setup = SetupDefaults::init();

        let err = credential_create_with_offer("test_credential_create_with_bad_offer", BAD_CREDENTIAL_OFFER).unwrap_err();
        assert_eq!(err.kind(), VcxErrorKind::InvalidCredentialOffer);
    }

    #[test]
    fn test_credential_serialize_deserialize() {
        let _setup = SetupDefaults::init();

        let handle = credential_create_with_offer("test_credential_serialize_deserialize", constants::CREDENTIAL_OFFER_JSON).unwrap();
        let credential_string = to_string(handle).unwrap();
        release(handle).unwrap();

        let handle = from_string(&credential_string).unwrap();
        let cred1: Credential = Credential::from_str(&credential_string).unwrap();
        assert_eq!(cred1.get_state(), 3);

        let cred2: Credential = Credential::from_str(&to_string(handle).unwrap()).unwrap();
        assert!(!cred1.is_payment_required());

        assert_eq!(cred1, cred2);
    }

    #[test]
    fn full_credential_test() {
        let _setup = SetupMocks::init();

        let connection_h = connection::tests::build_test_connection();

        let offer = _get_offer(connection_h);

        let c_h = credential_create_with_offer("TEST_CREDENTIAL", &offer).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(c_h).unwrap());

        send_credential_request(c_h, connection_h).unwrap();
        assert_eq!(VcxStateType::VcxStateOfferSent as u32, get_state(c_h).unwrap());

        assert_eq!(get_credential_id(c_h).unwrap(), "");

        AgencyMock::set_next_response(::utils::constants::CREDENTIAL_RESPONSE.to_vec());
        AgencyMock::set_next_response(::utils::constants::UPDATE_CREDENTIAL_RESPONSE.to_vec());

        update_state(c_h, None).unwrap();
        assert_eq!(get_state(c_h).unwrap(), VcxStateType::VcxStateAccepted as u32);

        assert_eq!(get_credential_id(c_h).unwrap(), "cred_id"); // this is set in test mode

        let msg = get_credential(c_h).unwrap();
        let msg_value: serde_json::Value = serde_json::from_str(&msg).unwrap();

        let _credential_struct: CredentialMessage = serde_json::from_str(msg_value["credential"].as_str().unwrap()).unwrap();

        delete_credential(c_h).unwrap();
        assert_eq!(get_credential(c_h).unwrap_err().kind(), VcxErrorKind::InvalidCredentialHandle);

    }

    #[test]
    fn test_get_request_msg() {
        let _setup = SetupMocks::init();

        let connection_h = connection::tests::build_test_connection();

        let offer = _get_offer(connection_h);

        let my_pw_did = ::connection::get_pw_did(connection_h).unwrap();
        let their_pw_did = ::connection::get_their_pw_did(connection_h).unwrap();

        let c_h = credential_create_with_offer("TEST_CREDENTIAL", &offer).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(c_h).unwrap());

        let msg = generate_credential_request_msg(c_h, &my_pw_did, &their_pw_did).unwrap();
        ::serde_json::from_str::<CredentialRequest>(&msg).unwrap();
    }

    #[test]
    fn test_get_credential_offer() {
        let _setup = SetupMocks::init();

        let connection_h = connection::tests::build_test_connection();

        let offer = get_credential_offer_messages(connection_h).unwrap();
        let o: serde_json::Value = serde_json::from_str(&offer).unwrap();
        let _credential_offer: CredentialOffer = serde_json::from_str(&o[0][0].to_string()).unwrap();
    }

    #[test]
    fn test_pay_for_credential_with_sufficient_funds() {
        let _setup = SetupMocks::init();

        let cred = create_credential_with_price(1);
        assert!(cred.is_payment_required());
        let (payment, _receipt): (PaymentTxn, String) = cred.submit_payment().unwrap();
        assert!(payment.amount > 0);
    }

    #[test]
    fn test_pay_for_non_premium_credential() {
        let _setup = SetupMocks::init();

        let cred: Credential = Credential::from_str(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        assert!(cred.payment_info.is_none());
        assert_eq!(cred.submit_payment().unwrap_err().kind(), VcxErrorKind::NoPaymentInformation);
    }

    #[test]
    fn test_pay_for_credential_with_insufficient_funds() {
        let _setup = SetupMocks::init();

        let cred = create_credential_with_price(10000000000);
        assert!(cred.submit_payment().is_err());
    }

    #[test]
    fn test_pay_for_credential_with_handle() {
        let _setup = SetupMocks::init();

        let handle = from_string(DEFAULT_SERIALIZED_CREDENTIAL_PAYMENT_REQUIRED).unwrap();
        submit_payment(handle).unwrap();
        get_payment_information(handle).unwrap();
        let handle2 = from_string(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        assert!(!is_payment_required(handle2).unwrap());
        let invalid_handle = 12345;
        assert_eq!(is_payment_required(invalid_handle).unwrap_err().kind(), VcxErrorKind::InvalidCredentialHandle);
    }

    #[test]
    fn test_get_credential() {
        let _setup = SetupMocks::init();

        let handle = from_string(constants::DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        let _offer_string = get_credential_offer(handle).unwrap();

        let handle = from_string(constants::FULL_CREDENTIAL_SERIALIZED).unwrap();
        let _cred_string = get_credential(handle).unwrap();
    }

    #[test]
    fn test_submit_payment_through_credential_request() {
        let _setup = SetupMocks::init();

        let connection_h = connection::tests::build_test_connection();

        let balance = get_wallet_token_info().unwrap().get_balance();
        assert!(balance > 0);

        let price = 5;
        let mut cred = create_credential_with_price(price);

        assert_eq!(cred.send_request(0).unwrap_err().kind(), VcxErrorKind::InvalidConnectionHandle);
        assert_eq!(get_wallet_token_info().unwrap().get_balance(), balance);

        cred.send_request(connection_h).unwrap();
        assert_eq!(get_wallet_token_info().unwrap().get_balance(), balance); // test mode doesn't change balance?
    }

    #[test]
    fn test_get_cred_offer_returns_json_string_with_cred_offer_json_nested() {
        let _setup = SetupMocks::init();

        let handle = from_string(constants::DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        let offer_string = get_credential_offer(handle).unwrap();
        let offer_value: serde_json::Value = serde_json::from_str(&offer_string).unwrap();

        let _offer_struct: CredentialOffer = serde_json::from_value(offer_value["credential_offer"].clone()).unwrap();
    }

    #[test]
    fn test_accept_credential_offer() {
        let _setup = SetupMocks::init();

        let connection_handle = connection::tests::build_test_connection();
        let offer = _get_offer(connection_handle);

        let (credential_handle, credential_serialized) =
            accept_credential_offer("test", &offer, connection_handle).unwrap();

        assert_eq!(VcxStateType::VcxStateOfferSent as u32, get_state(credential_handle).unwrap());
        assert_eq!(credential_serialized, to_string(credential_handle).unwrap());
    }
}