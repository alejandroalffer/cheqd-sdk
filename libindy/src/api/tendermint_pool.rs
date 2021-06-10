use indy_api_types::{errors::prelude::*, CommandHandle, ErrorCode};

use crate::services::CommandMetric;
use crate::Locator;
use indy_utils::ctypes;
use libc::c_char;

#[no_mangle]
pub extern "C" fn indy_tendermint_pool_add(
    command_handle: CommandHandle,
    alias: *const c_char,
    rpc_address: *const c_char,
    chain_id: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, pool_info: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_tendermint_pool_add > alias {:?} rpc_address {:?} chain_id {:?}",
        alias, rpc_address, chain_id
    );

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(rpc_address, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(chain_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "indy_tendermint_pool_add > alias {:?} rpc_address {:?} chain_id {:?}",
        alias, rpc_address, chain_id
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .tendermint_pool_controller
            .add(&alias, &rpc_address, &chain_id)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_tendermint_pool_add ? err {:?} pool_info {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::TendermintPoolCommandAdd, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_tendermint_pool_add < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_tendermint_pool_get_config(
    command_handle: CommandHandle,
    alias: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, pool_info: *const c_char),
    >,
) -> ErrorCode {
    debug!("indy_tendermint_pool_get_config > alias {:?}", alias);

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!("indy_tendermint_pool_get_config > alias {:?}", alias);

    let locator = Locator::instance();

    let action = async move {
        let res = locator.tendermint_pool_controller.get_config(&alias).await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_tendermint_pool_get_config ? err {:?} pool_info {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::TendermintPoolCommandGetConfig, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_tendermint_pool_get_config < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_tendermint_pool_broadcast_tx_commit(
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
        "indy_tendermint_pool_broadcast_tx_commit > pool_alias {:?} signed_tx_raw {:?} signed_tx_len {:?}",
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
        "indy_tendermint_pool_broadcast_tx_commit > pool_alias {:?} signed_tx_raw {:?} signed_tx_len {:?}",
        pool_alias, signed_tx_raw, signed_tx_len
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .tendermint_pool_controller
            .broadcast_tx_commit(&pool_alias, &signed_tx_raw)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_tendermint_pool_broadcast_tx_commit ? err {:?} tx_commit_response {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::TendermintPoolCommandBroadcastTxCommit, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_tendermint_pool_broadcast_tx_commit < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_tendermint_pool_abci_query(
    command_handle: CommandHandle,
    pool_alias: *const c_char,
    req_json: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_tendermint_pool_abci_query > pool_alias {:?}, req_json {:?} ",
        pool_alias, req_json
    );

    check_useful_c_str!(pool_alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(req_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    debug!(
        "indy_tendermint_pool_abci_query > pool_alias {:?}, req_json {:?} ",
        pool_alias, req_json
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .tendermint_pool_controller
            .abci_query(&pool_alias, &req_json)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!("indy_tendermint_pool_abci_query ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::TendermintPoolCommandAbciQuery, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_tendermint_pool_abci_query < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_tendermint_pool_abci_info(
    command_handle: CommandHandle,
    pool_alias: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_tendermint_pool_abci_info > pool_alias {:?}",
        pool_alias
    );

    check_useful_c_str!(pool_alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_tendermint_pool_abci_info > pool_alias {:?} ",
        pool_alias
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .tendermint_pool_controller
            .abci_info(&pool_alias)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, res) = prepare_result!(res, String::new());
        debug!("indy_tendermint_pool_abci_info ? err {:?} res {:?}", err, res);

        let res = ctypes::string_to_cstring(res);
        cb(command_handle, err, res.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::TendermintPoolCommandAbciInfo, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_tendermint_pool_abci_info < {:?}", res);
    res
}