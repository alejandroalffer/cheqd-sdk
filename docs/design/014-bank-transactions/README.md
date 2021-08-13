# Bank transactions

Bank transactions allow exchange coins between accounts and get balance info.

## Goals and ideas

* Bank transactions should provide ability to perform following operations:
  * Send coins to other account
  * Get balance for account
  
## Public API

```Rust
/// Send coins to other account.
/// 
/// #Params
/// command_handle: command handle to map callback to caller context.
/// from: address of sender coins
/// to: address of getter coins
/// amount: Amount of coins for sending
/// denom: Denomination of coins
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Success or error message.
#[no_mangle]
pub extern fn indy_cheqd_ledger_bank_build_msg_send(
    command_handle: CommandHandle,
    from: *const c_char,
    to: *const c_char,
    amount: *const c_char,
    denom: *const c_char,
    cb: Option<
        extern "C" fn(
            command_handle_: CommandHandle,
            err: ErrorCode,
            msg_raw: *const u8,
            msg_len: u32,
        ),
    >,
    ) -> ErrorCode {
}

/// Parse response for send coins tx.
/// 
/// #Params
/// command_handle: command handle to map callback to caller context.
/// commit_resp: response for send coins tx.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Success or error message.
#[no_mangle]
pub extern fn indy_cheqd_ledger_bank_parse_msg_send_resp(
  command_handle: CommandHandle,
  commit_resp: *const c_char,
  cb: Option<
      extern "C" fn(
          command_handle_: CommandHandle,
          err: ErrorCode,
          msg_resp: *const c_char
        ),
    >
  ) -> ErrorCode {
}

/// Get balance of account.
/// 
/// #Params
/// command_handle: command handle to map callback to caller context.
/// address: address of account which need to get.
/// denom: currency of balance for getting.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Success or error message.
#[no_mangle]
pub extern fn indy_cheqd_ledger_bank_build_query_balance(
  command_handle: CommandHandle,
  address: *const c_char,
  denom: *const c_char,
  cb: Option<
      extern "C" fn(
        command_handle_: CommandHandle,
        err: ErrorCode,
        msg_resp: *const c_char
      )
    >
  ) -> ErrorCode {

}

/// Parse response for get balance tx.
/// 
/// #Params
/// command_handle: command handle to map callback to caller context.
/// commit_resp: response for get balance tx.
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// Success or error message.
#[no_mangle]
pub extern fn indy_cheqd_ledger_bank_parse_query_balance_resp(
  command_handle: CommandHandle,
  commit_resp: *const c_char,
  cb: Option<
      extern "C" fn(
        command_handle_: CommandHandle,
        err: ErrorCode,
        msg_resp: *const c_char
      ),
    >
  ) -> ErrorCode {

}

```
