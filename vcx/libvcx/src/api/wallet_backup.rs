use libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::threadpool::spawn;
use error::prelude::*;
use wallet_backup::{create_wallet_backup, backup_wallet, get_source_id, get_state, from_string, to_string, update_state, restore_wallet};
use messages::get_message::Message;
use std::ptr;

/// -> Create a Wallet Backup object that provides a Cloud wallet backup and provision's backup protocol with Agent
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user
///
/// wallet_encryption_key: String representing the User's Key for securing (encrypting) the exported Wallet.
///
/// cb: Callback that provides wallet_backup handle and error status of request
///
/// #Returns
/// Error code as a u32
///
#[no_mangle]
#[allow(unused_assignments)]
pub extern fn vcx_wallet_backup_create(command_handle: u32,
                                       source_id: *const c_char,
                                       wallet_encryption_key: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, wallet_backup_handle: u32)>) -> u32 {
    info!("vcx_wallet_backup_create >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(wallet_encryption_key, VcxErrorKind::InvalidOption);

    trace!("vcx_wallet_backup_create(command_handle: {}, source_id: {}, wallet_backup_key: ***)", command_handle, source_id);

    spawn(move|| {
        match create_wallet_backup(&source_id, &wallet_encryption_key) {
            Ok(handle) => {
                trace!("vcx_wallet_backup_create(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, handle, source_id);
                cb(command_handle, error::SUCCESS.code_num, handle);
            },
            Err(x) => {
                warn!("vcx_wallet_backup_create(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x, 0, source_id);
                cb(command_handle, x.into(), 0);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Wallet Backup to the Cloud
///
/// #Params:
/// command_handle: Handle for User's Reference only.
/// wallet_backup_handle: Wallet Backup handle that was provided during creation. Used to access object
/*
    Todo: path is needed because the only exposed libindy functionality for exporting
    an encrypted wallet, writes it to the file system. A possible better way is for libindy's export_wallet
    to optionally return an encrypted stream of bytes instead of writing it to the fs. This could also
    be done in a separate libindy api call if necessary.
 */
/// Todo: path will not be necessary when libindy functionality for wallet export functionality is expanded
/// Todo: path must be different than other exported wallets because this instance is deleted after its uploaded to the cloud
/// path: Path to export wallet to User's File System. (This instance of the export
/// cb: Callback that provides the success/failure of the api call.
/// #Returns
/// Error code - success indicates that the api call was successfully created and execution
/// is scheduled to begin in a separate thread.

#[no_mangle]
pub extern fn vcx_wallet_backup_backup(command_handle: u32,
                                       wallet_backup_handle: u32,
                                       path: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    info!("vcx_wallet_backup_backup >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(path,  VcxErrorKind::InvalidOption);

    trace!("vcx_wallet_backup_backup(command_handle: {}, wallet_backup_handle: {}, path: {})",
           command_handle, wallet_backup_handle, path);

    spawn(move|| {
        trace!("vcx_wallet_backup_backup(command_handle: {}, wallet_backup_handle: {}, path: {})",
               command_handle, wallet_backup_handle, path);
        match backup_wallet(wallet_backup_handle, &path) {
            Ok(_) => {
                let return_code = error::SUCCESS.code_num;
                trace!("vcx_wallet_backup_backup(command_handle: {}, rc: {})", command_handle, return_code);
                cb(command_handle, return_code);
            }
            Err(e) => {
                warn!("vcx_wallet_backup_backup(command_handle: {}, rc: {})", command_handle, e);
                cb(command_handle, e.into());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks for any state change and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// wallet_backup_handle: was provided during creation. Used to identify connection object
///
/// cb: Callback that provides most current state of the wallet_backup and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_wallet_backup_update_state(command_handle: u32,
                                             wallet_backup_handle: u32,
                                             cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_wallet_backup_update_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(wallet_backup_handle).unwrap_or_default();
    trace!("vcx_wallet_backup_update_state(command_handle: {}, wallet_backup: {}), source_id: {:?}",
           command_handle, wallet_backup_handle, source_id);

    spawn(move|| {
        match update_state(wallet_backup_handle, None) {
            Ok(x) => {
                trace!("vcx_wallet_backup_update_state(command_handle: {}, rc: {}, wallet_backup_handle: {}, state: {}), source_id: {:?}",
                       command_handle, error::SUCCESS.message, wallet_backup_handle, get_state(wallet_backup_handle), source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(x) => {
                warn!("vcx_wallet_backup_update_state(command_handle: {}, rc: {}, wallet_backup_handle: {}, state: {}), source_id: {:?}",
                      command_handle, x, wallet_backup_handle, get_state(wallet_backup_handle), source_id);
                cb(command_handle, x.into(), 0);
            },
        }

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks the message any state change and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// wallet_backup_handle: was provided during creation. Used to identify connection object
///
/// message: message to process
///
/// cb: Callback that provides most current state of the wallet_backup and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_wallet_backup_update_state_with_message(command_handle: u32,
                                                          wallet_backup_handle: u32,
                                                          message: *const c_char,
                                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_wallet_backup_update_state_with_message >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(message, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(wallet_backup_handle).unwrap_or_default();
    trace!("vcx_wallet_backup_update_state_with_message(command_handle: {}, wallet_backup: {}), source_id: {:?}",
           command_handle, wallet_backup_handle, source_id);

    let message: Message = match serde_json::from_str(&message) {
        Ok(x) => x,
        Err(_) => return VcxError::from(VcxErrorKind::InvalidJson).into(),
    };

    spawn(move|| {
        match update_state(wallet_backup_handle, Some(message)) {
            Ok(x) => {
                trace!("vcx_wallet_backup_update_state_with_message(command_handle: {}, rc: {}, wallet_backup_handle: {}, state: {}), source_id: {:?}",
                       command_handle, error::SUCCESS.message, wallet_backup_handle, get_state(wallet_backup_handle), source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(x) => {
                warn!("vcx_wallet_backup_update_state_with_message(command_handle: {}, rc: {}, wallet_backup_handle: {}, state: {}), source_id: {:?}",
                      command_handle, x, wallet_backup_handle, get_state(wallet_backup_handle), source_id);
                cb(command_handle, x.into(), 0);
            },
        }

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the wallet backup object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// handle: Wallet Backup handle that was provided during creation. Used to identify the wallet backup object
///
/// cb: Callback that provides json string of the wallet backup's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_wallet_backup_serialize(command_handle: u32,
                                          wallet_backup_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, data: *const c_char)>) -> u32 {
    info!("vcx_wallet_backup_serialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(wallet_backup_handle).unwrap_or_default();
    trace!("vcx_wallet_backup_serialize(command_handle: {}, proof_handle: {}) source_id: {}",
           command_handle, wallet_backup_handle, source_id);

    spawn(move || {
        match to_string(wallet_backup_handle) {
            Ok(x) => {
                trace!("vcx_wallet_backup_serialize_cb(command_handle: {}, rc: {}, data: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                error!("vcx_wallet_backup_serialize_cb(command_handle: {}, rc: {}, data: {}) source_id: {}",
                       command_handle, x, 0, source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an wallet backup object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// data: json string representing a wallet backup object
///
///
/// cb: Callback that provides handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_wallet_backup_deserialize(command_handle: u32,
                                            wallet_backup_str: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {
    info!("vcx_wallet_backup_deserialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(wallet_backup_str, VcxErrorKind::InvalidOption);

    trace!("vcx_wallet_backup_deserialize(command_handle: {}, proof_data: {})",
           command_handle, wallet_backup_str);

    spawn(move || {
        match from_string(&wallet_backup_str) {
            Ok(x) => {
                trace!("vcx_wallet_backup_deserialize_cb(command_handle: {}, rc: {}, wallet_backup_handle: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, x, get_source_id(x).unwrap_or_default());

                cb(command_handle, 0, x);
            }
            Err(x) => {
                error!("vcx_wallet_backup_deserialize_cb(command_handle: {}, rc: {}, wallet_backup_handle: {}) source_id: {}",
                       command_handle, x, 0, "");
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Requests a recovery of a backup previously stored with a cloud agent
///
/// config: "{"wallet_name":"","wallet_key":"","exported_wallet_path":"","backup_key":"","key_derivation":""}"
/// backup_key: Key used when creating the backup of the wallet (For encryption/decrption)
/// cb: Callback that provides the success/failure of the api call.
/// #Returns
/// Error code - success indicates that the api call was successfully created and execution
/// is scheduled to begin in a separate thread.
#[no_mangle]
pub extern fn vcx_wallet_backup_restore(command_handle: u32,
                                        config: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    info!("vcx_wallet_backup_recovery >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(config,  VcxErrorKind::InvalidOption);

    trace!("vcx_wallet_backup_recovery(command_handle: {}, config: ****)",
           command_handle);

    spawn(move|| {
        trace!("vcx_wallet_backup_recovery(command_handle: {}, config: ****)", command_handle);
        match restore_wallet(&config) {
            Ok(_) => {
                trace!("vcx_wallet_backup_recovery(command_handle: {}, rc: {})", command_handle, error::SUCCESS.message);
                cb(command_handle, error::SUCCESS.code_num);
            }
            Err(e) => {
                warn!("vcx_wallet_backup_recovery(command_handle: {}, rc: {})", command_handle, e);
                cb(command_handle, e.into());
            }
        };
        Ok(())
    });

    error::SUCCESS.code_num
}

//#[cfg(feature = "wallet_backup")]
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::ffi::CString;
//    use std::ptr;
//    use utils::error;
//    use std::time::Duration;
//    use api::{return_types_u32};
//    use serde_json::Value;
//    use wallet_backup;
//
//    #[test]
//    fn test_vcx_wallet_backup_create() {
//        init!("true");
//        let cb = return_types_u32::Return_U32_U32::new().unwrap();
//        let rc = vcx_wallet_backup_create(cb.command_handle,
//                                       CString::new("test_create").unwrap().into_raw(),
//                                          CString::new("pass_phrae").unwrap().into_raw(),
//                                       Some(cb.get_callback()));
//        assert_eq!(rc, error::SUCCESS.code_num);
//        assert!(cb.receive(Some(Duration::from_secs(10))).unwrap() > 0);
//    }
//
//    #[test]
//    fn test_vcx_wallet_backup_create_fails() {
//        init!("true");
//        let rc = vcx_wallet_backup_create(0,
//                                          CString::new("test_create_fails").unwrap().into_raw(),
//                                          CString::new("pass_phrae").unwrap().into_raw(),
//                                       None);
//        assert_eq!(rc, error::INVALID_OPTION.code_num);
//        let cb = return_types_u32::Return_U32_U32::new().unwrap();
//        let rc = vcx_wallet_backup_create(cb.command_handle,
//                                       ptr::null(),
//                                          CString::new("pass_phrae").unwrap().into_raw(),
//                                       Some(cb.get_callback()));
//        assert_eq!(rc, error::INVALID_OPTION.code_num);
//    }
//
//    #[test]
//    fn test_wallet_backup() {
//        init!("true");
//
//        let cb = return_types_u32::Return_U32_U32::new().unwrap();
//        let rc = vcx_wallet_backup_create(cb.command_handle,
//                                          CString::new("test_create").unwrap().into_raw(),
//                                          CString::new("encryption_key").unwrap().into_raw(),
//                                          Some(cb.get_callback()));
//        let wallet_handle = cb.receive(Some(Duration::from_secs(50))).unwrap();
//
//        let cb = return_types_u32::Return_U32::new().unwrap();
//        assert_eq!(vcx_wallet_backup_backup(cb.command_handle,
//                                     wallet_handle,
//                                     CString::new("path").unwrap().into_raw(),
//                                     Some(cb.get_callback())), error::SUCCESS.code_num);
//        cb.receive(Some(Duration::from_secs(50))).unwrap();
//
//    }
//
//    #[test]
//    fn test_vcx_wallet_backup_serialize_and_deserialize() {
//        init!("true");
//        let cb = return_types_u32::Return_U32_STR::new().unwrap();
//        let handle = wallet_backup::create_wallet_backup("abc", "encryption_key").unwrap();
//        assert_eq!(vcx_wallet_backup_serialize(cb.command_handle,
//                                                 handle,
//                                                 Some(cb.get_callback())), error::SUCCESS.code_num);
//        let s = cb.receive(Some(Duration::from_secs(2))).unwrap().unwrap();
//        let j: Value = serde_json::from_str(&s).unwrap();
//        assert_eq!(j["version"], ::utils::constants::DEFAULT_SERIALIZE_VERSION);
//
//        let cb = return_types_u32::Return_U32_U32::new().unwrap();
//        assert_eq!(vcx_wallet_backup_deserialize(cb.command_handle,
//                                                   CString::new(s).unwrap().into_raw(),
//                                                   Some(cb.get_callback())),
//                   error::SUCCESS.code_num);
//
//        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
//        assert!(handle > 0);
//    }
//
//    #[test]
//    fn test_vcx_wallet_backup_update_state() {
//        init!("true");
//
//        let cb = return_types_u32::Return_U32_U32::new().unwrap();
//        let rc = vcx_wallet_backup_create(cb.command_handle,
//                                          CString::new("test_create").unwrap().into_raw(),
//                                          CString::new("encryption key").unwrap().into_raw(),
//                                          Some(cb.get_callback()));
//        let wallet_handle = cb.receive(Some(Duration::from_secs(50))).unwrap();
//
//        ::utils::httpclient::set_next_u8_response(Vec::new());
//        let cb = return_types_u32::Return_U32_U32::new().unwrap();
//        assert_eq!(vcx_wallet_backup_update_state(cb.command_handle,
//                                                  wallet_handle,
//                                            Some(cb.get_callback())), error::SUCCESS.code_num);
//        let state = cb.receive(Some(Duration::from_secs(50))).unwrap();
//        assert_eq!(state, ::api::WalletBackupState::InitRequested as u32)
//    }
//}
