use std::ptr;

use indy_api_types::{
    errors::prelude::*, validation::Validatable, CommandHandle, ErrorCode, PoolHandle, WalletHandle,
};

use indy_utils::ctypes;
use libc::c_char;
use serde_json;

use crate::services::CommandMetric;
use crate::{
    domain::{
        crypto::{
            did::{DidMethod, DidValue, MyDidInfo, TheirDidInfo},
            key::KeyInfo,
        },
        ledger::attrib::Endpoint,
    },
    Locator,
};

/// Creates keys (signing and encryption keys) for a new account.
/// #Params
/// alias: alias for a new keys
/// Example:
/// {
///     "alias": string
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   alias: alias for a new keys
///   account_id: address of a new keys
///   pub_key: public key
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_keys_add_random(
    command_handle: CommandHandle,
    alias: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, key_info: *const c_char),
    >,
) -> ErrorCode {
    // debug!("indy_add_random > alias {:?}", alias);
    //
    // check_useful_c_str!(alias, ErrorCode::CommonInvalidParam1);
    // check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);
    //
    // debug!("indy_add_random ? alias {:?}", alias);
    //
    // let locator = Locator::instance();
    //
    // let action = async move {
    //     let res = locator.keys_controller.add_random(alias).await;
    //     res
    // };
    //
    // let cb = move |res: IndyResult<_>| {
    //     let (err, (key_info)) = prepare_result!(res, KeyInfo::new());
    //
    //     debug!("indy_add_random ? err {:?} key_info {:?}", err, key_info);
    //
    //     let alias = ctypes::string_to_cstring(key_info.alias);
    //     let account_id = ctypes::string_to_cstring(key_info.account_id);
    //     let pub_key = ctypes::string_to_cstring(key_info.pub_key);
    //     cb(
    //         command_handle,
    //         err,
    //         alias.as_ptr(),
    //         account_id.as_ptr(),
    //         pub_key.as_ptr(),
    //     )
    // };
    //
    // // TODO: add metrics
    // // locator.executor.spawn_ok_instrumented(CommandMetric::DidCommandCreateAndStoreMyDid, action, cb);
    //
    // let res = ErrorCode::Success;
    // debug!("indy_add_random < {:?}", res);
    // res
    unimplemented!()
}

/// Creates keys (signing and encryption keys) for a new account.
/// #Params
/// alias: alias for a new keys
/// mnemonic: for generating keys
/// Example:
/// {
///     "alias": string
///     "mnemonic": string
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   alias: alias for a new keys
///   account_id: address of a new keys
///   pub_key: public key
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_keys_add_from_mnemonic(
    alias: *const c_char,
    mnemonic: *const c_char,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            alias: *const c_char,
            account_id: *const c_char,
            pub_key: *const c_char,
        ),
    >,
) -> ErrorCode {
    // debug!(
    //     "add_from_mnemonic > alias {:?} mnemonic {:?}",
    //     alias, mnemonic
    // );
    // check_useful_validatable_string!(alias, ErrorCode::CommonInvalidParam1, String);
    // check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);
    //
    // debug!(
    //     "add_from_mnemonic ? alias {:?} mnemonic {:?}",
    //     alias, mnemonic
    // );
    //
    // let locator = Locator::instance();
    //
    // let action = async move {
    //     let res = locator.keys_controller.add_random(alias, mnemonic).await;
    //     res
    // };
    //
    // let cb = move |res: IndyResult<_>| {
    //     let (err, (key_info)) = prepare_result!(res, KeyInfo::new());
    //
    //     debug!("add_from_mnemonic ? err {:?} key_info {:?}", err, key_info);
    //
    //     let alias = ctypes::string_to_cstring(key_info.alias);
    //     let account_id = ctypes::string_to_cstring(key_info.account_id);
    //     let pub_key = ctypes::string_to_cstring(key_info.pub_key);
    //     cb(
    //         command_handle,
    //         err,
    //         alias.as_ptr(),
    //         account_id.as_ptr(),
    //         pub_key.as_ptr(),
    //     )
    // };
    //
    // // TODO: add metrics
    // // locator.executor.spawn_ok_instrumented(CommandMetric::DidCommandCreateAndStoreMyDid, action, cb);
    //
    // let res = ErrorCode::Success;
    // debug!("add_from_mnemonic < {:?}", res);
    // res
    unimplemented!()
}

/// Get Key info by alias
/// #Params
/// alias: account alias for getting its keys
/// Example:
/// {
///     "alias": string
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   alias: alias of asked keys
///   account_id: address of asked keys
///   pub_key: public key of asked keys
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_keys_key_info(
    alias: *const c_char,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            alias: *const c_char,
            account_id: *const c_char,
            pub_key: *const c_char,
        ),
    >,
) -> ErrorCode {
    // debug!("key_info > alias {:?}", alias);
    // check_useful_validatable_string!(alias, ErrorCode::CommonInvalidParam1, String);
    // check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);
    //
    // debug!("key_info ? alias {:?}", alias);
    //
    // let locator = Locator::instance();
    //
    // let action = async move {
    //     let res = locator.keys_controller.key_info(alias).await;
    //     res
    // };
    //
    // let cb = move |res: IndyResult<_>| {
    //     let (err, (key_info)) = prepare_result!(res, KeyInfo::new());
    //
    //     debug!("key_info ? err {:?} key_info {:?}", err, key_info);
    //
    //     let alias = ctypes::string_to_cstring(key_info.alias);
    //     let account_id = ctypes::string_to_cstring(key_info.account_id);
    //     let pub_key = ctypes::string_to_cstring(key_info.pub_key);
    //     cb(
    //         command_handle,
    //         err,
    //         alias.as_ptr(),
    //         account_id.as_ptr(),
    //         pub_key.as_ptr(),
    //     )
    // };
    //
    // // TODO: add metrics
    // // locator.executor.spawn_ok_instrumented(CommandMetric::DidCommandCreateAndStoreMyDid, action, cb);
    //
    // let res = ErrorCode::Success;
    // debug!("key_info < {:?}", res);
    // res
    unimplemented!()
}
