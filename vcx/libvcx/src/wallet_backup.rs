use api::WalletBackupState;
use settings;
use messages;
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::wallet::{export, get_wallet_handle};
use utils::libindy::crypto::{create_key, sign, pack_message};
use utils::constants::{DEFAULT_SERIALIZE_VERSION};
use std::path::Path;
use std::fs;
use messages::RemoteMessageType;
use messages::wallet_backup::received_expected_message;
use messages::get_message::Message;
use utils::openssl::sha256_hex;

lazy_static! {
    static ref WALLET_BACKUP_MAP: ObjectCache<WalletBackup> = Default::default();
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct CloudAddress {
    agent_did: String,
    agent_vk: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DeadDropAddress {
    address: String,
    locator: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WalletBackupKeys {
    wallet_encryption_key: String,
    recovery_vk: String,
    dead_drop_address: DeadDropAddress,
    cloud_address: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WalletBackup {
    source_id: String,
    state: WalletBackupState,
    to_did: String, // user agent did
    uuid: Option<String>,
    keys: WalletBackupKeys,
    has_stored_backup: bool,
}

impl CloudAddress {
    fn to_string(&self) -> VcxResult<String> {
        messages::ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize CloudAddress"))
    }

    fn from_str(data: &str) -> VcxResult<CloudAddress> {
        messages::ObjectWithVersion::deserialize(data)
            .map(|obj: messages::ObjectWithVersion<CloudAddress>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize CloudAddress"))
    }
}

impl WalletBackup {

    fn get_source_id(&self) -> &String { &self.source_id }

    fn has_stored_backup(&self) -> bool {
        trace!("WalletBackup::has_cloud_backup >>>");
        self.has_stored_backup
    }

    fn set_state(&mut self, state: WalletBackupState) {
        trace!("WalletBackup::set_state: {:?} >>>", state);
        self.state = state
    }

    fn get_state(&self) -> u32 {
        trace!("WalletBackup::get_state >>>");
        self.state as u32
    }

    fn update_state(&mut self, message: Option<Message>) -> VcxResult<u32> {
        debug!("updating state for wallet_backup {}", self.source_id);

        match self.state {
            WalletBackupState::InitRequested =>
                if received_expected_message(message, RemoteMessageType::WalletBackupProvisioned)? {
                    self.state = WalletBackupState::ReadyToExportWallet
                },
            WalletBackupState::BackupInProgress =>
                if received_expected_message(message, RemoteMessageType::WalletBackupAck)? {
                    self.has_stored_backup = true;
                    self.state = WalletBackupState::ReadyToExportWallet
                },
            _ => ()
        }
        Ok(self.get_state())
    }

    fn create(source_id: &str, wallet_encryption_key: &str) -> VcxResult<WalletBackup> {
        Ok(WalletBackup {
            source_id: source_id.to_string(),
            state: WalletBackupState::Uninitialized,
            to_did: settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?,
            keys: gen_keys(wallet_encryption_key)?,
            uuid: None,
            has_stored_backup: false
        })
    }

    fn init_backup(&mut self) -> VcxResult<u32> {
        trace!("init_backup >>> ");

        messages::wallet_backup_init()
            .recovery_vk(&self.keys.recovery_vk)?
            .dead_drop_address(&self.keys.dead_drop_address.address)?
            .cloud_address(&self.keys.cloud_address)?
            .send_secure()?;

        self.state = WalletBackupState::InitRequested;

       Ok(error::SUCCESS.code_num)
    }

    fn backup(&mut self, exported_wallet_path: &str) -> VcxResult<u32> {
        let wallet_data = WalletBackup::_retrieve_exported_wallet(&self.keys.wallet_encryption_key, exported_wallet_path)?;

        messages::backup_wallet()
            .wallet_data(wallet_data)
            .send_secure()?;

        self.state = WalletBackupState::BackupInProgress;

        Ok(error::SUCCESS.code_num)
    }

    fn _retrieve_exported_wallet(backup_key: &str, exported_wallet_path: &str) -> VcxResult<Vec<u8>> {
        if settings::test_indy_mode_enabled() { return Ok(Vec::new()) }

        let path = Path::new(exported_wallet_path);
        export(get_wallet_handle(), &path, backup_key)?;
        let data = fs::read(&path).map_err(|err| VcxError::from(VcxErrorKind::RetrieveExportedWallet))?;
        fs::remove_file(path).map_err(|err| VcxError::from(VcxErrorKind::RetrieveExportedWallet))?;

        Ok(data)
    }

    fn to_string(&self) -> VcxResult<String> {
        trace!("WalletBackup::to_string >>>");
        messages::ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize WalletBackup"))
    }

    fn from_str(data: &str) -> VcxResult<WalletBackup> {
        trace!("WalletBackup::from_str >>> data: {}", secret!(&data));
        messages::ObjectWithVersion::deserialize(data)
            .map(|obj: messages::ObjectWithVersion<WalletBackup>| obj.data)
            .map_err(|err| err.extend("Cannot deserialize WalletBackup"))
    }
}

pub fn create_wallet_backup(source_id: &str, wallet_encryption_key: &str) -> VcxResult<u32> {
    trace!("create_wallet_backup >>> source_id: {}", source_id);

    let mut wb = WalletBackup::create(source_id, wallet_encryption_key)?;

    println!("keys: {:?}", wb.keys);
    wb.init_backup()?;

    WALLET_BACKUP_MAP.add(wb)
        .or(Err(VcxError::from(VcxErrorKind::CreateWalletBackup)))
}

fn gen_keys(wallet_encryption_key: &str) -> VcxResult<WalletBackupKeys> {
    let vk_seed = sha256_hex(wallet_encryption_key.as_bytes());
    let vk = &create_key(Some(&vk_seed), None)?;
    Ok(WalletBackupKeys {
        wallet_encryption_key: wallet_encryption_key.to_string(),
        recovery_vk: vk.to_string(),
        dead_drop_address: gen_deaddrop_address(vk)?,
        cloud_address: gen_cloud_address(vk)?,
    })
}

fn gen_deaddrop_address(vk: &str) -> VcxResult<DeadDropAddress> {
    trace!("gen_deaddrop_address >>> vk: {}", vk);
    let locator = sha256_hex(&sign(vk, "wallet-backup".as_bytes())?);
    Ok(DeadDropAddress {
        locator: locator.to_string(),
        address: sha256_hex((vk.to_string() + &locator).as_bytes()),
    })

}

fn gen_cloud_address(vk: &str) -> VcxResult<Vec<u8>> {
    trace!("gen_cloud_address >>> vk: {}", vk);
    let cloud_address = CloudAddress {
        agent_did: settings::get_config_value(::settings::CONFIG_REMOTE_TO_SDK_DID)?,
        agent_vk: settings::get_config_value(::settings::CONFIG_REMOTE_TO_SDK_VERKEY)?
    };

    let receiver_keys = json!([vk]).to_string();
    pack_message(None, &receiver_keys, cloud_address.to_string()?.as_bytes())
}

/*
    Todo: exported_wallet_path is needed because the only exposed libindy functionality for exporting
    an encrypted wallet, writes it to the file system. A possible better way is for libindy's export_wallet
    to optionally return an encrypted stream of bytes instead of writing it to the fs. This could also
    be done in a separate libindy api call if necessary.
 */
pub fn backup_wallet(handle: u32, exported_wallet_path: &str) -> VcxResult<u32> {
    WALLET_BACKUP_MAP.get_mut(handle, |wb| {
        wb.backup(exported_wallet_path)
    })
}

pub fn recover_wallet(backup_key: &str) -> VcxResult<u32> {
    Ok(1)
}

pub fn is_valid_handle(handle: u32) -> bool { WALLET_BACKUP_MAP.has_handle(handle) }

pub fn get_state(handle: u32) -> u32 {
    WALLET_BACKUP_MAP.get(handle, |wb| {
        debug!("get state for wallet_backup {}", wb.get_source_id());
        Ok(wb.get_state().clone())
    }).unwrap_or(WalletBackupState::Uninitialized as u32)
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    WALLET_BACKUP_MAP.get(handle, |wb| {
        Ok(wb.get_source_id().clone())
    }).or(Err(VcxError::from(VcxErrorKind::InvalidHandle)))
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    WALLET_BACKUP_MAP.get(handle, |obj| {
        WalletBackup::to_string(&obj)
    })
}

pub fn from_string(wallet_backup_data: &str) -> VcxResult<u32> {
    let wallet_backup: WalletBackup = WalletBackup::from_str(wallet_backup_data)?;

    let new_handle = WALLET_BACKUP_MAP.add(wallet_backup)?;

    info!("inserting handle {} into wallet backup table", new_handle);

    Ok(new_handle)
}

pub fn set_state(handle: u32, state: WalletBackupState) -> VcxResult<()> {
    WALLET_BACKUP_MAP.get_mut(handle, |wb| {
        Ok(wb.set_state(state))
    })
}

pub fn update_state(handle: u32, message: Option<Message>) -> VcxResult<u32> {
    WALLET_BACKUP_MAP.get_mut(handle, |wb| {
        wb.update_state(message.clone())
    })
}

pub fn has_known_cloud_backup(handle: u32) -> bool {
    WALLET_BACKUP_MAP.get(handle, |wb| {
        Ok(wb.has_stored_backup().clone())
    }).unwrap_or(false)
}

#[cfg(feature = "wallet_backup")]
#[cfg(test)]
mod tests {
    use super::*;
    use utils::devsetup::tests::setup_wallet_env;
    use serde_json::Value;
    use std::thread;
    use std::time::Duration;

    pub const WALLET_PROVISION_AGENT_RESPONSE: &'static [u8; 2] = &[79, 75];
    static SOURCE_ID: &str = r#"12345"#;
    static FILE_PATH: &str = r#"/tmp/tmp_wallet"#;
    static BACKUP_KEY: &str = r#"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY"#;

    mod create_wallet_backup {
       use super::*;

        #[cfg(feature = "agency")]
        #[cfg(feature = "pool_tests")]
        #[test]
        fn create_backup_succeeds_real() {
            init!("agency");
            ::utils::devsetup::tests::set_consumer();

            assert!(create_wallet_backup(SOURCE_ID, BACKUP_KEY).is_ok())
        }

    }

    mod update_state {
        use super::*;

        #[test]
        fn update_state_success() {
            init!("true");
            ::utils::httpclient::set_next_u8_response(WALLET_PROVISION_AGENT_RESPONSE.to_vec());

            let handle = create_wallet_backup(SOURCE_ID, BACKUP_KEY).unwrap();
            assert!(update_state(handle, None).is_ok());
            assert_eq!(get_state(handle), WalletBackupState::InitRequested as u32);
        }

        #[cfg(feature = "agency")]
        #[cfg(feature = "pool_tests")]
        #[test]
        fn update_state_with_provisioned_msg_changes_state_to_ready_to_export() {
            init!("agency");
            ::utils::devsetup::tests::set_consumer();

            let handle = create_wallet_backup(SOURCE_ID, BACKUP_KEY).unwrap();
            thread::sleep(Duration::from_millis(2000));

            assert!(update_state(handle, None).is_ok());
            assert_eq!(get_state(handle), WalletBackupState::ReadyToExportWallet as u32);
        }

        #[cfg(feature = "agency")]
        #[cfg(feature = "pool_tests")]
        #[test]
        fn update_state_with_backup_ack_msg_changes_state_to_ready_to_export() {
            init!("agency");

            ::utils::devsetup::tests::set_consumer();
            let handle = create_wallet_backup(SOURCE_ID, BACKUP_KEY).unwrap();
            thread::sleep(Duration::from_millis(2000));

            assert!(update_state(handle, None).is_ok());
            assert_eq!(get_state(handle), WalletBackupState::ReadyToExportWallet as u32);

            backup_wallet(handle, FILE_PATH).unwrap();
            assert_eq!(get_state(handle), WalletBackupState::BackupInProgress as u32);

            assert!(update_state(handle, None).is_ok());
            assert_eq!(get_state(handle), WalletBackupState::ReadyToExportWallet as u32);
        }
    }

    mod serialization {
        use super::*;

        #[test]
        fn to_string_test() {
            init!("true");
            ::utils::httpclient::set_next_u8_response(WALLET_PROVISION_AGENT_RESPONSE.to_vec());

            let handle = create_wallet_backup(SOURCE_ID, BACKUP_KEY).unwrap();
            let serialized = to_string(handle).unwrap();
            let j: Value = serde_json::from_str(&serialized).unwrap();
            assert_eq!(j["version"], "1.0");
            WalletBackup::from_str(&serialized).unwrap();
        }

        #[test]
        fn test_deserialize_fails() {
            assert_eq!(from_string("{}").unwrap_err().kind(), VcxErrorKind::InvalidJson);
        }
    }

    mod backup_wallet {
        use super::*;

        #[test]
        fn retrieving_exported_wallet_data_successful() {
            init!("true");
            setup_wallet_env(settings::DEFAULT_WALLET_NAME).unwrap();

            let data = WalletBackup::_retrieve_exported_wallet(BACKUP_KEY, FILE_PATH);

            assert!(data.unwrap().len() > 0);
        }

        #[test]
        fn backup_wallet_fails_with_invalid_handle() {
            init!("true");
            assert_eq!(backup_wallet(0, FILE_PATH).unwrap_err().kind(), VcxErrorKind::InvalidHandle)
        }

        #[cfg(feature = "agency")]
        #[cfg(feature = "pool_tests")]
        #[test]
        fn backup_wallet_succeeds_real() {
            init!("agency");

            let wallet_backup = create_wallet_backup(SOURCE_ID, BACKUP_KEY).unwrap();
            thread::sleep(Duration::from_millis(2000));

            assert_eq!(get_state(wallet_backup), WalletBackupState::InitRequested as u32);
            assert!(update_state(wallet_backup, None).is_ok());

            backup_wallet(wallet_backup, FILE_PATH).unwrap();
            assert_eq!(get_state(wallet_backup), WalletBackupState::BackupInProgress as u32);

            assert!(update_state(wallet_backup, None).is_ok());
            assert_eq!(get_state(wallet_backup), WalletBackupState::ReadyToExportWallet as u32);
            assert!(has_known_cloud_backup(wallet_backup))
        }
    }
}

