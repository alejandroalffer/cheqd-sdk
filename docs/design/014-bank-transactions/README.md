# Bank transactions

Bank transactions allow exchange coins between accounts and get balance info.

## Goals and ideas

* Bank transactions should provide ability to perform following operations:
  * Send coins to other account
  * Get balance account
  
## Methods

As write above, bank transactions include method for *sending coins to other account* (`bank_build_msg_send`) and method for *get balance account* (`bank_build_query_balance`).

### bank_build_msg_send

*bank_build_msg_send* - method for sending coins between accounts. This operation has 5 steps: `create message for sending coins tx -> build tx -> signing -> broadcasting tx -> parse response`.

#### Create message for sending coins tx

Create message for building transaction. *Messages* (or sdk.Msgs) are module-specific objects that trigger state transitions within the scope of the module they belong to. Other words  messages contain the information for state transition logic.

* Required params:
  * `from` - address of sender coins (`String`)
  * `to` - address of getter coins (`String`)
  * `amount` - amount of coins for sending (`String`)
  * `denom` - denomination of coins (`String`)

* Response is `MsgSend` after converting to bytes.

Structure of `MsgSend`:

```Rust
pub struct MsgSend {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Coin>,
}
```

Example response (before converting to bytes):
```
{
  from_address: "cosmos1fknpjldck6n3v2wu86arpz8xjnfc60f99ylcjd",
  to_address: "cosmos1pvnjjy3vz0ga6hexv32gdxydzxth7f86mekcpg"
  amount: [Coin { denom: "cheq", amount: "100" }]
}
```

#### Build tx

Build transaction from message. This operation has inner implementation - it passes params to `TxBuilder`. `TxBuilder` contains data closely related with the generation of transactions, which an end-user can freely set to generate the desired transaction. In short, `build tx` return specific transaction object from incoming specific message.

* Required params:
  * `pool_alias` - alias of working pool (`String`)
  * `sender_public_key` - sender pubkey (`String`)
  * `msg` - the array of messages included in the transaction. (`&[u8]`)
  * `account_number` - the account number of the account in state. (u64)
  * `sequence_number` - sequence number of tx (u64)
  * `max_gas` - limits the amount of gas that can be used in a block. The default value is `-1`, meaning no limit, or that the concept of gas is meaningless. (`u64`)
  * `max_coin_amount` - the maximum amount the user is willing to pay in fees. (`u64`)
  * `coin_denom` - denomination for fees. (`String`)
  * `timeout_height` - block height until which the transaction is valid. (`u64`)
  * `memo` - a note or comment to send with the transaction. Can be empty string. (`String`)

* Response is `SignDoc`.

Structure of `SignDoc`:
```Rust
pub struct SignDoc {
    /// `body_bytes` is protobuf serialization of a transaction [`Body`] that matches the
    /// representation in a [`Raw`] transaction.
    pub body_bytes: Vec<u8>,

    /// `auth_info_bytes` is a protobuf serialization of an [`AuthInfo`] that matches the
    /// representation in a [`Raw`].
    pub auth_info_bytes: Vec<u8>,

    /// `chain_id` is the unique identifier of the chain this transaction targets.
    ///
    /// It prevents signed transactions from being used on another chain by an
    /// attacker.
    pub chain_id: String,

    /// `account_number` is the account number of the account in state
    pub account_number: AccountNumber,
}
```

Example response
```
SignDoc { 
  body_bytes: [
    10, 139, 1, 10, 28, 47, 99, 111, 115, 109, 111, 115, 46, 98, 97, 110, 107, 46, 118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 83, 101, 110, 100, 18, 107, 10, 45, 99, 111, 115, 109, 111, 115, 49, 102, 107, 110, 112, 106, 108, 100, 99, 107, 54, 110, 51, 118, 50, 119, 117, 56, 54, 97, 114, 112, 122, 56, 120, 106, 110, 102, 99, 54, 48, 102, 57, 57, 121, 108, 99, 106, 100, 18, 45, 99, 111, 115, 109, 111, 115, 49, 110, 48, 57, 56, 100, 53, 99, 114, 51, 106, 122, 48, 109, 48, 114, 116, 120, 109, 102, 100, 51, 116, 110, 55, 106, 115, 55, 104, 56, 116, 99, 52, 109, 53, 118, 50, 107, 107, 26, 11, 10, 4, 99, 104, 101, 113, 18, 3, 49, 48, 48, 18, 4, 109, 101, 109, 111, 24, 233, 53], auth_info_bytes: [10, 80, 10, 70, 10, 31, 47, 99, 111, 115, 109, 111, 115, 46, 99, 114, 121, 112, 116, 111, 46, 115, 101, 99, 112, 50, 53, 54, 107, 49, 46, 80, 117, 98, 75, 101, 121, 18, 35, 10, 33, 3, 47, 165, 11, 249, 173, 204, 98, 247, 253, 79, 173, 94, 227, 12, 159, 27, 159, 176, 164, 57, 159, 220, 135, 67, 30, 191, 225, 72, 188, 85, 127, 125, 18, 4, 10, 2, 8, 1, 24, 10, 18, 15, 10, 9, 10, 4, 99, 104, 101, 113, 18, 1, 48, 16, 224, 167, 18
  ],
  chain_id: "cheqd",
  account_number: 2
}
```

#### Sign tx
This necessary step for confirm tx. Notice, tx is passed with converting to bytes.

* Required params:
  * wallet_handle - (`WalletHandle`)
  * key_alias - key alias of wallet (`String`)
  * tx - transaction for signing (`&[u8]`)

Response is `TxRaw`. `TxRaw` is a variant of Tx that pins the signer's exact binary representation of body and auth_info. This is used for `signing`, `broadcasting` and `verification`. The binary serialize(tx: TxRaw) is stored in Tendermint and the hash sha256(serialize(tx: TxRaw)) becomes the "txhash", commonly used as the transaction ID.

```Rust
pub struct TxRaw {
    /// body_bytes is a protobuf serialization of a TxBody that matches the
    /// representation in SignDoc.
    #[prost(bytes = "vec", tag = "1")]
    pub body_bytes: Vec<u8>,

    /// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
    /// representation in SignDoc.
    #[prost(bytes = "vec", tag = "2")]
    pub auth_info_bytes: Vec<u8>,

    /// signatures is a list of signatures that matches the length and order of
    /// AuthInfo's signer_infos to allow connecting signature meta information like
    /// public key and signing mode by position.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub signatures: Vec<Vec<u8>>,
}
```

Example response:
```
Raw(TxRaw {
  body_bytes: [
    10, 139, 1, 10, 28, 47, 99, 111, 115, 109, 111, 115, 46, 98, 97, 110, 107, 46, 118, 49, 98, 101, 116, 97, 49, 46, 77, 115, 103, 83, 101, 110, 100, 18, 107, 10, 45, 99, 111, 115, 109, 111, 115, 49, 102, 107, 110, 112, 106, 108, 100, 99, 107, 54, 110, 51, 118, 50, 119, 117, 56, 54, 97, 114, 112, 122, 56, 120, 106, 110, 102, 99, 54, 48, 102, 57, 57, 121, 108, 99, 106, 100, 18, 45, 99, 111, 115, 109, 111, 115, 49, 101, 115, 104, 52, 114, 119, 55, 112, 101, 120, 101, 99, 97, 122, 103, 121, 52, 55, 55, 55, 50, 52, 101, 108, 48, 119, 116, 101, 106, 48, 56, 110, 113, 108, 103, 120, 106, 103, 26, 11, 10, 4, 99, 104, 101, 113, 18, 3, 49, 48, 48, 18, 4, 109, 101, 109, 111, 24, 186, 59
    ],
  auth_info_bytes: [
      10, 80, 10, 70, 10, 31, 47, 99, 111, 115, 109, 111, 115, 46, 99, 114, 121, 112, 116, 111, 46, 115, 101, 99, 112, 50, 53, 54, 107, 49, 46, 80, 117, 98, 75, 101, 121, 18, 35, 10, 33, 3, 47, 165, 11, 249, 173, 204, 98, 247, 253, 79, 173, 94, 227, 12, 159, 27, 159, 176, 164, 57, 159, 220, 135, 67, 30, 191, 225, 72, 188, 85, 127, 125, 18, 4, 10, 2, 8, 1, 24, 11, 18, 15, 10, 9, 10, 4, 99, 104, 101, 113, 18, 1, 48, 16, 224, 167, 18
    ],
  signatures: [
    [
      162, 202, 89, 100, 100, 63, 20, 208, 68, 217, 130, 98, 99, 124, 211, 202, 158, 54, 179, 242, 118, 102, 144, 191, 93, 66, 204, 208, 129, 22, 97, 84, 58, 86, 247, 14, 168, 60, 156, 230, 251, 160, 108, 107, 89, 37, 174, 150, 34, 151, 253, 35, 198, 8, 17, 26, 130, 103, 196, 141, 33, 19, 16, 24
    ]
  ]
})
```

#### Broadcast tx

Broadcast the signed transaction to the network. This is possible because client connects to the node's RPC endpoint.

* Required params:
  * pool_alias - alias of pool (`String`)
  * tx - signed transaction for broadcasting(`Raw`)

* Response `TxResult` as String with following fields:
  * `check_tx` - checks the transaction without executing it. When a new transaction is added to the Tendermint Core, it will ask the application to check it (validate the format, signatures, etc.).
    * `check_tx::code` - if Code != 0, it will be rejected from the mempool and hence not broadcasted to other peers and not included in a proposal block.
    * `check_tx::data` - contains the result of the CheckTx transaction execution, if any. It is semantically meaningless to Tendermint.
    * `check_tx::gas_wanted` - is the maximum amount of gas the sender of a tx is willing to use.
    * `check_tx::gas_used` - is how much it actually used.
    * `check_tx::events` - include any events for the execution, though since the transaction has not been committed yet, they are /effectively ignored by Tendermint.
    * `check_tx::codespace` - the codespace is a namespace for the Code.
  * `deliver_tx` - returns a abci.Result, which includes a Code, Data, and Log. Tendermint sends the DeliverTx requests asynchronously but in order, and relies on the underlying socket protocol (ie. TCP) to ensure they are received by the app in order. They have already been ordered in the global consensus by the Tendermint protocol.
    * `deliver_tx::gas_wanted` - is the maximum amount of gas the sender of a tx is willing to use,
    * `deliver_tx::gas_used` - is how much it actually used,
    * `deliver_tx::data` - contains the result of the CheckTx transaction execution, if any. It is semantically meaningless to Tendermint.
    * `deliver_tx::events` - include any events for the execution, which Tendermint will use to index the transaction by. This allows transactions to be queried according to what events took place during their execution.
    * `deliver_tx::hash:transaction::Hash` - hash of transaction.
    * `deliver_tx::height` - height is a monotonically increasing data type that can be compared against another Height for the purposes of updating and freezing clients.

Example response:
```
Response {
   check_tx: TxResult {
      code: 0,
      data: None,
      log: "",
      info: "",
      gas_wanted: 0,
      gas_used: 0,
      events: [
      ],
      codespace: ""
   },
   deliver_tx: TxResult {
      code: 0,
      data: Some(Data([
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
      info: "",
      gas_wanted: 0,
      gas_used: 0,
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
                  key: Key("amount"),
                  value: Value("100cheq")
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
      codespace: ""
   },
   hash: "1B3B00849B4D50E8FCCF50193E35FD6CA5FD4686ED6AD8F847AC8C5E466CFD3E",
   height: 353
}
```

#### Parse response

Parse response from `TxResult` and return parsed response.

* Required params:
  * `resp` - `TxResult` as string (`String`)

* Response is empty structure `{}` as String or `IndyError`

Example response:
```
{}
```

### bank_build_query_balance

 Create query for sending to AbciQuery.

* Params:
  * `address` - address of sender coins (String)
  * `denom` - denomination of coins (String)


* Response fields:
  * `response` - query the application for some information. Note that calls to Query are not replicated across nodes, but rather query the local node's state - hence they may return stale reads. For reads that require consensus, use a transaction.
    * `code` - if Code != 0, it will be rejected from the mempool and hence not broadcasted to other peers and not included in a proposal block.
    * `proofs` - The Tendermint block header includes a number of hashes, each providing an anchor for some type of proof about the blockchain. 
    * `height` - height is a monotonically increasing data type that can be compared against another Height for the purposes of updating and freezing clients.
    * `codespace` - the codespace is a namespace for the Code.

Example response:

```
Response {
   response: AbciQuery {
      code: Ok,
      log: Log(""),
      info:"",
      index: 0,
      key: [],
      value:[
         10,
         17,
         10,
         4,
         99,
         104,
         101,
         113,
         18,
         9,
         49,
         48,
         48,
         48,
         48,
         48,
         52,
         48,
         48
      ],
      proof: None,
      height": block::Height(352),
      codespace: ""
   }
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
