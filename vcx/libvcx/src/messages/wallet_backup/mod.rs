pub mod backup_init;
pub mod backup;
pub mod restore;

use settings;
use messages::get_message::{Message, download_agent_messages};
use error::prelude::*;
use messages::{A2AMessage, prepare_forward_message_for_agency_v2, ForwardV2, A2AMessageKinds, RemoteMessageType};
use utils::libindy::crypto;
use messages::message_type::MessageTypes;

//Todo: This is meant only as a short term solution. Wallet backup is the only protocol using V2 as of 06/2019
// Eventually, WalletBackup can use the general prepare_message_for_agency
// This is all Duplicate code that can be found in messages/mod.rs -> 'pack_for_agency_v2' and 'prepare_forward_message'
pub fn prepare_message_for_agency_v2(message: &A2AMessage, agency_did: &str, agency_vk: &str, my_vk: &str) -> VcxResult<Vec<u8>> {
    if settings::test_indy_mode_enabled() { return Ok(Vec::new()) }

    let message = _pack_for_agency_v2_without_fwd(message, agency_did, agency_vk, my_vk)?;
    _prepare_fwd_v2(message, agency_did)
}

fn _pack_for_agency_v2_without_fwd(message: &A2AMessage, agency_did: &str, agency_vk: &str, my_vk: &str) -> VcxResult<Vec<u8>> {
    let message = ::serde_json::to_string(&message)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::SerializationError, format!("Cannot serialize A2A message: {}", err)))?;

    let receiver_keys = ::serde_json::to_string(&vec![&agency_vk])
        .map_err(|err| VcxError::from_msg(VcxErrorKind::SerializationError, format!("Cannot serialize receiver keys: {}", err)))?;

    crypto::pack_message(Some(&my_vk), &receiver_keys, message.as_bytes())
}

fn _prepare_fwd_v2(message: Vec<u8>, did: &str) -> VcxResult<Vec<u8>> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;

    let msg = serde_json::from_slice(message.as_slice())
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidState, err))?;

    let message= ForwardV2 {
        msg_type: MessageTypes::build_v2(A2AMessageKinds::Forward),
        fwd: did.to_string(),
        msg,
    };

    prepare_forward_message_for_agency_v2(&message, &agency_vk)
}

pub fn received_expected_message(message: Option<Message>, expected_type: RemoteMessageType) -> VcxResult<bool> {
    // Todo: If multiple responses have the same type, how to know which one corresponds to the request?? MSG_ID??
    if let Some(msg) = message {
        if msg.msg_type == expected_type { return Ok(true) };
    } else {
        let messages = download_agent_messages(None, None)?;
        for msg in messages.iter() {
            // Todo: This will return ok if it finds any matching type... FIX
            if msg.msg_type == expected_type { return Ok(true) };
        }
    }
    Ok(false)
}
