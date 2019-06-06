use settings;
use messages::{A2AMessage, A2AMessageV1, A2AMessageV2, A2AMessageKinds, prepare_message_for_agency, parse_response_from_agency};
use messages::message_type::MessageTypes;
use error::VcxResult;
use utils::httpclient;
use error::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "fromDID")]
    from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    from_vk: String,
    wallet_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupBuilder {
    from_did: String,
    from_vk: String,
    wallet_data: Vec<u8>,
}

impl BackupBuilder {
    pub fn create() -> BackupBuilder {
        BackupBuilder {
            from_did: String::new(),
            from_vk: String::new(),
            wallet_data: Vec::new(),
        }
    }

    pub fn from_did(&mut self, did: &str) -> &mut Self {
        self.from_did = did.to_string();
        self
    }

    pub fn from_vk(&mut self, did: &str) -> &mut Self {
        self.from_vk = did.to_string();
        self
    }

    pub fn wallet_data(&mut self, wallet_data: Vec<u8>) -> &mut Self {
        self.wallet_data = wallet_data;
        self
    }

    pub fn send_secure(&mut self) -> VcxResult<()> {
        trace!("WalletBackup::send >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        let message = match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 =>
                A2AMessage::Version1(
                    A2AMessageV1::Backup(
                        Backup {
                            msg_type: MessageTypes::build(A2AMessageKinds::Backup),
                            from_did: self.from_did.clone(),
                            from_vk: self.from_vk.clone(),
                            wallet_data: self.wallet_data.clone(),
                        }
                    )
                ),
            settings::ProtocolTypes::V2 =>
                A2AMessage::Version2(
                    A2AMessageV2::Backup(
                        Backup {
                            msg_type: MessageTypes::build(A2AMessageKinds::Backup),
                            from_did: self.from_did.clone(),
                            from_vk: self.from_vk.clone(),
                            wallet_data: self.wallet_data.clone(),
                        }
                    )
                )
        };

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        prepare_message_for_agency(&message, &agency_did)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<()> {
        trace!("parse_get_messages_response >>>");

        let mut response = parse_response_from_agency(&response)?;

        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::Backup(res)) => Ok(()),
            A2AMessage::Version2(A2AMessageV2::Backup(res)) => Ok(()),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of WalletBackupProvision"))
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupResp {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupAck {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}



#[cfg(test)]
mod tests {
    use super::*;
    use messages::backup_wallet;
    use settings::{CONFIG_PROTOCOL_TYPE, CONFIG_REMOTE_TO_SDK_DID, CONFIG_REMOTE_TO_SDK_VERKEY};
    use utils::libindy::signus::create_and_store_my_did;
    use utils::constants::{MY1_SEED, MY2_SEED, MY3_SEED};

    #[test]
    fn test_wallet_backup() {
        init!("false");

        let (user_did, user_vk) = create_and_store_my_did(None).unwrap();
        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        settings::set_config_value(CONFIG_PROTOCOL_TYPE, &settings::ProtocolTypes::V2.to_string());

        let msg = backup_wallet()
            .from_did(&settings::get_config_value(CONFIG_REMOTE_TO_SDK_DID).unwrap())
            .from_vk(&settings::get_config_value(CONFIG_REMOTE_TO_SDK_VERKEY).unwrap())
            .wallet_data(vec![1, 2, 3])
            .prepare_request().unwrap();
        assert!(msg.len() > 0);

    }
}