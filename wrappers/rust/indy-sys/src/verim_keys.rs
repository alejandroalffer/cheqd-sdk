use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_verim_keys_add_random(
        command_handle: CommandHandle,
        alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_keys_add_from_mnemonic(
        command_handle: CommandHandle,
        alias: CString,
        mnemonic: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_keys_get_info(
        command_handle: CommandHandle,
        alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;

    pub fn indy_verim_keys_sign(
        command_handle: CommandHandle,
        alias: CString,
        tx_raw: BString,
        tx_len: u32,
        cb: Option<ResponseSliceCB>,
    ) -> Error;
}
