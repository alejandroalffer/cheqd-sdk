use settings;
use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds, prepare_message_for_agency, parse_response_from_agency};
use messages::message_type::MessageTypes;
use error::VcxResult;
use utils::httpclient;
use error::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupProvision {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

pub struct BackupProvisionBuilder { }

impl BackupProvisionBuilder {
    pub fn create() -> BackupProvisionBuilder {
        BackupProvisionBuilder {}
    }

    pub fn send_secure(&mut self) -> VcxResult<()> {
        trace!("WalletBackupProvision::send >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        let message = match settings::get_protocol_type() {
            settings::ProtocolTypes::V2 =>
                A2AMessage::Version2(
                    A2AMessageV2::BackupProvision(
                        BackupProvision {
                            msg_type: MessageTypes::build(A2AMessageKinds::BackupProvision),
                        }
                    )
                ),
            _ => return Err(VcxError::from(VcxErrorKind::InvalidMsgVersion))
        };

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        prepare_message_for_agency(&message, &agency_did)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<()> {
        trace!("parse_get_messages_response >>>");

        let mut response = parse_response_from_agency(&response)?;

        match response.remove(0) {
            A2AMessage::Version2(A2AMessageV2::BackupProvision(res)) => Ok(()),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of WalletBackupProvision"))
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupProvisionResp {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupProvisioned {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}



#[cfg(test)]
mod tests {
    use super::*;
    use messages::wallet_backup_provision;
    use settings::{CONFIG_PROTOCOL_TYPE, CONFIG_PROTOCOL_VERSION};
    use utils::libindy::signus::create_and_store_my_did;
    use utils::constants::{MY1_SEED, MY2_SEED, MY3_SEED};

    #[test]
    fn test_wallet_backup_provision() {
        init!("false");

        let (user_did, user_vk) = create_and_store_my_did(None).unwrap();
        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        settings::set_config_value(CONFIG_PROTOCOL_TYPE, &settings::ProtocolTypes::V2.to_string());

        let msg = wallet_backup_provision()
            .prepare_request().unwrap();
        assert!(msg.len() > 0);

    }

    #[test]
    fn test_wallet_backup_provision_not_supported_for_version_1() {
        init!("false");

        settings::set_config_value(CONFIG_PROTOCOL_TYPE, &settings::ProtocolTypes::V1.to_string());
        settings::set_config_value(CONFIG_PROTOCOL_VERSION, "1.0");

        assert_eq!(wallet_backup_provision().prepare_request().unwrap_err().kind(), VcxErrorKind::InvalidMsgVersion);
    }
}