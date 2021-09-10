use crate::{CString, CommandHandle, Error};
use crate::{ResponseSliceCB, ResponseStringCB};

extern "C" {
    pub fn indy_cheqd_ledger_bank_build_msg_send(
        command_handle: CommandHandle,
        from: CString,
        to: CString,
        amount: CString,
        denom: CString,
        cb: Option<ResponseSliceCB>,
    ) -> Error;

    pub fn indy_cheqd_ledger_bank_parse_msg_send_resp(
        command_handle: CommandHandle,
        commit_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cheqd_ledger_bank_build_query_balance(
        command_handle: CommandHandle,
        address: CString,
        denom: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_cheqd_ledger_bank_parse_query_balance_resp(
        command_handle: CommandHandle,
        commit_resp: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}
