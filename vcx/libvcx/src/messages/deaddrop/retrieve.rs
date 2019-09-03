use settings;
use messages::wallet_backup::prepare_message_for_agency_v2;
use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds, parse_message_from_response};
use messages::message_type::{ MessageTypes };
use error::{VcxResult, VcxErrorKind, VcxError};
use utils::httpclient;

#[derive(Serialize, Deserialize, Debug)]
pub struct RetrieveDeadDrop {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "recoveryVerKey")]
    recovery_vk: String,
    address: String,
    locator: String,
    #[serde(rename = "locatorSignature")]
    signature: Vec<u8>,
}

pub struct RetrieveDeadDropBuilder {
    recovery_vk: Option<String>,
    dead_drop_address: Option<String>,
    locator: Option<String>,
    signature: Option<Vec<u8>>,
}

impl RetrieveDeadDropBuilder {
    pub fn create() -> RetrieveDeadDropBuilder {
        RetrieveDeadDropBuilder {
            recovery_vk: None,
            dead_drop_address: None,
            locator: None,
            signature: None,
        }
    }

    pub fn recovery_vk(&mut self, key: &str) -> VcxResult<&mut Self> {
        self.recovery_vk = Some(key.to_string());
        Ok(self)
    }

    pub fn dead_drop_address(&mut self, address: &str) -> VcxResult<&mut Self> {
        self.dead_drop_address = Some(address.to_string());
        Ok(self)
    }

    pub fn locator(&mut self, locator: &str) -> VcxResult<&mut Self> {
        self.locator = Some(locator.to_string());
        Ok(self)
    }

    pub fn signature(&mut self, sig: &[u8]) -> VcxResult<&mut Self> {
        self.signature = Some(sig.to_vec());
        Ok(self)
    }

    pub fn send_secure(&mut self) -> VcxResult<RetrievedDeadDropResult> {
        trace!("DeadDropRetrieve::send >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<RetrievedDeadDropResult> {
        trace!("parse_retrieve_deaddrop_response >>>");

        let response = parse_message_from_response(&response)?;

        serde_json::from_str(&response)
            .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of DeadDropRetrievedResult"))
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        let init_err = |e: &str| VcxError::from_msg(
            VcxErrorKind::RetrieveDeadDrop,
            format!("RetrieveDeadDrop expects {} but got None", e)
        );

        let message = A2AMessage::Version2( A2AMessageV2::RetrieveDeadDrop(
            RetrieveDeadDrop {
                msg_type: MessageTypes::MessageTypeV2(MessageTypes::build_v2(
                    A2AMessageKinds::RetrieveDeadDrop,
                )),
                recovery_vk: self.recovery_vk.clone().ok_or(init_err("recovery_key"))?,
                address: self.dead_drop_address.clone().ok_or(init_err("dead_drop_address"))?,
                locator: self.locator.clone().ok_or(init_err("locator"))?,
                signature: self.signature.clone().ok_or(init_err("signature"))?,
            }
        ));

        let agency_did = settings::get_config_value(settings::CONFIG_AGENCY_DID)?;
        let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;
        let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

        prepare_message_for_agency_v2(&message, &agency_did, &agency_vk, &my_vk)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeadDropRetrievedEntry {
    pub address: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RetrievedDeadDropResult {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    pub entry: Option<DeadDropRetrievedEntry>
}

#[cfg(feature = "wallet_backup")]
#[cfg(test)]
mod tests {
    use super::*;
    use messages::{retrieve_dead_drop};
    use wallet_backup::tests::init_backup;
    use utils::libindy::signus::create_and_store_my_did;
    use wallet_backup::WalletBackup;
    use utils::libindy::crypto::sign;
    use rand::Rng;

    #[cfg(feature = "wallet_backup")]
    #[test]
    fn test_dead_drop_retrieve() {
        init!("ledger");
        let (agent_did, agent_vk) = create_and_store_my_did(Some(::utils::constants::MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(::utils::constants::MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(::utils::constants::MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = retrieve_dead_drop()
            .recovery_vk(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .dead_drop_address(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .signature(&settings::CONFIG_REMOTE_TO_SDK_DID.as_bytes()).unwrap()
            .locator(&settings::CONFIG_REMOTE_TO_SDK_DID).unwrap()
            .prepare_request()
            .unwrap();
        assert!(msg.len() > 0);

    }

    #[cfg(feature = "wallet_backup")]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_retrieve_dead_drop_real() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        let wb = init_backup();

        assert!(retrieve_dead_drop()
            .recovery_vk(&wb.recovery_vk).unwrap()
            .dead_drop_address(&wb.dd_address).unwrap()
            .locator(&wb.locator).unwrap()
            .signature(&wb.sig).unwrap()
            .send_secure().is_ok());
        teardown!("agency");
    }

    #[cfg(feature = "wallet_backup")]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_retrieved_dead_drop_result_real() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        let wb = init_backup();

        let dead_drop_result = retrieve_dead_drop()
            .recovery_vk(&wb.recovery_vk).unwrap()
            .dead_drop_address(&wb.dd_address).unwrap()
            .locator(&wb.locator).unwrap()
            .signature(&wb.sig).unwrap()
            .send_secure().unwrap();

        let entry = dead_drop_result.entry.unwrap();
        assert_eq!(entry.address, wb.dd_address.clone());
        teardown!("agency");
    }

    #[cfg(feature = "wallet_backup")]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_retrieved_dead_drop_fails_with_invalid_address() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        let wb = init_backup();

        let err = retrieve_dead_drop()
            .recovery_vk(&wb.recovery_vk).unwrap()
            .dead_drop_address(&(wb.dd_address + "B")).unwrap()
            .locator(&wb.locator).unwrap()
            .signature(&wb.sig).unwrap()
            .send_secure();

        assert_eq!(
            err.unwrap_err().to_string(),
            "Error: Message failed in post\n  Caused by: POST failed with: {\"detail\":\"java.lang.RuntimeException: invalid address\",\"statusCode\":\"GNR-105\",\"statusMsg\":\"unhandled error\"}\n"
        );
        teardown!("agency");
    }

    #[cfg(feature = "wallet_backup")]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_retrieve_dd_with_none_stored_returns_none() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        let backup_key = rand::thread_rng()
            .gen_ascii_chars()
            .take(44)
            .collect::<String>();

        let wb = WalletBackup::create("a123", &backup_key).unwrap();
        let sig = sign(&wb.keys.recovery_vk, wb.keys.dead_drop_address.locator.as_bytes()).unwrap();

        let empty_backup = retrieve_dead_drop()
            .recovery_vk(&wb.keys.recovery_vk).unwrap()
            .dead_drop_address(&wb.keys.dead_drop_address.address).unwrap()
            .locator(&wb.keys.dead_drop_address.locator).unwrap()
            .signature(&sig).unwrap()
            .send_secure().unwrap();

        assert!(empty_backup.entry.is_none());
        teardown!("agency");
    }
}