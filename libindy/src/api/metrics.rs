use indy_api_types::{ErrorCode, CommandHandle};
use crate::commands::Locator;
use indy_utils::ctypes;
use libc::c_char;

/// Collect metrics.
///
/// #Returns
/// Map in the JSON format. Where keys are names of metrics.
///
/// #Errors
/// Common*
#[no_mangle]
pub extern fn indy_collect_metrics(command_handle: CommandHandle,
                                   cb: Option<extern fn(command_handle_: CommandHandle,
                                                        err: ErrorCode,
                                                        metrics_json: *const c_char)>) -> ErrorCode {
    trace!("indy_collect_metrics: >>> command_handle: {:?}, cb: {:?}",
           command_handle, cb);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let (executor, controller) = {
        let locator = Locator::instance();
        let executor = locator.executor.clone();
        let controller = locator.metrics_command_executor.clone();
        (executor, controller)
    };

    executor.spawn_ok(async move {
        let res = controller.collect().await;
        let (err, metrics) = prepare_result_1!(res, String::new());

        trace!("indy_collect_metrics ? err {:?} metrics {:?}", err, metrics);

        let did = ctypes::string_to_cstring(metrics);
        cb(command_handle, err, did.as_ptr())
    });

    let res = ErrorCode::Success;
    trace!("indy_collect_metrics: <<< res: {:?}", res);
    res
}
