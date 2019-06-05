use settings;
use messages::{A2AMessage, A2AMessageV1, A2AMessageV2, A2AMessageKinds, prepare_message_for_agency, parse_response_from_agency};
use messages::message_type::MessageTypes;
use error::VcxResult;
//use utils::constants::*;
use utils::{httpclient};
//use utils::libindy::{wallet, anoncreds};
use error::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletBackupInitReq {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "fromDID")]
    from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    from_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletBackupInitReqBuilder {
    from_did: String,
    from_vk: String,
}

impl WalletBackupInitReqBuilder {
    pub fn create() -> WalletBackupInitReqBuilder {
        WalletBackupInitReqBuilder {
            from_did: String::new(),
            from_vk: String::new(),
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

    pub fn send_secure(&mut self) -> VcxResult<()> {
        trace!("WalletBackupInitReq::send >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        let message = match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 =>
                A2AMessage::Version1(
                    A2AMessageV1::WalletBackupInitReq(
                        WalletBackupInitReq {
                            msg_type: MessageTypes::build(A2AMessageKinds::WalletBackupInitReq),
                            from_did: self.from_did.clone(),
                            from_vk: self.from_vk.clone(),
                        }
                    )
                ),
            settings::ProtocolTypes::V2 =>
                A2AMessage::Version2(
                    A2AMessageV2::WalletBackupInitReq(
                        WalletBackupInitReq {
                            msg_type: MessageTypes::build(A2AMessageKinds::WalletBackupInitReq),
                            from_did: self.from_did.clone(),
                            from_vk: self.from_vk.clone(),
                        }
                    )
                )
        };

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        println!("message: {:?}", message);
        prepare_message_for_agency(&message, &agency_did)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<()> {
        trace!("parse_get_messages_response >>>");

        let mut response = parse_response_from_agency(&response)?;

        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::WalletBackupInitResp(res)) => Ok(()),
            A2AMessage::Version2(A2AMessageV2::WalletBackupInitResp(res)) => Ok(()),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of WalletBackupInitResp"))
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletBackupInitReqResp {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}



#[cfg(test)]
mod tests {
    use messages::wallet_backup_init_req;

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_wallet_backup_init_req() {
        init!("true");

        let my_did = "Ab8TvZa3Q19VNkQVzAWVL7";
        let my_vk = "5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf";

        let msg = wallet_backup_init_req()
            .from_did(my_did)
            .from_vk(my_vk)
            .prepare_request().unwrap();
        assert!(msg.len() > 0);

    }
}