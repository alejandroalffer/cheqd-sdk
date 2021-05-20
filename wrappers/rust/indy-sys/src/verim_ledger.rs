use super::*;

use {CString, CommandHandle, Error};

extern "C" {
    pub fn indy_build_msg_create_nym(
        command_handle: CommandHandle,
        did: CString,
        creator: CString,
        verkey: CString,
        alias: CString,
        role: CString,
        cb: Option<ResponseSliceCB>,
    ) -> Error;

    pub fn indy_build_msg_update_nym(
        command_handle: CommandHandle,
        did: CString,
        creator: CString,
        verkey: CString,
        alias: CString,
        role: CString,
        id: u64,
        cb: Option<ResponseSliceCB>,
    ) -> Error;

    pub fn indy_build_msg_delete_nym(
        command_handle: CommandHandle,
        creator: CString,
        id: u64,
        cb: Option<ResponseSliceCB>,
    ) -> Error;
}