pub mod backup_init;
pub mod backup;
pub mod restore;

use settings;
use messages::get_message::{Message, download_agent_messages};
use error::prelude::*;
use messages::{A2AMessage, prepare_forward_message_for_agency_v2, ForwardV2, A2AMessageKinds, RemoteMessageType, Bundled};
use utils::libindy::crypto;
use messages::message_type::MessageTypes;

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
