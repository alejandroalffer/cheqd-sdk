use super::*;

use crate::{BString, CString, CommandHandle, Error};

extern "C" {
    pub fn indy_vdr_create(
        command_handle: CommandHandle,
        cb: Option<ResponseVdrHandleCB>,
    ) -> Error;
    
    pub fn indy_vdr_register_indy_ledger(
        command_handle: CommandHandle,
        handle: VdrHandle,
        namespace_list: CString,
        genesis_txn_data: CString,
        taa_config: CString,
        cb: Option<ResponseEmptyCB>,
    ) -> Error;
    
    pub fn indy_vdr_register_cheqd_ledger(
        command_handle: CommandHandle,
        handle: VdrHandle,
        namespace_list: CString,
        chain_id: CString,
        node_addrs_list: CString,
        cb: Option<ResponseEmptyCB>,
    ) -> Error;
    
    pub fn indy_vdr_ping(
        command_handle: CommandHandle,
        handle: VdrHandle,
        namespace_list: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
    
    pub fn indy_vdr_cleanup(
        command_handle: CommandHandle,
        handle: VdrHandle,
        cb: Option<ResponseEmptyCB>,
    ) -> Error;
    
    pub fn indy_vdr_resolve_did(
        command_handle: CommandHandle,
        handle: VdrHandle,
        fqdid: CString,
        cache_options: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
    
    pub fn indy_vdr_resolve_schema(
        command_handle: CommandHandle,
        handle: VdrHandle,
        fqdid: CString,
        cache_options: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
    
    pub fn indy_vdr_resolve_cred_def(
        command_handle: CommandHandle,
        handle: VdrHandle,
        fqcreddef: CString,
        cache_options: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
    
    pub fn indy_vdr_prepare_did(
        command_handle: CommandHandle,
        handle: VdrHandle,
        txn_specific_params: CString,
        submitter_did: CString,
        endorser: CString,
        cb: Option<ResponsePreparedTxnCB>,
    ) -> Error;
    
    pub fn indy_vdr_prepare_schema(
        command_handle: CommandHandle,
        handle: VdrHandle,
        txn_specific_params: CString,
        submitter_did: CString,
        endorser: CString,
        cb: Option<ResponsePreparedTxnCB>,
    ) -> Error;

    pub fn indy_vdr_prepare_cred_def(
        command_handle: CommandHandle,
        handle: VdrHandle,
        txn_specific_params: CString,
        submitter_did: CString,
        endorser: CString,
        cb: Option<ResponsePreparedTxnCB>,
    ) -> Error;

    pub fn indy_vdr_submit_txn(
        command_handle: CommandHandle,
        handle: VdrHandle,
        context: CString,
        signature_spec: CString,
        bytes_to_sign_raw: BString,
        bytes_to_sign_len: u32,
        endorsement_spec: CString,
        signature_raw: BString,
        signature_len: u32,
        endorsement_raw: BString,
        endorsement_len: u32,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}
