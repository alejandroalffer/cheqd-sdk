# Bank transactions

Bank transactions allow exchange coins between accounts and get balance info.

## Goals and ideas

* Bank transactions should provide ability to perform following operations:
  * Send coins to other account
  * Get balance for account
  
## Methods

### bank_build_msg_send

* Params:
  * `from` - address of sender coins (String)
  * `to` - address of getter coins (String)
  * `amount` - amount of coins for sending (String)
  * `denom` - denomination of coins (String)

* Response fields:
  * `check_tx` - checks the transaction without executing it. When a new transaction is added to the Tendermint Core, it will ask the application to check it (validate the format, signatures, etc.).
    * `check_tx::code` - if Code != 0, it will be rejected from the mempool and hence not broadcasted to other peers and not included in a proposal block.
    * `check_tx::data` - contains the result of the CheckTx transaction execution, if any. It is semantically meaningless to Tendermint.
    * `check_tx::gas_wanted` - is the maximum amount of gas the sender of a tx is willing to use.
    * `check_tx::gas_used` - is how much it actually used.
    * `check_tx::events` - include any events for the execution, though since the transaction has not been committed yet, they are /effectively ignored by Tendermint.
    * `check_tx::codespace` - The codespace is a namespace for the Code.
  * `deliver_tx` - returns a abci.Result, which includes a Code, Data, and Log. Tendermint sends the DeliverTx requests asynchronously but in order, and relies on the underlying socket protocol (ie. TCP) to ensure they are received by the app in order. They have already been ordered in the global consensus by the Tendermint protocol.
    * `deliver_tx::gas_wanted` - is the maximum amount of gas the sender of a tx is willing to use,
    * `deliver_tx::gas_used` - is how much it actually used,
    * `deliver_tx::data` - contains the result of the CheckTx transaction execution, if any. It is semantically meaningless to Tendermint.
    * `deliver_tx::events` - include any events for the execution, which Tendermint will use to index the transaction by. This allows transactions to be queried according to what events took place during their execution.
    * `deliver_tx::hash:transaction::Hash` - hash of transaction.
    * `deliver_tx::height` - Height is a monotonically increasing data type that can be compared against another Height for the purposes of updating and freezing clients.

Example response:

```Rust
Response {
   check_tx: TxResult {
      code: Ok,
      data: None,
      log: Log("[]"),
      info: Info(""),
      gas_wanted: Gas(0),
      gas_used: Gas(0),
      events: [
      ],
      codespace: Codespace("")
   },
   deliver_tx: TxResult {
      code:Ok,
      data:Some(Data([
         10,
         6,
         10,
         4,
         115,
         101,
         110,
         100
      ])),
      log:Log("[{\"events\":[{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"send\"},{\"key\":\"sender\",\"value\":\"cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd\"},{\"key\":\"module\",\"value\":\"bank\"}]},{\"type\":\"transfer\",\"attributes\":[{\"key\":\"recipient\",\"value\":\"cosmos1pvnjjy3vz0ga6hexv32gdxydzxth7f86mekcpg\"},{\"key\":\"sender\",\"value\":\"cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd\"},{\"key\":\"amount\",\"value\":\"100cheq\"}]}]}]"),
      info:Info(""),
      gas_wanted:Gas(0),
      gas_used:Gas(0),
      events: [
         Event {
            type_str: "message",
            attributes:[
               Tag {
                  key: Key("action"),
                  value: Value("send")
               }
            ]
         },
         Event {
            type_str: "transfer",
            attributes: [
               Tag {
                  key:Key("recipient"),
                  value:Value("cosmos1pvnjjy3vz0ga6hexv32gdxydzxth7f86mekcpg")
               },
               Tag {
                  key:Key("sender"),
                  value:Value("cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd")
               },
               Tag {
                  key:Key("amount"),
                  value:Value("100cheq")
               }
            ]
         },
         Event {
            type_str: "message",
            attributes:[
               Tag {
                  key: Key("sender"),
                  value: Value("cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd")
               }
            ]
         },
         Event {
            type_str: "message",
            attributes: [
               Tag {
                  key: Key("module"),
                  value: Value("bank")
               }
            ]
         }
      ], 
      codespace:Codespace("")
   },
   hash:transaction::Hash(A04F501BB9A6E82A9254E08265902C410FEED102D02AB13D635C0228B6B85B98),
   height:block::Height(353)
}
```

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
