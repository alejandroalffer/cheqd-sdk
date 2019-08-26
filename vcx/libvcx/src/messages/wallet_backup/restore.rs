use messages::wallet_backup::prepare_message_for_agency_v2;
use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds, parse_message_from_response};
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
        trace!("Restore Backup::send >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<BackupRestored> {
        trace!("restore wallet >>>");

        let response = parse_message_from_response(&response)?;

        serde_json::from_str(&response)
            .map_err(|_| VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of BackupRestored"))
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
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

        let agency_did = self.agent_did.clone().ok_or(init_err("agency_did"))?;
        let agency_vk = self.agent_vk.clone().ok_or(init_err("agency_vk"))?;
        let recovery_vk = self.recovery_vk.clone().ok_or(init_err("recovery_vk"))?;

        prepare_message_for_agency_v2(&message, &agency_did, &agency_vk, &recovery_vk)
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
    use utils::devsetup::tests::{test_wallet, delete_connected_wallets};
    use wallet_backup::tests::{init_backup, TestBackupData, backup_wallet_utils, RECORD_VALUE, RECORD_TYPE, ID, PATH};
    use std::fs::File;
    use utils::libindy::wallet;
    use std::io::Write;

    pub fn restore_wallet_utils(encrypted_wallet: &[u8], wb: &TestBackupData) -> serde_json::Value {
        delete_connected_wallets(&test_wallet());
        ::api::vcx::vcx_shutdown(true);

        let mut ofile = File::create(PATH).unwrap();
        ofile.write_all(encrypted_wallet).unwrap();

        let import_config = json!({
            settings::CONFIG_WALLET_NAME: &test_wallet(),
            settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
            settings::CONFIG_EXPORTED_WALLET_PATH: PATH.to_string(),
            settings::CONFIG_WALLET_BACKUP_KEY: wb.encryption_key.to_string(),
        }).to_string();

        wallet::import(&import_config).unwrap();
        wallet::open_wallet(&test_wallet(), None, None, None).unwrap();

        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": true
        }).to_string();
        let record = wallet::get_record(RECORD_TYPE, ID, &options).unwrap();
        let record: serde_json::Value = serde_json::from_str(&record).unwrap();

        ::std::fs::remove_file(PATH).unwrap();
        delete_connected_wallets(&test_wallet());
        record
    }


    #[test]
    fn test_backup_restore() {
        init!("ledger");

        let (agent_did, agent_vk) = create_and_store_my_did(Some(::utils::constants::MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(::utils::constants::MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(::utils::constants::MY3_SEED)).unwrap();

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
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        let wb = init_backup();

        let err = wallet_backup_restore()
            .recovery_vk(&wb.recovery_vk).unwrap()
            .agent_did(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID).unwrap()).unwrap()
            .agent_vk(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY).unwrap()).unwrap()
            .send_secure();

        assert!( err.unwrap_err().to_string().contains("GNR-121") );
        teardown!("agency");
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_backup_restore_backup_real() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();
        let wb = backup_wallet_utils();

        let backup = wallet_backup_restore()
            .recovery_vk(&wb.recovery_vk).unwrap()
            .agent_did(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID).unwrap()).unwrap()
            .agent_vk(&settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY).unwrap()).unwrap()
            .send_secure().unwrap();

        let encrypted_wallet: &[u8] = &base64::decode(&backup.wallet).unwrap();
        let record = restore_wallet_utils(encrypted_wallet, &wb);

        assert_eq!(&record, &json!({"value":RECORD_VALUE, "type": RECORD_TYPE, "id": ID, "tags": {}}));
        teardown!("agency");
    }
}
