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
