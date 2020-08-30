use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds, parse_message_from_response, prepare_message_for_agent_v2};
use messages::message_type::{ MessageTypes };
use error::{VcxResult, VcxErrorKind, VcxError};
use utils::httpclient;

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupRestore {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

pub struct BackupRestoreBuilder {
    recovery_vk: Option<String>,
    agent_did: Option<String>,
    agent_vk: Option<String>,
}

impl BackupRestoreBuilder {
    pub fn create() -> BackupRestoreBuilder {
        BackupRestoreBuilder {
            recovery_vk: None,
            agent_did: None,
            agent_vk: None,
        }
    }

    pub fn recovery_vk(&mut self, key: &str) -> VcxResult<&mut Self> {
        self.recovery_vk = Some(key.to_string());
        Ok(self)
    }

    pub fn agent_did(&mut self, did: &str) -> VcxResult<&mut Self> {
        self.agent_did = Some(did.to_string());
        Ok(self)
    }

    pub fn agent_vk(&mut self, vk: &str) -> VcxResult<&mut Self> {
        self.agent_vk = Some(vk.to_string());
        Ok(self)
    }

    pub fn send_secure(&mut self) -> VcxResult<BackupRestored> {
        trace!("BackupRestoreBuilder::send_secure >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<BackupRestored> {
        trace!("BackupRestoreBuilder::parse_response >>>");

        let response = parse_message_from_response(&response)?;

        serde_json::from_str(&response)
            .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidAgencyResponse, "Agency response does not match any variant of BackupRestored"))
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        trace!("BackupRestoreBuilder::prepare_request >>>");

        let init_err = |e: &str| VcxError::from_msg(
            VcxErrorKind::RetrieveExportedWallet,
            format!("BackupRestore expects {} but got None", e)
        );

        let message = A2AMessage::Version2( A2AMessageV2::BackupRestore(
            BackupRestore {
                msg_type: MessageTypes::MessageTypeV2(MessageTypes::build_v2(
                    A2AMessageKinds::BackupRestore,
                )),
            }
        ));

        trace!("BackupRestoreBuilder::prepare_request >>> message: {:?}", secret!(message));

        let agency_did = self.agent_did.clone().ok_or(init_err("agency_did"))?;
        let agency_vk = self.agent_vk.clone().ok_or(init_err("agency_vk"))?;
        let recovery_vk = self.recovery_vk.clone().ok_or(init_err("recovery_vk"))?;

        prepare_message_for_agent_v2(vec![message], &recovery_vk, &agency_did, &agency_vk)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupRestored {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    pub wallet: String,
}

#[cfg(feature = "wallet_backup")]
#[cfg(test)]
mod tests {
    use settings;
    use messages::wallet_backup_restore;
    use utils::libindy::signus::create_and_store_my_did;
    use wallet_backup::tests::{init_backup, TestBackupData, backup_wallet_utils, RECORD_VALUE, RECORD_TYPE, ID, PATH};
    use std::fs::File;
    use utils::libindy::wallet;
    use std::io::Write;
    use utils::devsetup::{SetupLibraryWalletPoolZeroFees, SetupLibraryAgencyV1, SetupDefaults};
    use utils::libindy::wallet::delete_wallet;

    pub fn restore_wallet_utils(encrypted_wallet: &[u8], wb: &TestBackupData) -> serde_json::Value {
        let wallet_name = "restored_wallet";

        let mut ofile = File::create(PATH).unwrap();
        ofile.write_all(encrypted_wallet).unwrap();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: &wallet_name,
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: PATH.to_string(),
            settings::CONFIG_WALLET_BACKUP_KEY: wb.encryption_key.to_string(),
        }).to_string();

        wallet::import(&import_config).unwrap();
        wallet::open_wallet(&wallet_name, None, None, None).unwrap();

        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": true
        }).to_string();
        let record = wallet::get_record(RECORD_TYPE, ID, &options).unwrap();
        let record: serde_json::Value = serde_json::from_str(&record).unwrap();

        ::std::fs::remove_file(PATH).unwrap();
        delete_wallet(&wallet_name, None, None, None).ok();
        record
    }


    #[test]
    fn test_backup_restore() {
        let _setup = SetupLibraryWalletPoolZeroFees::init();

        let (agent_did, agent_vk) = create_and_store_my_did(Some(::utils::constants::MY2_SEED), None).unwrap();
        let (_, my_vk) = create_and_store_my_did(Some(::utils::constants::MY1_SEED), None).unwrap();
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = wallet_backup_restore()
            .recovery_vk(&my_vk).unwrap()
            .agent_did(&agent_did).unwrap()
            .agent_vk(&agent_vk).unwrap()
            .prepare_request()
            .unwrap();
        assert!(msg.len() > 0);

    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_backup_restore_no_backup_real() {
        let _setup = SetupLibraryAgencyV1::init();

        ::utils::devsetup::set_consumer();

        let wb = init_backup();

        let err = wallet_backup_restore()
            .recovery_vk(&wb.recovery_vk).unwrap()
            .agent_did(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID).unwrap()).unwrap()
            .agent_vk(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY).unwrap()).unwrap()
            .send_secure();

        assert!( err.unwrap_err().to_string().contains("GNR-111") );
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_backup_restore_backup_real() {
        let wb;
        let encrypted_wallet;
        {
            let _setup = SetupLibraryAgencyV1::init();

            ::utils::devsetup::set_consumer();
            wb = backup_wallet_utils();

            let backup = wallet_backup_restore()
                .recovery_vk(&wb.recovery_vk).unwrap()
                .agent_did(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID).unwrap()).unwrap()
                .agent_vk(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY).unwrap()).unwrap()
                .send_secure().unwrap();

            encrypted_wallet = base64::decode(&backup.wallet).unwrap();
        }

        let _setup = SetupDefaults::init();

        let record = restore_wallet_utils(&encrypted_wallet, &wb);

        assert_eq!(&record, &json!({"value":RECORD_VALUE, "type": RECORD_TYPE, "id": ID, "tags": {}}));
    }
}
