use indy_api_types::{errors::prelude::*, CommandHandle, ErrorCode};

use crate::services::CommandMetric;
use crate::Locator;
use indy_utils::ctypes;
use libc::c_char;
use serde_json;

#[no_mangle]
pub extern "C" fn indy_cosmos_pool_add(
    command_handle: CommandHandle,
    alias: *const c_char,
    rpc_address: *const c_char,
    chain_id: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, pool_info: *const c_char),
    >,
) -> ErrorCode {
    debug!(
        "indy_cosmos_pool_add > alias {:?} rpc_address {:?} chain_id {:?}",
        alias, rpc_address, chain_id
    );

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(rpc_address, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(chain_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "indy_cosmos_pool_add > alias {:?} rpc_address {:?} chain_id {:?}",
        alias, rpc_address, chain_id
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cosmos_pool_controller
            .add(&alias, &rpc_address, &chain_id)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_cosmos_pool_add ? err {:?} pool_info {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosPoolAdd, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cosmos_pool_add < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_cosmos_pool_get_config(
    command_handle: CommandHandle,
    alias: *const c_char,
    cb: Option<
        extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, pool_info: *const c_char),
    >,
) -> ErrorCode {
    debug!("indy_cosmos_pool_get_config > alias {:?}", alias);

    check_useful_c_str!(alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!("indy_cosmos_pool_get_config > alias {:?}", alias);

    let locator = Locator::instance();

    let action = async move {
        let res = locator.cosmos_pool_controller.get_config(&alias).await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "indy_cosmos_pool_get_config ? err {:?} pool_info {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosPoolGetConfig, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cosmos_pool_get_config < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_cosmos_pool_build_tx(
    command_handle: CommandHandle,
    pool_alias: *const c_char,
    sender_alias: *const c_char,
    msg_raw: *const u8,
    msg_len: u32,
    account_number: u64,
    sequence_number: u64,
    max_gas: u64,
    max_coin_amount: u64,
    max_coin_denom: *const c_char,
    timeout_height: u64,
    memo: *const c_char,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            tx_raw: *const u8,
            tx_len: u32,
        ),
    >,
) -> ErrorCode {
    debug!(
        "indy_cosmos_pool_build_tx > pool_alias {:?} sender_alias {:?} msg_raw {:?} \
        msg_len {:?} account_number {:?} sequence_number {:?} max_gas {:?} max_coin_amount \
        {:?} max_coin_denom {:?} timeout_height {:?} memo {:?}",
        pool_alias,
        sender_alias,
        msg_raw,
        msg_len,
        account_number,
        sequence_number,
        max_gas,
        max_coin_amount,
        max_coin_denom,
        timeout_height,
        memo
    );

    check_useful_c_str!(pool_alias, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(sender_alias, ErrorCode::CommonInvalidParam3);
    check_useful_c_byte_array!(
        msg_raw,
        msg_len,
        ErrorCode::CommonInvalidParam4,
        ErrorCode::CommonInvalidParam5
    );
    check_useful_c_str!(max_coin_denom, ErrorCode::CommonInvalidParam10);
    check_useful_c_str!(memo, ErrorCode::CommonInvalidParam12);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam13);

    debug!(
        "indy_cosmos_pool_build_tx > pool_alias {:?} sender_alias {:?} msg_raw {:?} \
        account_number {:?} sequence_number {:?} max_gas {:?} max_coin_amount \
        {:?} max_coin_denom {:?} timeout_height {:?} memo {:?}",
        pool_alias,
        sender_alias,
        msg_raw,
        account_number,
        sequence_number,
        max_gas,
        max_coin_amount,
        max_coin_denom,
        timeout_height,
        memo
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cosmos_pool_controller
            .build_tx(
                &pool_alias,
                &sender_alias,
                &msg_raw,
                account_number,
                sequence_number,
                max_gas,
                max_coin_amount,
                &max_coin_denom,
                timeout_height,
                &memo,
            )
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, tx) = prepare_result!(res, Vec::new());
        debug!("indy_cosmos_pool_build_tx ? err {:?} tx {:?}", err, tx);

        let (tx_raw, tx_len) = ctypes::vec_to_pointer(&tx);
        cb(command_handle, err, tx_raw, tx_len)
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosPoolBuildTx, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cosmos_pool_build_tx < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_cosmos_pool_broadcast_tx_commit(
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
        "broadcast_tx_commit > pool_alias {:?} signed_tx_raw {:?} signed_tx_len {:?}",
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
        "broadcast_tx_commit > pool_alias {:?} signed_tx_raw {:?} signed_tx_len {:?}",
        pool_alias, signed_tx_raw, signed_tx_len
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .cosmos_pool_controller
            .broadcast_tx_commit(&pool_alias, &signed_tx_raw)
            .await;
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, pool_info) = prepare_result!(res, String::new());
        debug!(
            "broadcast_tx_commit ? err {:?} tx_commit_response {:?}",
            err, pool_info
        );

        let pool_info = ctypes::string_to_cstring(pool_info);
        cb(command_handle, err, pool_info.as_ptr())
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosPoolBroadcastTxCommit, action, cb);

    let res = ErrorCode::Success;
    debug!("broadcast_tx_commit < {:?}", res);
    res
}
