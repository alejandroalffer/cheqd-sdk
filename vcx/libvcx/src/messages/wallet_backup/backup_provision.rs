use settings;
use messages::wallet_backup::{prepare_message_for_agency_v2};
use messages::{A2AMessage, A2AMessageV2, A2AMessageKinds};
use messages::message_type::{ MessageTypes };
use error::VcxResult;
use utils::httpclient;

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

        // Agency is no longer sending Specific Response - 200 is sufficient
        httpclient::post_u8(&data)?;

        Ok(())
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        let message = A2AMessage::Version2( A2AMessageV2::BackupProvision(
            BackupProvision {
                msg_type: MessageTypes::MessageTypeV2(MessageTypes::build_v2(
                    A2AMessageKinds::BackupProvision
                )),
            }
        ));

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        prepare_message_for_agency_v2(&message, &agency_did)
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
    use messages::{wallet_backup_provision, RemoteMessageType};
    use std::thread;
    use std::time::Duration;
    use utils::libindy::signus::create_and_store_my_did;
    use messages::wallet_backup::received_expected_message;

    #[test]
    fn test_wallet_backup_provision() {
        init!("ledger");
        let (user_did, user_vk) = create_and_store_my_did(None).unwrap();
        let (agent_did, agent_vk) = create_and_store_my_did(Some(::utils::constants::MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(::utils::constants::MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(::utils::constants::MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = wallet_backup_provision().prepare_request().unwrap();
        assert!(msg.len() > 0);

    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_backup_provision_real() {
        init!("agency");
        ::utils::devsetup::tests::set_institution();

        assert!(wallet_backup_provision().send_secure().is_ok());
        teardown!("agency")
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_received_provisioned_response_true() {
        init!("agency");
        ::utils::devsetup::tests::set_institution();

        wallet_backup_provision().send_secure().unwrap();
        thread::sleep(Duration::from_millis(2000));

        assert_eq!(received_expected_message(None, RemoteMessageType::WalletBackupProvisioned).unwrap(), true);
        teardown!("agency")
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_received_provisioned_response_false() {
        init!("agency");
        ::utils::devsetup::tests::set_institution();

        assert_eq!(received_expected_message(None, RemoteMessageType::WalletBackupProvisioned).unwrap(), false);
        teardown!("agency")
    }
}