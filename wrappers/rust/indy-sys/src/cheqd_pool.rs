use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_cheqd_pool_add(
        command_handle: CommandHandle,
        alias: CString,
        rpc_address: CString,
        chain_id: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cheqd_pool_get_config(
        command_handle: CommandHandle,
        alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cheqd_pool_broadcast_tx_commit(
        command_handle: CommandHandle,
        pool_alias: CString,
        signed_tx_raw: BString,
        signed_tx_len: u32,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cheqd_pool_abci_query(
        command_handle: CommandHandle,
        pool_alias: CString,
        req_json: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cheqd_pool_abci_info(
        command_handle: CommandHandle,
        pool_alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}
