use messages::update_message::{UIDsByConn, update_messages as update_messages_status};
use messages::MessageStatusCode;
use messages::get_message::{Message, get_connection_messages};
use messages::update_connection::send_delete_connection_message;

use v3::messages::connection::did_doc::DidDoc;
use v3::messages::a2a::A2AMessage;
use v3::utils::encryption_envelope::EncryptionEnvelope;

use std::collections::HashMap;

use connection::create_agent_keys;
use utils::httpclient;
use utils::libindy::signus::create_and_store_my_did;
use settings;
use error::prelude::*;
use settings::ProtocolTypes;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AgentInfo {
    pub pw_did: String,
    pub pw_vk: String,
    pub agent_did: String,
    pub agent_vk: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_remote_agent_responses: Option<bool>,
}

impl Default for AgentInfo {
    fn default() -> AgentInfo {
        AgentInfo {
            pw_did: String::new(),
            pw_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
            wait_remote_agent_responses: None,
        }
    }
}

impl AgentInfo {
    pub fn create_agent(&self, wait_remote_agent_responses: &Option<bool>) -> VcxResult<AgentInfo> {
        trace!("Agent::create_agent >>>");
        debug!("Agent: creating pairwise agent for connection");

        let method_name = settings::get_config_value(settings::CONFIG_DID_METHOD).ok();
        let (pw_did, pw_vk) = create_and_store_my_did(None, method_name.as_ref().map(String::as_str))?;

        /*
            Create User Pairwise Agent in old way.
            Send Messages corresponding to V2 Protocol to avoid code changes on Agency side.
        */
        let (agent_did, agent_vk) = create_agent_keys("", &pw_did, &pw_vk)?;

        let agent = AgentInfo {
            pw_did,
            pw_vk,
            agent_did,
            agent_vk,
            wait_remote_agent_responses: wait_remote_agent_responses.clone(),
        };

        trace!("Agent::create_agent <<< pairwise_agent: {:?}", secret!(agent));
        Ok(agent)
    }

    pub fn agency_endpoint(&self) -> VcxResult<String> {
        trace!("Agent::agency_endpoint >>>");
        debug!("Agent: Getting Agency endpoint");

        settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)
            .map(|str| format!("{}/agency/msg", str))
    }

    pub fn routing_keys(&self) -> VcxResult<Vec<String>> {
        trace!("Agent::routing_keys >>>");
        debug!("Agent: Getting routing keys");

        let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;
        Ok(vec![self.agent_vk.to_string(), agency_vk])
    }

    pub fn recipient_keys(&self) -> Vec<String> {
        trace!("Agent::recipient_keys >>>");
        debug!("Agent: Getting recipient keys");

        vec![self.pw_vk.to_string()]
    }

    pub fn update_message_status(&self, uid: String) -> VcxResult<()> {
        trace!("Agent::update_message_status_as_reviewed >>> uid: {:?}", uid);
        debug!("Agent: Updating message {:?} status on reviewed", uid);

        let messages_to_update = vec![UIDsByConn {
            pairwise_did: self.pw_did.clone(),
            uids: vec![uid],
        }];

        update_messages_status(MessageStatusCode::Reviewed, messages_to_update)?;

        trace!("Agent::update_message_status_as_reviewed <<<");
        Ok(())
    }

    pub fn get_messages(&self) -> VcxResult<HashMap<String, A2AMessage>> {
        trace!("Agent::get_messages >>>");
        debug!("Agent: Getting all received messages from the agent");

        let messages = get_connection_messages(&self.pw_did,
                                               &self.pw_vk,
                                               &self.agent_did,
                                               &self.agent_vk,
                                               None,
                                               Some(vec![MessageStatusCode::Received]),
                                               &Some(ProtocolTypes::V2))?;


        let mut a2a_messages: HashMap<String, A2AMessage> = HashMap::new();

        for message in messages {
            a2a_messages.insert(message.uid.clone(), Self::decode_message(&message)?);
        }

        trace!("Agent::get_messages <<< a2a_messages: {:?}", secret!(a2a_messages));
        Ok(a2a_messages)
    }

    pub fn get_message_by_id(&self, msg_id: &str) -> VcxResult<A2AMessage> {
        trace!("Agent::get_message_by_id >>> msg_id: {:?}", msg_id);
        debug!("Agent: Getting message by id {}", msg_id);

        let mut messages = get_connection_messages(&self.pw_did,
                                                   &self.pw_vk,
                                                   &self.agent_did,
                                                   &self.agent_vk,
                                                   Some(vec![msg_id.to_string()]),
                                                   None,
                                                   &Some(ProtocolTypes::V2))?;

        let message =
            messages
                .pop()
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, format!("Message not found for id: {:?}", msg_id)))?;

        let message = Self::decode_message(&message)?;

        trace!("Agent::get_message_by_id <<< message: {:?}", secret!(message));
        Ok(message)
    }

    pub fn decode_message(message: &Message) -> VcxResult<A2AMessage> {
        trace!("Agent::decode_message >>> message: {:?}", secret!(message));
        debug!("Agent: Decoding received message");

        let message = match message.decrypted_payload {
            Some(ref payload) => {
                debug!("Agent: Message Payload is already decoded");

                let message: ::messages::payload::PayloadV1 = ::serde_json::from_str(&payload)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, format!("Cannot deserialize message: {}", err)))?;

                ::serde_json::from_str::<A2AMessage>(&message.msg)
                    .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, format!("Cannot deserialize A2A message: {}", err)))
            }
            None => EncryptionEnvelope::open(message.payload()?)
        }?;

        trace!("Agent::decode_message <<< message: {:?}", secret!(message));
        Ok(message)
    }

    pub fn send_message(&self, message: &A2AMessage, did_doc: &DidDoc) -> VcxResult<()> {
        trace!("Agent::send_message >>> message: {:?}, did_doc: {:?}", secret!(message), secret!(did_doc));
        debug!("Agent: Sending message on the remote endpoint");

        let pw_key = if self.pw_vk.is_empty() { None } else { Some(self.pw_vk.clone()) };
        let envelope = EncryptionEnvelope::create(&message, pw_key.as_ref().map(String::as_str), &did_doc)?;
        Self::_send_message(&envelope, &did_doc, self.wait_remote_agent_responses.unwrap_or(true))?;
        trace!("Agent::send_message <<<");
        Ok(())
    }

    pub fn send_message_anonymously(message: &A2AMessage, did_dod: &DidDoc) -> VcxResult<()> {
        trace!("Agent::send_message_anonymously >>> message: {:?}, did_doc: {:?}", secret!(message), secret!(did_dod));
        debug!("Agent: Sending message on the remote anonymous endpoint");

        let envelope = EncryptionEnvelope::create(&message, None, &did_dod)?;
        Self::_send_message(&envelope, &did_dod, false)?;
        trace!("Agent::send_message_anonymously <<<");
        Ok(())
    }

    fn _send_message(envelope: &EncryptionEnvelope, did_doc: &DidDoc, wait_remote_agent_responses: bool) -> VcxResult<Vec<u8>> {
        if wait_remote_agent_responses {
            httpclient::post_message(&envelope.0, &did_doc.get_endpoint())
        } else {
            httpclient::post_message_async(&envelope.0, &did_doc.get_endpoint());
            Ok(Vec::new())
        }
    }

    pub fn delete(&self) -> VcxResult<()> {
        trace!("Agent::delete >>>");
        debug!("Agent: deleting");

        send_delete_connection_message(&self.pw_did, &self.pw_vk, &self.agent_did, &self.agent_vk)?;
        trace!("Agent::delete <<<");
        Ok(())
    }
}