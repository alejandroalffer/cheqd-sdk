use std::ptr;

use indy_api_types::{
    errors::prelude::*, validation::Validatable, CommandHandle, ErrorCode, PoolHandle, WalletHandle,
};

use indy_utils::ctypes;
use libc::c_char;
use serde_json;

use crate::services::CommandMetric;
use crate::Locator;

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
pub extern "C" fn indy_cosmos_keys_add_random(
    command_handle: CommandHandle,
    alias: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, key_info: *const c_char),
    >,
) -> ErrorCode {
    debug!("indy_cosmos_keys_add_random > alias {:?} ", alias);

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam1);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    debug!("indy_cosmos_keys_add_random > alias {:?} ", alias);

    let locator = Locator::instance();

    let action = async move {
        let res = locator.cosmos_keys_controller.add_random(&alias).await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!("indy_cosmos_keys_add_random ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosKeysAddRandom, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_replace_keys_start < {:?}", res);
    res
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
pub extern "C" fn indy_cosmos_keys_add_from_mnemonic(
    command_handle: CommandHandle,
    alias: *const c_char,
    mnemonic: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, key_info: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_cosmos_keys_add_from_mnemonic > alias {:?}, mnemonic {:?} ",
        alias, mnemonic
    );

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam1);
    check_useful_c_str!(mnemonic, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_cosmos_keys_add_from_mnemonic > alias {:?}, mnemonic {:?} ",
        alias, mnemonic
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cosmos_keys_controller
            .add_from_mnemonic(&alias, &mnemonic)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!(
            "indy_cosmos_keys_add_from_mnemonic ? err {:?} res {:?}",
            err, res
        );

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosKeysAddFromMnemonic, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cosmos_keys_add_from_mnemonic < {:?}", res);
    res
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
pub extern "C" fn indy_cosmos_keys_get_info(
    command_handle: CommandHandle,
    alias: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, key_info: *const c_char),
    >,
) -> ErrorCode {
    debug!("indy_cosmos_keys_key_info > alias {:?} ", alias);

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam1);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    debug!("indy_cosmos_keys_key_info > alias {:?} ", alias);

    let locator = Locator::instance();

    let action = async move {
        let res = locator.cosmos_keys_controller.get_info(&alias).await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!("indy_cosmos_keys_key_info ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosKeysKeyInfo, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cosmos_keys_key_info < {:?}", res);
    res
}

/// Sign
/// #Params
/// alias: account alias for getting its keys
/// tx: SignDoc
/// Example:
/// {
///     "alias": string
///     "tx": &[u8]
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Tx string
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_cosmos_keys_sign(
    command_handle: CommandHandle,
    alias: *const c_char,
    tx_raw: *const u8,
    tx_len: u32,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            signed_raw: *const u8,
            signed_len: u32,
        ),
    >,
) -> ErrorCode {
    debug!(
        "indy_cosmos_keys_sign > alias {:?} tx_raw {:?} tx_len {:?}",
        alias, tx_raw, tx_len
    );

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam1);
    check_useful_c_byte_array!(
        tx_raw,
        tx_len,
        ErrorCode::CommonInvalidParam2,
        ErrorCode::CommonInvalidParam3
    );
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    debug!("indy_cosmos_keys_sign > alias {:?} ", alias);

    let locator = Locator::instance();

    let action = async move {
        let res = locator.cosmos_keys_controller.sign(&alias, &tx_raw).await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, Vec::new());
        debug!("indy_cosmos_keys_sign ? err {:?} res {:?}", err, res);

        let (signed_raw, signed_len) = ctypes::vec_to_pointer(&res);
        cb(command_handle, err, signed_raw, signed_len)
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosKeysSign, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cosmos_keys_sign < {:?}", res);
    res
}
