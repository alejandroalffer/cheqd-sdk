use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_cosmos_pool_add(
        command_handle: CommandHandle,
        alias: CString,
        rpc_address: CString,
        chain_id: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cosmos_pool_get_config(
        command_handle: CommandHandle,
        alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cosmos_pool_build_tx(
        command_handle: CommandHandle,
        pool_alias: CString,
        sender_alias: CString,
        msg_raw: BString,
        msg_len: u32,
        account_number: u64,
        sequence_number: u64,
        max_gas: u64,
        max_coin_amount: u64,
        max_coin_denom: CString,
        timeout_height: u64,
        memo: CString,
        cb: Option<ResponseSliceCB>,
    ) -> Error;

    pub fn indy_cosmos_pool_broadcast_tx_commit(
        command_handle: CommandHandle,
        pool_alias: CString,
        signed_tx_raw: BString,
        signed_tx_len: u32,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cosmos_pool_abci_query(
        command_handle: CommandHandle,
        pool_alias: CString,
        req_json: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}
