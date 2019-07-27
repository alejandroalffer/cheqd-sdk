use settings;
use messages::wallet_backup::{prepare_message_for_agency_v2};
use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds};
use messages::message_type::{ MessageTypes };
use error::{VcxResult, VcxErrorKind, VcxError};
use utils::httpclient;

#[derive(Serialize, Deserialize, Debug)]
pub struct RetrieveDeadDrop {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "recoveryVk")]
    recovery_vk: String,
    #[serde(rename = "deadDropAddress")]
    dead_drop_address: String,
    locator: String,
    signature: String,
}

pub struct RetrieveDeadDropBuilder {
    recovery_vk: Option<String>,
    dead_drop_address: Option<String>,
    locator: Option<String>,
    signature: Option<String>,
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

    pub fn signature(&mut self, sig: &str) -> VcxResult<&mut Self> {
        self.signature = Some(sig.to_string());
        Ok(self)
    }

    pub fn send_secure(&mut self) -> VcxResult<()> {
        trace!("DeadDropRetrieve::send >>>");

        let data = self.prepare_request()?;

        // Agency is no longer sending Specific Response - 200 is sufficient
        httpclient::post_u8(&data)?;

        Ok(())
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
                dead_drop_address: self.dead_drop_address.clone().ok_or(init_err("dead_drop_address"))?,
                locator: self.locator.clone().ok_or(init_err("locator"))?,
                signature: self.signature.clone().ok_or(init_err("signature"))?,
            }
        ));

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        prepare_message_for_agency_v2(&message, &agency_did)
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct DeadDropRetrievedEntry {
    address: String,
    data: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RetrievedDeadDropResult {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    entry: Option<DeadDropRetrievedEntry>
}

#[cfg(feature = "wallet_backup")]
#[cfg(test)]
mod tests {
    use super::*;
    use messages::{retrieve_dead_drop};
//    use messages::{wallet_backup_init, RemoteMessageType, retrieve_dead_drop};
//    use std::thread;
//    use std::time::Duration;
    use utils::libindy::signus::create_and_store_my_did;
//    use messages::wallet_backup::received_expected_message;

    #[test]
    fn test_dead_drop_retrieve() {
        init!("ledger");
        let (user_did, user_vk) = create_and_store_my_did(None).unwrap();
        let (agent_did, agent_vk) = create_and_store_my_did(Some(::utils::constants::MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(::utils::constants::MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(::utils::constants::MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = retrieve_dead_drop()
            .recovery_vk(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .dead_drop_address(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .signature(&settings::CONFIG_REMOTE_TO_SDK_DID).unwrap()
            .locator(&settings::CONFIG_REMOTE_TO_SDK_DID).unwrap()
            .prepare_request()
            .unwrap();
        assert!(msg.len() > 0);

    }

//    #[cfg(feature = "agency")]
//    #[cfg(feature = "pool_tests")]
//    #[test]
//    fn test_backup_provision_real() {
//        init!("agency");
//        ::utils::devsetup::tests::set_consumer();
//
//        assert!(retrieve_dead_drop()
//            .recovery_vk(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
//            .dead_drop_address(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
//            .cloud_address(&settings::CONFIG_REMOTE_TO_SDK_DID.as_bytes().to_vec()).unwrap()
//            .send_secure().is_ok());
//        teardown!("agency")
//    }
//
//    #[cfg(feature = "agency")]
//    #[cfg(feature = "pool_tests")]
//    #[test]
//    fn test_received_provisioned_response_true() {
//        init!("agency");
//        ::utils::devsetup::tests::set_consumer();
//
//        wallet_backup_init()
//            .recovery_vk(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
//            .dead_drop_address(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
//            .cloud_address(&settings::CONFIG_REMOTE_TO_SDK_DID.as_bytes().to_vec()).unwrap()
//            .send_secure().unwrap();
//        thread::sleep(Duration::from_millis(2000));
//
//        assert_eq!(received_expected_message(None, RemoteMessageType::WalletBackupProvisioned).unwrap(), true);
//        teardown!("agency")
//    }
//
//    #[cfg(feature = "agency")]
//    #[cfg(feature = "pool_tests")]
//    #[test]
//    fn test_received_provisioned_response_false() {
//        init!("agency");
//        ::utils::devsetup::tests::set_consumer();
//
//        assert_eq!(received_expected_message(None, RemoteMessageType::WalletBackupProvisioned).unwrap(), false);
//        teardown!("agency")
//    }
}