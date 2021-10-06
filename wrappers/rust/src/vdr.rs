use std::ffi::CString;

use futures::Future;

use crate::{ErrorCode, IndyError};
use crate::{CommandHandle, VdrHandle};
use crate::ffi::{ResponseEmptyCB, ResponseStringCB, ResponseVdrHandleCB};
use crate::ffi::vdr;
use crate::utils::callbacks::{ClosureHandler, ResultHandler};

pub fn vdr_create() -> Box<dyn Future<Item=VdrHandle, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_vdrhandle();

    let err = _vdr_create(command_handle, cb);

    ResultHandler::vdrhandle(command_handle, err, receiver)
}

fn _vdr_create(command_handle: CommandHandle, cb: Option<ResponseVdrHandleCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        vdr::indy_vdr_create(command_handle, cb)
    })
}

pub fn vdr_register_indy_ledger(vdr_handle: VdrHandle, namespace_list: &str, genesis_txn_data: &str, taa_config: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _vdr_register_indy_ledger(command_handle, vdr_handle, namespace_list, genesis_txn_data, taa_config, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _vdr_register_indy_ledger(command_handle: CommandHandle, vdr_handle: VdrHandle, namespace_list: &str, genesis_txn_data: &str, taa_config: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let namespace_list = c_str!(namespace_list);
    let genesis_txn_data = c_str!(genesis_txn_data);
    let taa_config = c_str!(taa_config);
    ErrorCode::from(unsafe {
        vdr::indy_vdr_register_indy_ledger(command_handle, vdr_handle, namespace_list.as_ptr(), genesis_txn_data.as_ptr(), taa_config.as_ptr(), cb)
    })
}

pub fn vdr_register_cheqd_ledger(vdr_handle: VdrHandle, namespace_list: &str, chain_id: &str, node_addrs_list: &str) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _vdr_register_cheqd_ledger(command_handle, vdr_handle, namespace_list, chain_id, node_addrs_list, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _vdr_register_cheqd_ledger(command_handle: CommandHandle, vdr_handle: VdrHandle, namespace_list: &str, chain_id: &str, node_addrs_list: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    let namespace_list = c_str!(namespace_list);
    let chain_id = c_str!(chain_id);
    let node_addrs_list = c_str!(node_addrs_list);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_register_indy_ledger(command_handle, vdr_handle, namespace_list.as_ptr(), chain_id.as_ptr(), node_addrs_list.as_ptr(), cb)
    })
}

pub fn vdr_ping(vdr_handle: VdrHandle, namespace_list: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_ping(command_handle, vdr_handle, namespace_list, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_ping(command_handle: CommandHandle, vdr_handle: VdrHandle, namespace_list: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let namespace_list = c_str!(namespace_list);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_ping(command_handle, vdr_handle, namespace_list.as_ptr(), cb)
    })
}

pub fn vdr_cleanup(vdr_handle: VdrHandle) -> Box<dyn Future<Item=(), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

    let err = _vdr_cleanup(command_handle, vdr_handle, cb);

    ResultHandler::empty(command_handle, err, receiver)
}

fn _vdr_cleanup(command_handle: CommandHandle, vdr_handle: VdrHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
        vdr::indy_vdr_cleanup(command_handle, vdr_handle, cb)
    })
}

pub fn vdr_resolve_did(vdr_handle: VdrHandle, fqdid: &str, cache_options: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_resolve_did(command_handle, vdr_handle, fqdid, cache_options, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_resolve_did(command_handle: CommandHandle, vdr_handle: VdrHandle, fqdid: &str, cache_options: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let fqdid = c_str!(fqdid);
    let cache_options = c_str!(cache_options);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_resolve_did(command_handle, vdr_handle, fqdid.as_ptr(), cache_options.as_ptr(), cb)
    })
}

pub fn vdr_resolve_schema(vdr_handle: VdrHandle, fqschema: &str, cache_options: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_resolve_schema(command_handle, vdr_handle, fqschema, cache_options, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_resolve_schema(command_handle: CommandHandle, vdr_handle: VdrHandle, fqschema: &str, cache_options: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let fqschema = c_str!(fqschema);
    let cache_options = c_str!(cache_options);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_resolve_schema(command_handle, vdr_handle, fqschema.as_ptr(), cache_options.as_ptr(), cb)
    })
}

pub fn vdr_resolve_cred_def(vdr_handle: VdrHandle, fqcreddef: &str, cache_options: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_resolve_cred_def(command_handle, vdr_handle, fqcreddef, cache_options, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_resolve_cred_def(command_handle: CommandHandle, vdr_handle: VdrHandle, fqcreddef: &str, cache_options: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let fqcreddef = c_str!(fqcreddef);
    let cache_options = c_str!(cache_options);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_resolve_cred_def(command_handle, vdr_handle, fqcreddef.as_ptr(), cache_options.as_ptr(), cb)
    })
}

pub fn vdr_prepare_did(vdr_handle: VdrHandle, txn_specific_params: &str, submitter_did: &str, endorser: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_prepare_did(command_handle, vdr_handle, txn_specific_params, submitter_did, endorser, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_prepare_did(command_handle: CommandHandle, vdr_handle: VdrHandle, txn_specific_params: &str, submitter_did: &str, endorser: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let txn_specific_params = c_str!(txn_specific_params);
    let submitter_did = c_str!(submitter_did);
    let endorser = c_str!(endorser);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_prepare_did(command_handle, vdr_handle, txn_specific_params.as_ptr(), submitter_did.as_ptr(), endorser.as_ptr(), cb)
    })
}

pub fn vdr_prepare_schema(vdr_handle: VdrHandle, txn_specific_params: &str, submitter_schema: &str, endorser: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_prepare_schema(command_handle, vdr_handle, txn_specific_params, submitter_schema, endorser, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_prepare_schema(command_handle: CommandHandle, vdr_handle: VdrHandle, txn_specific_params: &str, submitter_schema: &str, endorser: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let txn_specific_params = c_str!(txn_specific_params);
    let submitter_schema = c_str!(submitter_schema);
    let endorser = c_str!(endorser);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_prepare_schema(command_handle, vdr_handle, txn_specific_params.as_ptr(), submitter_schema.as_ptr(), endorser.as_ptr(), cb)
    })
}

pub fn vdr_prepare_cred_def(vdr_handle: VdrHandle, txn_specific_params: &str, submitter_cred_def: &str, endorser: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_prepare_cred_def(command_handle, vdr_handle, txn_specific_params, submitter_cred_def, endorser, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_prepare_cred_def(command_handle: CommandHandle, vdr_handle: VdrHandle, txn_specific_params: &str, submitter_cred_def: &str, endorser: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let txn_specific_params = c_str!(txn_specific_params);
    let submitter_cred_def = c_str!(submitter_cred_def);
    let endorser = c_str!(endorser);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_prepare_cred_def(command_handle, vdr_handle, txn_specific_params.as_ptr(), submitter_cred_def.as_ptr(), endorser.as_ptr(), cb)
    })
}

pub fn vdr_submit_txn(vdr_hanlde: VdrHandle, prepared_txn: &str, signature: &[u8], endorsement: &[u8]) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _vdr_submit_txn(command_handle, vdr_hanlde, prepared_txn, signature, endorsement, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _vdr_submit_txn(command_handle: CommandHandle, vdr_hanlde: VdrHandle, prepared_txn: &str, signature: &[u8], endorsement: &[u8], cb: Option<ResponseStringCB>) -> ErrorCode {
    let prepared_txn = c_str!(prepared_txn);

    ErrorCode::from(unsafe {
        vdr::indy_vdr_submit_txn(command_handle, vdr_hanlde, prepared_txn.as_ptr(),
                                 signature.as_ptr() as *const u8, signature.len() as u32,
                                 endorsement.as_ptr() as *const u8, endorsement.len() as u32, cb)
    })
}
