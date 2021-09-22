use indy_api_types::{errors::prelude::*, CommandHandle, ErrorCode};

use crate::services::CommandMetric;
use crate::Locator;
use indy_utils::ctypes;
use libc::c_char;

/// Add information about pool
/// #Params
/// command_handle: command handle to map callback to caller context.
/// alias: name of a pool
/// rpc_address: address for making remote calls
/// chain_id: name of network
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   Structure with PoolInfo
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_cheqd_pool_add(
    command_handle: CommandHandle,
    alias: *const c_char,
    rpc_address: *const c_char,
    chain_id: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, pool_info: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_cheqd_pool_add > alias {:?} rpc_address {:?} chain_id {:?}",
        alias, rpc_address, chain_id
    );

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(rpc_address, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(chain_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "indy_cheqd_pool_add > alias {:?} rpc_address {:?} chain_id {:?}",
        alias, rpc_address, chain_id
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cheqd_pool_controller
            .add(&alias, &rpc_address, &chain_id)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_cheqd_pool_add ? err {:?} pool_info {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CheqdPoolCommandAdd, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cheqd_pool_add < {:?}", res);
    res
}

/// Get pool config
/// #Params
/// command_handle: command handle to map callback to caller context.
/// alias: name of a pool
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   Structure with PoolInfo
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_cheqd_pool_get_config(
    command_handle: CommandHandle,
    alias: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, pool_info: *const c_char),
    >,
) -> ErrorCode {
    debug!("indy_cheqd_pool_get_config > alias {:?}", alias);

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!("indy_cheqd_pool_get_config > alias {:?}", alias);

    let locator = Locator::instance();

    let action = async move {
        let res = locator.cheqd_pool_controller.get_config(&alias).await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_cheqd_pool_get_config ? err {:?} pool_info {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CheqdPoolCommandGetConfig, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cheqd_pool_get_config < {:?}", res);
    res
}

/// Get all pool configs
/// #Params
/// command_handle: command handle to map callback to caller context.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   List of pool configs as string json.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_cheqd_pool_get_all_config(
    command_handle: CommandHandle,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, pool_info: *const c_char),
    >,
) -> ErrorCode {
    debug!("indy_cheqd_pool_get_all_config >");

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!("indy_cheqd_pool_get_all_config >");

    let locator = Locator::instance();

    let action = async move {
        let res = locator.cheqd_pool_controller.get_all_config().await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_cheqd_pool_get_all_config ? err {:?} pool_info {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CheqdPoolCommandGetAllConfig, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cheqd_pool_get_all_config < {:?}", res);
    res
}

/// Send broadcast transaction to the whole pool
/// #Params
/// command_handle: command handle to map callback to caller context.
/// pool_alias: name of a pool
/// signed_tx_raw: signed transaction in the raw format
/// signed_tx_len: length of signed transaction
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   Structure TxCommitResponse
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_cheqd_pool_broadcast_tx_commit(
    command_handle: CommandHandle,
    pool_alias: *const c_char,
    signed_tx_raw: *const u8,
    signed_tx_len: u32,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            tx_commit_response: *const c_char,
        ),
    >,
) -> ErrorCode {
    debug!(
        "indy_cheqd_pool_broadcast_tx_commit > pool_alias {:?} signed_tx_raw {:?} signed_tx_len {:?}",
        pool_alias, signed_tx_raw, signed_tx_len
    );

    check_useful_c_str!(pool_alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_byte_array!(
        signed_tx_raw,
        signed_tx_len,
        ErrorCode::CommonInvalidParam3,
        ErrorCode::CommonInvalidParam4
    );
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "indy_cheqd_pool_broadcast_tx_commit > pool_alias {:?} signed_tx_raw {:?} signed_tx_len {:?}",
        pool_alias, signed_tx_raw, signed_tx_len
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cheqd_pool_controller
            .broadcast_tx_commit(&pool_alias, &signed_tx_raw)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, tx_commit_response) = prepare_result!(res, String::new());
        debug!(
            "indy_cheqd_pool_broadcast_tx_commit ? err {:?} tx_commit_response {:?}",
            err, tx_commit_response
        );

        let tx_commit_response = ctypes::string_to_cstring(tx_commit_response);
        cb(command_handle, err, tx_commit_response.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CheqdPoolCommandBroadcastTxCommit, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cheqd_pool_broadcast_tx_commit < {:?}", res);
    res
}

/// Send general ABCI request
/// #Params
/// command_handle: command handle to map callback to caller context.
/// alias: name of a pool
/// req_json: string of ABCI query in json format
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   SResponse with result of ABCI query
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_cheqd_pool_abci_query(
    command_handle: CommandHandle,
    pool_alias: *const c_char,
    req_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_cheqd_pool_abci_query > pool_alias {:?}, req_json {:?} ",
        pool_alias, req_json
    );

    check_useful_c_str!(pool_alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    debug!(
        "indy_cheqd_pool_abci_query > pool_alias {:?}, req_json {:?} ",
        pool_alias, req_json
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cheqd_pool_controller
            .abci_query(&pool_alias, &req_json)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!("indy_cheqd_pool_abci_query ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CheqdPoolCommandAbciQuery, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cheqd_pool_abci_query < {:?}", res);
    res
}

/// Request ABCI information
/// #Params
/// command_handle: command handle to map callback to caller context.
/// pool_alias: name of a pool
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Error Code
/// cb:
/// - err: Error code.
///   General response with information about pool state
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_cheqd_pool_abci_info(
    command_handle: CommandHandle,
    pool_alias: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_cheqd_pool_abci_info > pool_alias {:?}",
        pool_alias
    );

    check_useful_c_str!(pool_alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_cheqd_pool_abci_info > pool_alias {:?} ",
        pool_alias
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cheqd_pool_controller
            .abci_info(&pool_alias)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!("indy_cheqd_pool_abci_info ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CheqdPoolCommandAbciInfo, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cheqd_pool_abci_info < {:?}", res);
    res
}