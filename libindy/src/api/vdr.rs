use indy_api_types::{
    CommandHandle, ErrorCode, VdrHandle,
};

use libc::c_char;

#[no_mangle]
pub extern "C" fn indy_vdr_create(
    command_handle: CommandHandle,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, handle: VdrHandle)>,
) -> ErrorCode {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn indy_vdr_register_indy_ledger(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace_list: *const c_char,
    genesis_txn_data: *const c_char,
    taa_config: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn indy_vdr_register_cheqd_ledger(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace_list: *const c_char,
    chain_id: *const c_char,
    node_addrs_list: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_ping(
    command_handle: CommandHandle,
    handle: VdrHandle,
    namespace_list: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, status_list: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_cleanup(
    command_handle: CommandHandle,
    handle: VdrHandle,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_resolve_did(
    command_handle: CommandHandle,
    handle: VdrHandle,
    fqdid: *const c_char,
    cache_options: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, diddoc: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_resolve_schema(
    command_handle: CommandHandle,
    handle: VdrHandle,
    fqdid: *const c_char,
    cache_options: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, schema: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_resolve_cred_def(
    command_handle: CommandHandle,
    handle: VdrHandle,
    fqcreddef: *const c_char,
    cache_options: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, cred_def: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_prepare_did(
    command_handle: CommandHandle,
    handle: VdrHandle,
    txn_specific_params: *const c_char,
    submitter_did: *const c_char,
    endorser: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, prepared_txn: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_prepare_schema(
    command_handle: CommandHandle,
    handle: VdrHandle,
    txn_specific_params: *const c_char,
    submitter_did: *const c_char,
    endorser: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, prepared_txn: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_prepare_cred_def(
    command_handle: CommandHandle,
    handle: VdrHandle,
    txn_specific_params: *const c_char,
    submitter_did: *const c_char,
    endorser: *const c_char,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, prepared_txn: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn indy_vdr_submit_txn(
    command_handle: CommandHandle,
    handle: VdrHandle,
    prepared_txn: *const c_char,
    signature_raw: *const u8,
    signature_len: u32,
    endorsement_raw: *const u8,
    endorsement_len: u32,
    cb: Option<extern "C" fn(command_handle_: CommandHandle, err: ErrorCode, status: *const c_char)>,
) -> ErrorCode {
    unimplemented!();
}
