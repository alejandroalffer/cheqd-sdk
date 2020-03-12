use libc::c_char;
use utils::libindy::wallet;

extern {
    fn indy_pack_message(command_handle: i32,
                         wallet_handle: i32,
                         message: *const u8,
                         message_len: u32,
                         receiver_keys: *const c_char,
                         sender: *const c_char,
                         cb: Option<extern fn(xcommand_handle: i32, err: i32, jwe_data: *const u8, jwe_len: u32)>) -> i32;

    fn indy_unpack_message(command_handle: i32,
                           wallet_handle: i32,
                           jwe_data: *const u8,
                           jwe_len: u32,
                           cb: Option<extern fn(xcommand_handle: i32, err: i32, res_json_data: *const u8, res_json_len: u32 )>) -> i32;
}

#[no_mangle]
pub extern fn vcx_pack_message(command_handle: i32,
                               _wallet_handle: i32, //ignored
                               message: *const u8,
                               message_len: u32,
                               receiver_keys: *const c_char,
                               sender: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: i32, jwe_data: *const u8, jwe_len: u32)>) -> i32 {
    unsafe {
        indy_pack_message(command_handle, wallet::get_wallet_handle().0, message, message_len, receiver_keys, sender, cb)
    }
}

#[no_mangle]
pub extern fn vcx_unpack_message(command_handle: i32,
                                 _wallet_handle: i32, //ignored
                                 jwe_data: *const u8,
                                 jwe_len: u32,
                                 cb: Option<extern fn(xcommand_handle: i32, err: i32, res_json_data : *const u8, res_json_len : u32)>) -> i32 {
    unsafe {
        indy_unpack_message(command_handle, wallet::get_wallet_handle().0, jwe_data, jwe_len, cb)
    }
}
