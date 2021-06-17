use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_cosmos_ledger_build_tx(
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

    pub fn indy_cosmos_ledger_build_query_cosmos_auth_account(
        command_handle: CommandHandle,
        address: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_auth_parse_query_account_resp(
        command_handle: CommandHandle,
        query_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}
