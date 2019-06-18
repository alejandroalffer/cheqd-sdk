use api::WalletBackupState;
use settings;
use messages;
use object_cache::ObjectCache;
use error::prelude::*;
use utils::error;
use utils::libindy::wallet::{export, get_wallet_handle};
use utils::constants::{DEFAULT_SERIALIZE_VERSION};
use std::path::Path;
use std::fs;


lazy_static! {
    static ref WALLET_BACKUP_MAP: ObjectCache<WalletBackup> = Default::default();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WalletBackup {
    source_id: String,
    state: WalletBackupState,
    to_did: String, // user agent did
    uuid: Option<String>,
}

impl WalletBackup {

    fn get_source_id(&self) -> &String { &self.source_id }

    fn get_state(&self) -> u32 {
        trace!("WalletBackup::get_state >>>");
        self.state as u32
    }

    fn create(source_id: &str) -> VcxResult<WalletBackup> {
        Ok(WalletBackup {
            source_id: source_id.to_string(),
            state: WalletBackupState::Uninitialized,
            to_did: settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?,
            uuid: None,
        })
    }

    fn provision_backup(&mut self) -> VcxResult<u32> {
        trace!("provision_backup >>> ");

        messages::wallet_backup_provision()
//            .send_secure()?
            ;

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

pub fn create_wallet_backup(source_id: &str) -> VcxResult<u32> {
    trace!("create_wallet_backup >>> source_id: {}", source_id);

    let mut wb = WalletBackup::create(source_id)?;

    wb.provision_backup()?;

    WALLET_BACKUP_MAP.add(wb)
        .or(Err(VcxError::from(VcxErrorKind::CreateWalletBackup)))
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


#[cfg(test)]
mod tests {
    use super::*;
    use utils::devsetup::tests::setup_wallet_env;
    use serde_json::Value;

    static SOURCE_ID: &str = r#"12345"#;
    static FILE_PATH: &str = r#"/tmp/tmp_wallet"#;
    static BACKUP_KEY: &str = r#"12345"#;

    mod create_wallet_backup {
       use super::*;

        #[test]
        fn create_backup_succeeds() {
            init!("true");
            assert!(create_wallet_backup(SOURCE_ID).is_ok())
        }

    }

    mod serialization {
        use super::*;

        #[test]
        fn to_string_test() {
            init!("true");
            let handle = create_wallet_backup(SOURCE_ID).unwrap();
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

