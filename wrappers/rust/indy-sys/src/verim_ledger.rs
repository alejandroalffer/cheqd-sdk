use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_verim_ledger_build_msg_create_nym(
        command_handle: CommandHandle,
        did: CString,
        creator: CString,
        verkey: CString,
        alias: CString,
        role: CString,
        cb: Option<ResponseSliceCB>,
    ) -> Error;

    pub fn indy_verim_ledger_parse_msg_create_nym_resp(
        command_handle: CommandHandle,
        commit_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_build_msg_update_nym(
        command_handle: CommandHandle,
        did: CString,
        creator: CString,
        verkey: CString,
        alias: CString,
        role: CString,
        id: u64,
        cb: Option<ResponseSliceCB>,
    ) -> Error;

    pub fn indy_verim_ledger_parse_msg_update_nym_resp(
        command_handle: CommandHandle,
        commit_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_build_msg_delete_nym(
        command_handle: CommandHandle,
        creator: CString,
        id: u64,
        cb: Option<ResponseSliceCB>,
    ) -> Error;

    pub fn indy_verim_ledger_parse_msg_delete_nym_resp(
        command_handle: CommandHandle,
        commit_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_build_query_get_nym(
        command_handle: CommandHandle,
        id: u64,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_parse_query_get_nym_resp(
        command_handle: CommandHandle,
        query_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_build_query_all_nym(
        command_handle: CommandHandle,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_parse_query_all_nym_resp(
        command_handle: CommandHandle,
        query_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_build_query_cosmos_auth_account(
        command_handle: CommandHandle,
        address: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_ledger_parse_query_cosmos_auth_account_resp(
        command_handle: CommandHandle,
        query_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}
