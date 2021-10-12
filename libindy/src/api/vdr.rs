use std::ptr;

use indy_api_types::{CommandHandle, ErrorCode, errors::prelude::*, INVALID_VDR_HANDLE, VdrHandle};
use indy_api_types::errors::IndyResult;
use indy_utils::ctypes;
use libc::c_char;

use crate::Locator;
use crate::services::CommandMetric;

#[no_mangle]
pub extern "C" fn vdr_create(
    command_handle: CommandHandle,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, handle: VdrHandle)>,
) -> ErrorCode {
    debug!("vdr_create >");

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    debug!("vdr_create ?");

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .create_vdr()
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, vdr_handle) = prepare_result!(res, INVALID_VDR_HANDLE);

        debug!(
            "vdr_create ? err {:?} vdr_handle {:?}",
            err, vdr_handle
        );

        cb(command_handle, err, vdr_handle)
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandCreateVdr, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_create > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_register_indy_ledger(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace_list: *const c_char,
    genesis_txn_data: *const c_char,
    taa_config: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    debug!(
        "vdr_register_indy_ledger > handle {:?} namespace_list {:?} genesis_txn_data {:?} taa_config {:?}",
        handle, namespace_list, genesis_txn_data, taa_config
    );

    check_useful_c_str!(namespace_list, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(genesis_txn_data, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(taa_config, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    debug!(
        "vdr_register_indy_ledger ? handle {:?} namespace_list {:?} genesis_txn_data {:?} taa_config {:?}",
        handle, namespace_list, genesis_txn_data, taa_config
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .register_indy_ledger(&namespace_list, &genesis_txn_data, &taa_config)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<()>| {
        let err = prepare_result!(res);

        debug!("vdr_register_indy_ledger ? err {:?} ", err);

        cb(command_handle, err)
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandRegisterIndyLedger, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_register_indy_ledger > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_register_cheqd_ledger(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace_list: *const c_char,
    chain_id: *const c_char,
    node_addrs_list: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    debug!(
        "vdr_register_cheqd_ledger > handle {:?} namespace_list {:?} chain_id {:?} node_addrs_list {:?}",
        handle, namespace_list, chain_id, node_addrs_list
    );

    check_useful_c_str!(namespace_list, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(chain_id, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(node_addrs_list, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    debug!(
        "vdr_register_cheqd_ledger ? handle {:?} namespace_list {:?} chain_id {:?} node_addrs_list {:?}",
        handle, namespace_list, chain_id, node_addrs_list
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .register_cheqd_ledger(&namespace_list, &chain_id, &node_addrs_list)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<()>| {
        let err = prepare_result!(res);

        debug!("vdr_register_cheqd_ledger ? err {:?} ", err);

        cb(command_handle, err)
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandRegisterCheqdLedger, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_register_cheqd_ledger > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_ping(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace_list: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, status_list: *const c_char)>,
) -> ErrorCode {
    debug!("vdr_ping > handle {:?} namespace_list {:?}", handle, namespace_list);

    check_useful_c_str!(namespace_list, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    debug!("vdr_ping ? handle {:?} namespace_list {:?} ", handle, namespace_list);

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .ping(&namespace_list)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, status_list) = prepare_result!(res, String::new());

        debug!("vdr_ping ? err {:?} status_list {:?}", err, status_list);

        let status_list = ctypes::string_to_cstring(status_list);

        cb(command_handle, err, status_list.as_ptr())
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandPing, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_ping > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_cleanup(
    command_handle: CommandHandle,
    handle: VdrHandle,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    debug!("vdr_cleanup >");

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    debug!("vdr_cleanup ?");

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .cleanup()
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<()>| {
        let err = prepare_result!(res);

        debug!("vdr_cleanup ? err {:?} ", err);

        cb(command_handle, err)
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandCleanup, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_cleanup > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_resolve_did(
    command_handle: CommandHandle,
    handle: VdrHandle,
    fqdid: *const c_char,
    cache_options: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, diddoc: *const c_char)>,
) -> ErrorCode {
    debug!(
        "vdr_resolve_did > handle {:?} fqdid {:?} cache_options {:?}",
        handle, fqdid, cache_options
    );

    check_useful_c_str!(fqdid, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cache_options, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "vdr_resolve_did ? handle {:?} fqdid {:?} cache_options {:?}",
        handle, fqdid, cache_options
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .reslove_did(&fqdid, &cache_options)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, diddoc) = prepare_result!(res, String::new());

        debug!("vdr_resolve_did ? err {:?} diddoc {:?}", err, diddoc);

        let diddoc = ctypes::string_to_cstring(diddoc);

        cb(command_handle, err, diddoc.as_ptr())
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandResolveDid, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_resolve_did > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_resolve_schema(
    command_handle: CommandHandle,
    handle: VdrHandle,
    fqschema: *const c_char,
    cache_options: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, schema: *const c_char)>,
) -> ErrorCode {
    debug!(
        "vdr_resolve_schema > handle {:?} fqschema {:?} cache_options {:?}",
        handle, fqschema, cache_options
    );

    check_useful_c_str!(fqschema, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cache_options, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "vdr_resolve_schema ? handle {:?} fqschema {:?} cache_options {:?}",
        handle, fqschema, cache_options
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .resolve_schema(&fqschema, &cache_options)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, schema) = prepare_result!(res, String::new());

        debug!("vdr_resolve_schema ? err {:?} schema {:?}", err, schema);

        let schema = ctypes::string_to_cstring(schema);

        cb(command_handle, err, schema.as_ptr())
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandResolveSchema, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_resolve_schema > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_resolve_cred_def(
    command_handle: CommandHandle,
    handle: VdrHandle,
    fqcreddef: *const c_char,
    cache_options: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, cred_def: *const c_char)>,
) -> ErrorCode {
    debug!(
        "vdr_resolve_cred_def > handle {:?} fqcreddef {:?} cache_options {:?}",
        handle, fqcreddef, cache_options
    );

    check_useful_c_str!(fqcreddef, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(cache_options, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "vdr_resolve_cred_def ? handle {:?} fqcreddef {:?} cache_options {:?}",
        handle, fqcreddef, cache_options
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .resolve_cred_def(&fqschema, &cache_options)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, cred_def) = prepare_result!(res, String::new());

        debug!("vdr_resolve_cred_def ? err {:?} cred_def {:?}", err, cred_def);

        let cred_def = ctypes::string_to_cstring(cred_def);

        cb(command_handle, err, cred_def.as_ptr())
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandResolveCredDef, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_resolve_cred_def > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_prepare_did(
    command_handle: CommandHandle,
    handle: VdrHandle,
    txn_specific_params: *const c_char,
    submitter_did: *const c_char,
    endorser: *const c_char,
    cb: Option<extern "C" fn(
        command_handle_: CommandHandle,
        err: ErrorCode,
        namespace: *const c_char,
        signature_spec: *const c_char,
        txn_bytes_raw: *const u8,
        txn_bytes_len: u32,
        bytes_to_sign_raw: *const u8,
        bytes_to_sign_len: u32,
        endorsement_spec: *const c_char)>, ) -> ErrorCode {
    debug!(
        "vdr_prepare_did > handle {:?} txn_specific_params {:?} submitter_did {:?} endorser {:?}",
        handle, txn_specific_params, submitter_did, endorser
    );

    check_useful_c_str!(txn_specific_params, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(endorser, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    debug!(
        "vdr_prepare_did ? handle {:?} txn_specific_params {:?} submitter_did {:?} endorser {:?}",
        handle, txn_specific_params, submitter_did, endorser
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .prepare_did(&txn_specific_params, &submitter_did, &endorser)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, (namespace, signature_spec, txn_bytes, bytes_to_sign, endorsement_spec)) = prepare_result!(
            res, String::new(), String::new(), Vec::new(), Vec::new(), None
        );

        debug!(
            "vdr_prepare_did ? err {:?} namespace {:?} signature_spec {:?} txn_bytes {:?} bytes_to_sign {:?} endorsement_spec {:?}",
            err, namespace, signature_spec, txn_bytes, bytes_to_sign, endorsement_spec);

        let namespace = ctypes::string_to_cstring(namespace);
        let signature_spec = ctypes::string_to_cstring(signature_spec);
        let (txn_data, txn_len) = ctypes::vec_to_pointer(&txn_bytes);
        let (bytes_data, bytes_len) = ctypes::vec_to_pointer(&bytes_to_sign);
        let endorsement_spec = endorsement_spec.map(ctypes::string_to_cstring);

        cb(
            command_handle,
            err,
            namespace.as_ptr(),
            signature_spec.as_ptr(),
            txn_data,
            txn_len,
            bytes_data,
            bytes_len,
            endorsement_spec
                .as_ref()
                .map(|vk| vk.as_ptr())
                .unwrap_or(ptr::null()),
        )
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandPrepareDid, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_prepare_did > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_prepare_schema(
    command_handle: CommandHandle,
    handle: VdrHandle,
    txn_specific_params: *const c_char,
    submitter_did: *const c_char,
    endorser: *const c_char,
    cb: Option<extern "C" fn(
        command_handle_: CommandHandle,
        err: ErrorCode,
        namespace: *const c_char,
        signature_spec: *const c_char,
        txn_bytes_raw: *const u8,
        txn_bytes_len: u32,
        bytes_to_sign_raw: *const u8,
        bytes_to_sign_len: u32,
        endorsement_spec: *const c_char)>, ) -> ErrorCode {
    debug!(
        "vdr_prepare_schema > handle {:?} txn_specific_params {:?} submitter_did {:?} endorser {:?}",
        handle, txn_specific_params, submitter_did, endorser
    );

    check_useful_c_str!(txn_specific_params, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(endorser, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    debug!(
        "vdr_prepare_schema ? handle {:?} txn_specific_params {:?} submitter_did {:?} endorser {:?}",
        handle, txn_specific_params, submitter_did, endorser
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .prepare_schema(&txn_specific_params, &submitter_did, &endorser)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, (namespace, signature_spec, txn_bytes, bytes_to_sign, endorsement_spec)) = prepare_result!(
            res, String::new(), String::new(), Vec::new(), Vec::new(), None
        );

        debug!(
            "vdr_prepare_schema ? err {:?} namespace {:?} signature_spec {:?} txn_bytes {:?} bytes_to_sign {:?} endorsement_spec {:?}",
            err, namespace, signature_spec, txn_bytes, bytes_to_sign, endorsement_spec);

        let namespace = ctypes::string_to_cstring(namespace);
        let signature_spec = ctypes::string_to_cstring(signature_spec);
        let (txn_data, txn_len) = ctypes::vec_to_pointer(&txn_bytes);
        let (bytes_data, bytes_len) = ctypes::vec_to_pointer(&bytes_to_sign);
        let endorsement_spec = endorsement_spec.map(ctypes::string_to_cstring);

        cb(
            command_handle,
            err,
            namespace.as_ptr(),
            signature_spec.as_ptr(),
            txn_data,
            txn_len,
            bytes_data,
            bytes_len,
            endorsement_spec
                .as_ref()
                .map(|vk| vk.as_ptr())
                .unwrap_or(ptr::null()),
        )
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandPrepareSchema, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_prepare_schema > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_prepare_cred_def(
    command_handle: CommandHandle,
    handle: VdrHandle,
    txn_specific_params: *const c_char,
    submitter_did: *const c_char,
    endorser: *const c_char,
    cb: Option<extern "C" fn(
        command_handle_: CommandHandle,
        err: ErrorCode,
        namespace: *const c_char,
        signature_spec: *const c_char,
        txn_bytes_raw: *const u8,
        txn_bytes_len: u32,
        bytes_to_sign_raw: *const u8,
        bytes_to_sign_len: u32,
        endorsement_spec: *const c_char)>,
) -> ErrorCode {
    debug!(
        "vdr_prepare_cred_def > handle {:?} txn_specific_params {:?} submitter_did {:?} endorser {:?}",
        handle, txn_specific_params, submitter_did, endorser
    );

    check_useful_c_str!(txn_specific_params, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(endorser, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    debug!(
        "vdr_prepare_cred_def ? handle {:?} txn_specific_params {:?} submitter_did {:?} endorser {:?}",
        handle, txn_specific_params, submitter_did, endorser
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .prepare_cred_def(&txn_specific_params, &submitter_did, &endorser)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, (namespace, signature_spec, txn_bytes, bytes_to_sign, endorsement_spec)) = prepare_result!(
            res, String::new(), String::new(), Vec::new(), Vec::new(), None
        );

        debug!(
            "vdr_prepare_cred_def ? err {:?} namespace {:?} signature_spec {:?} txn_bytes {:?} bytes_to_sign {:?} endorsement_spec {:?}",
            err, namespace, signature_spec, txn_bytes, bytes_to_sign, endorsement_spec);

        let namespace = ctypes::string_to_cstring(namespace);
        let signature_spec = ctypes::string_to_cstring(signature_spec);
        let (txn_data, txn_len) = ctypes::vec_to_pointer(&txn_bytes);
        let (bytes_data, bytes_len) = ctypes::vec_to_pointer(&bytes_to_sign);
        let endorsement_spec = endorsement_spec.map(ctypes::string_to_cstring);

        cb(
            command_handle,
            err,
            namespace.as_ptr(),
            signature_spec.as_ptr(),
            txn_data,
            txn_len,
            bytes_data,
            bytes_len,
            endorsement_spec
                .as_ref()
                .map(|vk| vk.as_ptr())
                .unwrap_or(ptr::null()),
        )
    };

    //locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandPrepareCredDef, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_prepare_cred_def > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_submit_txn(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace: *const c_char,
    signature_spec: *const c_char,
    txn_bytes_raw: *const u8,
    txn_bytes_len: u32,
    signature_raw: *const u8,
    signature_len: u32,
    endorsement: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, response: *const c_char)>,
) -> ErrorCode {
    debug!(
        "vdr_submit_txn > handle {:?} namespace {:?} signature_spec {:?} txn_bytes_raw {:?} bytes_to_sign_len {:?} signature_raw {:?} signature_len {:?} endorsement {:?}",
        handle, namespace, signature_spec, txn_bytes_raw, txn_bytes_len, signature_raw, signature_len, endorsement
    );

    check_useful_c_str!(namespace, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(signature_spec, ErrorCode::CommonInvalidParam4);
    check_useful_c_byte_array!(
        txn_bytes_raw,
        txn_bytes_len,
        ErrorCode::CommonInvalidParam5,
        ErrorCode::CommonInvalidParam6
    );
    check_useful_c_byte_array!(
        signature_raw,
        signature_len,
        ErrorCode::CommonInvalidParam7,
        ErrorCode::CommonInvalidParam8
    );
    check_useful_opt_c_str!(endorsement, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam10);

    debug!(
        "vdr_submit_txn ? handle {:?} namespace {:?} txn_bytes_raw {:?} txn_bytes_len {:?} signature_raw {:?} signature_len {:?} endorsement {:?}",
        handle, namespace, txn_bytes_raw, txn_bytes_len, signature_raw, signature_len, endorsement
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .submit_txn(&prepared_txn, &signature_raw, &endorsement_raw)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, response) = prepare_result!(res, String::new());

        debug!("vdr_submit_txn ? err {:?} response {:?}", err, response);

        let response = ctypes::string_to_cstring(response);

        cb(command_handle, err, response.as_ptr())
    };

    //locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandSubmitTxn, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_submit_txn > {:?}", res);
    res
}

#[no_mangle]
pub extern "C" fn vdr_submit_query(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace: *const c_char,
    query: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, response: *const c_char)>,
) -> ErrorCode {
    debug!(
        "vdr_submit_query > handle {:?} namespace {:?} query {:?}",
        handle, namespace, query
    );

    check_useful_c_str!(namespace, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(query, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    debug!(
        "vdr_submit_query ? handle {:?} namespace {:?} query {:?}",
        handle, namespace, query
    );

    let locator = Locator::instance();

    // let action = async move {
    //     let res = locator
    //         .vdr_controller
    //         .submit_query(handle, namespace, query)
    //         .await;
    //     res
    // };

    let cb = move |res: IndyResult<_>| {
        let (err, response) = prepare_result!(res, String::new());

        debug!("vdr_submit_query ? err {:?} response {:?}", err, response);

        let response = ctypes::string_to_cstring(response);

        cb(command_handle, err, response.as_ptr())
    };

    // locator.executor.spawn_ok_instrumented(CommandMetric::VdrCommandSubmitTxn, action, cb);

    let res = ErrorCode::Success;
    debug!("vdr_submit_query > {:?}", res);
    res
}
