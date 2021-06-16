#[macro_use]
extern crate serde_derive;

extern crate libc;

pub mod anoncreds;
pub mod blob_storage;
pub mod cache;

pub mod crypto;
pub mod did;
pub mod ledger;
pub mod logger;
pub mod metrics;
pub mod non_secrets;
pub mod pairwise;
pub mod payments;
pub mod pool;
pub mod wallet;
pub mod verim_ledger;
pub mod verim_keys;
pub mod cosmos_ledger;
pub mod verim_pool;

use libc::{c_char, c_void};

pub type CVoid = c_void;
pub type BString = *const u8;
pub type CString = *const c_char;

#[repr(transparent)]
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct WalletHandle(pub i32);
pub const INVALID_WALLET_HANDLE: WalletHandle = WalletHandle(0);

pub type PoolHandle = i32;
pub const INVALID_POOL_HANDLE: PoolHandle = 0;

pub type CommandHandle = i32;
pub const INVALID_COMMAND_HANDLE: CommandHandle = 0;

//pub type Handle = i32;
pub type IndyHandle = i32;
pub type SearchHandle = i32;
pub type RecordHandle = i32;
pub type TailWriterHandle = i32;
pub type StorageHandle = i32;
pub type BlobStorageReaderHandle = i32;
pub type BlobStorageReaderCfgHandle = i32;
pub type MetadataHandle = i32;
pub type Timeout = i32;
pub type TailsWriterHandle = i32;

pub type Error = i32;

pub type ResponseEmptyCB = extern "C" fn(xcommand_handle: CommandHandle, err: Error);
pub type ResponseBoolCB = extern "C" fn(xcommand_handle: CommandHandle, err: Error, bool1: bool);
pub type ResponseI32CB =
    extern "C" fn(xcommand_handle: CommandHandle, err: Error, handle: IndyHandle);
pub type ResponseWalletHandleCB =
    extern "C" fn(xcommand_handle: CommandHandle, err: Error, handle: WalletHandle);
pub type ResponseI32UsizeCB = extern "C" fn(
    xcommand_handle: CommandHandle,
    err: Error,
    handle: IndyHandle,
    total_count: usize,
);
pub type ResponseStringCB =
    extern "C" fn(xcommand_handle: CommandHandle, err: Error, str1: CString);
pub type ResponseStringStringCB =
    extern "C" fn(xcommand_handle: CommandHandle, err: Error, str1: CString, str2: CString);
pub type ResponseStringStringStringCB = extern "C" fn(
    xcommand_handle: CommandHandle,
    err: Error,
    str1: CString,
    str2: CString,
    str3: CString,
);
pub type ResponseSliceCB =
    extern "C" fn(xcommand_handle: CommandHandle, err: Error, raw: BString, len: u32);
pub type ResponseStringSliceCB = extern "C" fn(
    xcommand_handle: CommandHandle,
    err: Error,
    str1: CString,
    raw: BString,
    len: u32,
);
pub type ResponseStringStringU64CB = extern "C" fn(
    xcommand_handle: CommandHandle,
    err: Error,
    arg1: CString,
    arg2: CString,
    arg3: u64,
);
pub type ResponseStringI64CB =
    extern "C" fn(xcommand_handle: CommandHandle, err: Error, arg1: CString, arg3: i64);

extern "C" {
    pub fn indy_set_runtime_config(config: CString) -> Error;

    pub fn indy_get_current_error(error_json_p: *mut CString);
}
