use settings;
use messages::wallet_backup::{prepare_message_for_agency_v2};
use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds};
use messages::message_type::{ MessageTypes };
use error::{VcxResult, VcxErrorKind, VcxError};
use utils::httpclient;

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupInitParams {
    #[serde(rename = "recoveryVk")]
    recovery_vk: String,
    #[serde(rename = "ddAddress")]
    dead_drop_address: String,
    #[serde(rename = "cloudAddress")]
    cloud_address: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupInit {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    params: BackupInitParams,
}

pub struct BackupInitBuilder {
    recovery_vk: Option<String>,
    dead_drop_address: Option<String>,
    cloud_address: Option<Vec<u8>>,
}

impl BackupInitBuilder {
    pub fn create() -> BackupInitBuilder {
        BackupInitBuilder {
            recovery_vk: None,
            dead_drop_address: None,
            cloud_address: None,
        }
    }

    pub fn recovery_vk(&mut self, key: &str) -> VcxResult<&mut Self> {
        self.recovery_vk = Some(key.to_string());
        Ok(self)
    }

    pub fn dead_drop_address(&mut self, address: &str) -> VcxResult<&mut Self> {
        // Todo: Hash(vk + hash(namespace))
        self.dead_drop_address = Some(address.to_string());
        Ok(self)
    }

    pub fn cloud_address(&mut self, address: &Vec<u8>) -> VcxResult<&mut Self> {
        self.cloud_address = Some(address.clone());
        Ok(self)
    }


    pub fn send_secure(&mut self) -> VcxResult<()> {
        trace!("WalletBackupInit::send >>>");

        let data = self.prepare_request()?;

        // Agency is no longer sending Specific Response - 200 is sufficient
        httpclient::post_u8(&data)?;

        Ok(())
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        let init_err = |e: &str| VcxError::from_msg(
            VcxErrorKind::CreateWalletBackup,
            format!("BackupInit expects {} but got None", e)
        );

        let params = BackupInitParams {
            recovery_vk: self.recovery_vk.clone().ok_or(init_err("recovery_key"))?,
            dead_drop_address: self.dead_drop_address.clone().ok_or(init_err("dead_drop_address"))?,
            cloud_address: self.cloud_address.clone().ok_or(init_err("cloud_address"))?,
        };
        let message = A2AMessage::Version2( A2AMessageV2::BackupProvision(
            BackupInit {
                msg_type: MessageTypes::MessageTypeV2(MessageTypes::build_v2(
                    A2AMessageKinds::BackupInit,
                )),
                params
            }
        ));

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
        let agency_vk = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?;
        let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

        prepare_message_for_agency_v2(&message, &agency_did, &agency_vk, &my_vk)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupProvisioned {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

#[cfg(feature = "wallet_backup")]
#[cfg(test)]
mod tests {
    use super::*;
    use messages::{wallet_backup_init, RemoteMessageType};
    use std::thread;
    use std::time::Duration;
    use utils::libindy::signus::create_and_store_my_did;
    use messages::wallet_backup::received_expected_message;

    #[cfg(feature = "wallet_backup")]
    #[test]
    fn test_wallet_backup_provision() {
        init!("ledger_zero_fees");
        let (user_did, user_vk) = create_and_store_my_did(None).unwrap();
        let (agent_did, agent_vk) = create_and_store_my_did(Some(::utils::constants::MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(::utils::constants::MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(::utils::constants::MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = wallet_backup_init()
            .recovery_vk(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .dead_drop_address(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .cloud_address(&settings::CONFIG_REMOTE_TO_SDK_DID.as_bytes().to_vec()).unwrap()
            .prepare_request()
            .unwrap();
        assert!(msg.len() > 0);

    }

    #[cfg(feature = "wallet_backup")]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_backup_provision_real() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        assert!(wallet_backup_init()
            .recovery_vk(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .dead_drop_address(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .cloud_address(&settings::CONFIG_REMOTE_TO_SDK_DID.as_bytes().to_vec()).unwrap()
            .send_secure().is_ok());
        teardown!("agency")
    }

    #[cfg(feature = "wallet_backup")]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_received_provisioned_response_true() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        wallet_backup_init()
            .recovery_vk(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .dead_drop_address(settings::CONFIG_WALLET_BACKUP_KEY).unwrap()
            .cloud_address(&settings::CONFIG_REMOTE_TO_SDK_DID.as_bytes().to_vec()).unwrap()
            .send_secure().unwrap();
        thread::sleep(Duration::from_millis(2000));

        assert_eq!(received_expected_message(None, RemoteMessageType::WalletBackupProvisioned).unwrap(), true);
        teardown!("agency")
    }

    #[cfg(feature = "wallet_backup")]
    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_received_provisioned_response_false() {
        init!("agency");
        ::utils::devsetup::tests::set_consumer();

        assert_eq!(received_expected_message(None, RemoteMessageType::WalletBackupProvisioned).unwrap(), false);
        teardown!("agency")
    }
}