use {ErrorCode, IndyError};

use std::ffi::CString;

use futures::Future;

use ffi::cosmos_keys;
use ffi::{ResponseKeyInfoCB};

use utils::callbacks::{ClosureHandler, ResultHandler};
use {CommandHandle};

pub fn add_random(alias: &str) -> Box<dyn Future<Item=(String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _add_random(command_handle, alias, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _add_random(command_handle: CommandHandle, alias: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let alias = c_str!(alias);

    ErrorCode::from(unsafe { cosmos_keys::indy_add_random(command_handle, alias.as_ptr(), cb) })
}