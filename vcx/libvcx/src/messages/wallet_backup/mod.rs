pub mod backup_provision;
pub mod backup;

use settings;
use messages;
use messages::get_message::Message;
use settings::get_config_value;
use error::prelude::*;
use messages::{GeneralMessage, A2AMessage, prepare_forward_message_for_agency_v2, ForwardV2, A2AMessageKinds};
use utils::libindy::crypto;
use messages::message_type::MessageTypes;

//Todo: this could be refactored ad put in Utils so that all code (not just wallet backup) could
// retrieve messages from the UserAgent
pub fn get_wallet_backup_messages() -> VcxResult<Vec<Message>> {
    messages::get_messages()
        .to(&get_config_value(settings::CONFIG_SDK_TO_REMOTE_DID)?)?
        .to_vk(&get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?)?
        .agent_did(&get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?)?
        .agent_vk(&get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?)?
        .send_secure()
}

//Todo: This is meant only as a short term solution. Wallet backup is the only protocol using V2 as of 06/2019
// Eventually, WalletBackup can use the general prepare_message_for_agency
// This is all Duplicate code that can be found in messages/mod.rs -> 'pack_for_agency_v2' and 'prepare_forward_message'
pub fn prepare_message_for_agency_v2(message: &A2AMessage, agency_did: &str) -> VcxResult<Vec<u8>> {

    let message = _pack_for_agency_v2_without_fwd(message, agency_did)?;
    _prepare_fwd_v2(message, agency_did)
}

fn _pack_for_agency_v2_without_fwd(message: &A2AMessage, agency_did: &str) -> VcxResult<Vec<u8>> {
    let agent_vk = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?;
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    let message = ::serde_json::to_string(&message)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::SerializationError, format!("Cannot serialize A2A message: {}", err)))?;

    let receiver_keys = ::serde_json::to_string(&vec![&agent_vk])
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
