use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_add_random(
        command_handle: CommandHandle,
        alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_add_from_mnemonic(
        command_handle: CommandHandle,
        alias: CString,
        mnemonic: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_key_info(
        command_handle: CommandHandle,
        alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}