use libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::threadpool::spawn;
use error::prelude::*;
use wallet_backup::{create_wallet_backup, backup_wallet, get_source_id, get_state};
use messages::get_message::Message;

/// -> Create a Wallet Backup object that provides a Cloud wallet backup and provision's backup protocol with Agent
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user
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
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, wallet_backup_handle: u32)>) -> u32 {
    info!("vcx_wallet_backup_create >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);

    trace!("vcx_wallet_backup_create(command_handle: {}, source_id: {})", command_handle, source_id);

    spawn(move|| {
        match create_wallet_backup(&source_id) {
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
/// backup_key: String representing the User's Key for securing (encrypting) the exported Wallet.
/// cb: Callback that provides the success/failure of the api call.
/// #Returns
/// Error code - success indicates that the api call was successfully created and execution
/// is scheduled to begin in a separate thread.

#[no_mangle]
pub extern fn vcx_wallet_backup_backup(command_handle: u32,
                                       wallet_backup_handle: u32,
                                       path: *const c_char,
                                       backup_key: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    info!("vcx_wallet_backup_backup >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(path,  VcxErrorKind::InvalidOption);
    check_useful_c_str!(backup_key, VcxErrorKind::InvalidOption);

    trace!("vcx_wallet_backup_backup(command_handle: {}, wallet_backup_handle: {}, path: {}, backup_key: ****)",
           command_handle, wallet_backup_handle, path);

    spawn(move|| {
        trace!("vcx_wallet_backup_backup(command_handle: {}, wallet_backup_handle: {}, path: {}, backup_key: ****)",
               command_handle, wallet_backup_handle, path);
        match backup_wallet(wallet_backup_handle, &backup_key, &path) {
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
        // Todo: Remove
        let rc = 0; // Temporary for testing
        let state = get_state(wallet_backup_handle);
        cb(command_handle, rc, state);

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
//        let rc = match process_acceptance_message(wallet_backup_handle, message) {
//            Ok(x) => {
//                trace!("vcx_wallet_backup_update_state_with_message(command_handle: {}, rc: {}, wallet_backup_handle: {}, state: {}), source_id: {:?}",
//                       command_handle, error::SUCCESS.message, connection_handle, get_state(connection_handle), source_id);
//                x
//            },
//            Err(x) => {
//                warn!("vcx_wallet_backup_update_state_with_message(command_handle: {}, rc: {}, wallet_backup_handle: {}, state: {}), source_id: {:?}",
//                      command_handle, x, connection_handle, get_state(connection_handle), source_id);
//                x.into()
//            },
//        };
        // Todo: Remove
        let rc = 0; // Temporary for testing
        let state = get_state(wallet_backup_handle);
        cb(command_handle, rc, state);

        Ok(())
    });

    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use utils::error;
    use std::time::Duration;
    use api::{return_types_u32};

    #[test]
    fn test_vcx_wallet_backup_create() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_wallet_backup_create(cb.command_handle,
                                       CString::new("test_create").unwrap().into_raw(),
                                       Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(cb.receive(Some(Duration::from_secs(10))).unwrap() > 0);
    }

    #[test]
    fn test_vcx_wallet_backup_create_fails() {
        init!("true");
        let rc = vcx_wallet_backup_create(0,
                                          CString::new("test_create_fails").unwrap().into_raw(),
                                       None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_wallet_backup_create(cb.command_handle,
                                       ptr::null(),
                                       Some(cb.get_callback()));
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    #[test]
    fn test_wallet_backup() {
        use utils::libindy::wallet::delete_wallet;
        use utils::devsetup::tests::setup_wallet_env;
        use std::env;
        use std::fs;
        use std::path::Path;
        use std::time::Duration;
        use settings;

        settings::set_defaults();
        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let backup_key = "backup_key";
        let mut dir = env::temp_dir();
        dir.push("tmp_exported_wallet");
        if Path::new(&dir).exists() {
            fs::remove_file(Path::new(&dir)).unwrap();
        }
        let dir_c_str = CString::new(dir.to_str().unwrap()).unwrap();

       setup_wallet_env(&wallet_name).unwrap();

        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_wallet_backup_create(cb.command_handle,
                                          CString::new("test_create").unwrap().into_raw(),
                                          Some(cb.get_callback()));
        let wallet_handle = cb.receive(Some(Duration::from_secs(50))).unwrap();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_wallet_backup_backup(cb.command_handle,
                                     wallet_handle,
                                     dir_c_str.as_ptr(),
                                     CString::new(backup_key).unwrap().into_raw(),
                                     Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(50))).unwrap();

        delete_wallet(&wallet_name, None, None, None).unwrap();
    }
}
