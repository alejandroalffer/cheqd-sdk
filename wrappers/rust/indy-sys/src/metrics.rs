use super::*;

use crate::{Error, CommandHandle};

extern {
    pub fn indy_collect_metrics(command_handle: CommandHandle,
                                cb: Option<ResponseStringCB>) -> Error;
}
