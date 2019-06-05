//use serde_json;
//use serde_json::Value;

//use api::{VcxStateType};
use settings;
//use messages;
//use messages::{GeneralMessage, MessageStatusCode, RemoteMessageType, ObjectWithVersion};
//use messages::get_message::{Message, MessagePayload};
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::wallet::{export, get_wallet_handle};
use std::path::Path;
use std::fs;
//use utils::constants::DEFAULT_SERIALIZE_VERSION;
//use utils::json::KeyMatch;
//use std::collections::HashMap;


lazy_static! {
    static ref WALLET_BACKUP_MAP: ObjectCache<WalletBackup> = Default::default();
}

 #[derive(Clone, Debug, Serialize, Deserialize)]
enum WalletBackupState {
    Uninitialized(),
    Initialized(),
    BackupInProgress(),
    WalletBackupStored(),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WalletBackup {
    source_id: String,
    state: WalletBackupState,
    to_did: String, // user agent did
    uuid: Option<String>,
}

impl WalletBackup {

    fn create(source_id: &str) -> VcxResult<WalletBackup> {
        Ok(WalletBackup {
            source_id: source_id.to_string(),
            state: WalletBackupState::Uninitialized(),
            to_did: settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?,
            uuid: None,
        })
    }

    fn initialize_wallet_backup(&mut self) -> VcxResult<u32> {
        //Todo: Agency Message for Initializing Wallet Protocol
       Ok(error::SUCCESS.code_num)
    }

    fn backup(&mut self, backup_key: &str, exported_wallet_path: &str) -> VcxResult<u32> {
        let wallet_data = WalletBackup::_retrieve_exported_wallet(backup_key, exported_wallet_path)?;
        // Todo: Agency Message Posting to deliver wallet_data to the user agent
        Ok(error::SUCCESS.code_num)
    }

    fn _retrieve_exported_wallet(backup_key: &str, exported_wallet_path: &str) -> VcxResult<Vec<u8>> {

        let path = Path::new(exported_wallet_path);
        export(get_wallet_handle(), &path, backup_key)?;
        let data = fs::read(&path).map_err(|err| VcxError::from(VcxErrorKind::RetrieveExportedWallet))?;
        fs::remove_file(path).map_err(|err| VcxError::from(VcxErrorKind::RetrieveExportedWallet))?;

        Ok(data)
    }
}

pub fn create_wallet_backup(source_id: &str) -> VcxResult<u32> {
    trace!("create_wallet_backup >>> source_id: {}", source_id);

    // Send WalletBackupInit -> Agency
    let wb = WalletBackup::create(source_id)?;

    WALLET_BACKUP_MAP.add(wb)
        .or(Err(VcxError::from(VcxErrorKind::CreateWalletBackup)))
}

pub fn initialize_wallet_backup(handle: u32) -> VcxResult<u32> {
    WALLET_BACKUP_MAP.get_mut(handle, |wb| {
        wb.initialize_wallet_backup()
    })
}

/*
    Todo: exported_wallet_path is needed because the only exposed libindy functionality for exporting
    an encrypted wallet, writes it to the file system. A possible better way is for libindy's export_wallet
    to optionally return an encrypted stream of bytes instead of writing it to the fs. This could also
    be done in a separate libindy api call if necessary.
 */
pub fn backup_wallet(handle: u32, backup_key: &str, exported_wallet_path: &str) -> VcxResult<u32> {
    WALLET_BACKUP_MAP.get_mut(handle, |wb| {
        wb.backup(backup_key, exported_wallet_path)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::devsetup::tests::setup_wallet_env;

    static SOURCE_ID: &str = r#"12345"#;
    static FILE_PATH: &str = r#"/tmp/tmp_wallet"#;
    static BACKUP_KEY: &str = r#"12345"#;

    mod create_backup_wallet {
       use super::*;

        #[test]
        fn create_wallet_backup_succeeds() {
            init!("true");
            assert!(create_wallet_backup(SOURCE_ID).is_ok())
        }

    }

    mod initialize_wallet_backup_protocol {
        use super::*;

        #[test]
        fn initialize_protocol_fails_with_invalid_handle() {
            init!("true");
            assert_eq!(initialize_wallet_backup(0).unwrap_err().kind(), VcxErrorKind::InvalidHandle)
        }

        #[test]
        fn initialize_protocol_success() {
            init!("true");
            let handle = create_wallet_backup(SOURCE_ID).unwrap();
            assert!(initialize_wallet_backup(handle).is_ok())
        }
    }

    mod wallet_backup_init_response {

    }

    mod backup_wallet {
        use super::*;

        mod retrieve_exported_wallet {
            use super::*;

            #[test]
            fn retrieving_exported_wallet_data_successful() {
                init!("true");
                setup_wallet_env(settings::DEFAULT_WALLET_NAME).unwrap();

                let data = WalletBackup::_retrieve_exported_wallet(BACKUP_KEY, FILE_PATH);

                assert!(data.unwrap().len() > 0);
            }
        }


        #[test]
        fn backup_wallet_fails_with_invalid_handle() {
            init!("true");
            assert_eq!(backup_wallet(0, BACKUP_KEY, FILE_PATH).unwrap_err().kind(), VcxErrorKind::InvalidHandle)
        }

        #[test]
        fn backup_wallet_succeeds() {
            init!("true");
            setup_wallet_env(settings::DEFAULT_WALLET_NAME).unwrap();

            let wallet_backup = create_wallet_backup(SOURCE_ID).unwrap();
            assert!(backup_wallet(wallet_backup, BACKUP_KEY, FILE_PATH).is_ok());
        }

        #[test]
        fn backup_wallet_fails_with_no_wallet() {

        }

        #[test]
        fn backup_fails_with_agency_error_response() {

        }
    }
}

