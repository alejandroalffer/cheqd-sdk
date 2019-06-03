//use serde_json;
//use serde_json::Value;

//use api::{VcxStateType};
use settings;
//use messages;
//use messages::{GeneralMessage, MessageStatusCode, RemoteMessageType, ObjectWithVersion};
//use messages::get_message::{Message, MessagePayload};
use object_cache::ObjectCache;
use error::prelude::*;
//use utils::error;
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
}

pub fn create_wallet_backup(source_id: &str) -> VcxResult<u32> {
    trace!("create_wallet_backup >>> source_id: {}", source_id);

    let wb = WalletBackup::create(source_id)?;

    WALLET_BACKUP_MAP.add(wb)
        .or(Err(VcxError::from(VcxErrorKind::CreateWalletBackup)))
}


#[cfg(test)]
mod tests {
    use super::*;

    mod create_backup_wallet {
       use super::*;

        #[test]
        fn create_wallet_backup_succeeds() {
            init!("true");
            assert!(create_wallet_backup("my id").is_ok())
        }

    }
    mod backup_wallet {
//        use super::*;

//        #[test]
//        fn backup_wallet_succeeds() {
//            assert!(backup_wallet().is_ok())
//        }
//
//        #[test]
//        fn backup_wallet_fails_with_no_wallet() {
//
//        }
//
//        #[test]
//        fn backup_fails_with_agency_error_response() {
//
//        }
    }
}

