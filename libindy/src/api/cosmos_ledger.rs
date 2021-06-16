use indy_api_types::{errors::prelude::*, CommandHandle, ErrorCode};

use crate::services::CommandMetric;
use crate::Locator;
use indy_utils::ctypes;
use libc::c_char;

#[no_mangle]
pub extern "C" fn indy_cosmos_ledger_build_tx(
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
        "indy_cosmos_ledger_build_tx > pool_alias {:?} sender_alias {:?} msg_raw {:?} \
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
        "indy_cosmos_ledger_build_tx > pool_alias {:?} sender_alias {:?} msg_raw {:?} \
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
            .verim_ledger_controller
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
        debug!("indy_cosmos_ledger_build_tx ? err {:?} tx {:?}", err, tx);

        let (tx_raw, tx_len) = ctypes::vec_to_pointer(&tx);
        cb(command_handle, err, tx_raw, tx_len)
    };

    locator
        .executor
        .spawn_ok_instrumented(CommandMetric::CosmosLedgerCommandBuildTx, action, cb);

    let res = ErrorCode::Success;
    debug!("indy_cosmos_ledger_build_tx < {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn indy_cosmos_ledger_build_query_cosmos_auth_account(
    command_handle: CommandHandle,
    address: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, query: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_cosmos_ledger_build_query_cosmos_auth_account > address {:?}",
        address
    );

    check_useful_c_str!(address, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_cosmos_ledger_build_query_cosmos_auth_account > address {:?}",
        address
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .build_query_cosmos_auth_account(&address);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, query) = prepare_result!(res, String::new());
        debug!(
            "indy_cosmos_ledger_build_query_cosmos_auth_account: query: {:?}",
            query
        );

        let query = ctypes::string_to_cstring(query);
        cb(command_handle, err, query.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::CosmosLedgerCommandBuildQueryCosmosAuthAccount,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!(
        "indy_cosmos_ledger_build_query_cosmos_auth_account < {:?}",
        res
    );
    res
}

#[no_mangle]
pub extern "C" fn indy_cosmos_ledger_parse_query_cosmos_auth_account_resp(
    command_handle: CommandHandle,
    query_resp: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, resp: *const c_char)>,
) -> ErrorCode {
    debug!(
        "indy_cosmos_ledger_parse_query_cosmos_auth_account_resp > query_resp {:?}",
        query_resp
    );

    check_useful_c_str!(query_resp, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!(
        "indy_cosmos_ledger_parse_query_cosmos_auth_account_resp > query_resp {:?}",
        query_resp
    );

    let locator = Locator::instance();

    let action = async move {
        let res = locator
            .verim_ledger_controller
            .parse_query_cosmos_auth_account_resp(&query_resp);
        res
    };

    let cb = move |res: IndyResult<_>| {
        let (err, resp) = prepare_result!(res, String::new());
        debug!(
            "indy_cosmos_ledger_parse_query_cosmos_auth_account_resp: resp: {:?}",
            resp
        );
        let resp = ctypes::string_to_cstring(resp);
        cb(command_handle, err, resp.as_ptr())
    };

    locator.executor.spawn_ok_instrumented(
        CommandMetric::CosmosLedgerCommandParseQueryCosmosAuthAccountResp,
        action,
        cb,
    );

    let res = ErrorCode::Success;
    debug!(
        "indy_cosmos_ledger_parse_query_cosmos_auth_account_resp < {:?}",
        res
    );
    res
}