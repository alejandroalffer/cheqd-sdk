use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_add_random(
        command_handle: CommandHandle,
        alias: CString,
        cb: Option<ResponseStringCB>,
    ) -> Error;
}