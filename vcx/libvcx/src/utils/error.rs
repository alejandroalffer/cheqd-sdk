#![deny(unreachable_patterns)]
#![deny(unreachable_code)]

use std::ffi::CStr;
use std::fmt;

macro_rules! impl_error {
    ($f:ident, $($name:ident: $code:expr => $msg:expr),* $(,)?) => {
        // generate static items for converting enum variants to u32s
        $(pub static $name: Error = Error { code_num: $code, message: concat!($msg, "\0") });+;
        // resolve u32s to null-terminated strings
        fn $f(n: u32) -> &'static str {
            // NOTE: this was originally a lazily-initialized HashMap, which has
            // initialization and lookup overhead; a `match` statement like this will
            // compile down into a highly efficient jump table and avoids allocation
            match n {
                $($code => $name.message),+,
                _ => UNKNOWN_ERROR.message,
            }
        }
    }
}

// to add a new error, just add a line at the end with the following syntax:
// STATIC_NAME: code_num => message,
impl_error! {
    internal_error_c_message,
    SUCCESS: 0 => "Success",
    UNKNOWN_ERROR: 1001 => "Unknown Error",
    INVALID_CONNECTION_HANDLE: 1003 => "Invalid Connection Handle",
    INVALID_CONFIGURATION: 1004 => "Invalid Configuration",
    NOT_READY: 1005 => "Object not ready for specified action",
    INVALID_OPTION: 1007 => "Invalid Option",
    INVALID_DID: 1008 => "Invalid DID",
    INVALID_VERKEY: 1009 => "Invalid VERKEY",
    POST_MSG_FAILURE: 1010 => "Message failed in post",
    INVALID_NONCE: 1011 => "Invalid NONCE",
    INVALID_URL: 1013 => "Invalid URL",
    NOT_BASE58: 1014 => "Value needs to be base58",
    INVALID_ISSUER_CREDENTIAL_HANDLE: 1015 => "Invalid Credential Issuer Handle",
    INVALID_JSON: 1016 => "Invalid JSON string",
    INVALID_PROOF_HANDLE: 1017 => "Invalid Proof Handle",
    INVALID_CREDENTIAL_REQUEST: 1018 => "Invalid Credential Request",
    INVALID_MSGPACK: 1019 => "Invalid MessagePack",
    INVALID_AGENCY_RESPONSE: 1020 => "Error Retrieving messages from API",
    INVALID_ATTRIBUTES_STRUCTURE: 1021 => "Attributes provided to Credential Offer / Proof Request are not correct, possibly malformed",
    BIG_NUMBER_ERROR: 1022 => "Could not encode string to a big integer.",
    INVALID_PROOF: 1023 => "Proof had invalid format",
    INVALID_GENESIS_TXN_PATH: 1024 => "Must have valid genesis txn file path",
    POOL_LEDGER_CONNECT: 1025 => "Connection to Pool Ledger.",
    CREATE_POOL_CONFIG: 1026 => "Formatting for Pool Config are incorrect.",
    INVALID_PROOF_CREDENTIAL_DATA: 1027 => "The Proof received does not have valid credentials listed.",
    INVALID_PREDICATES_STRUCTURE: 1028 => "Predicates provided to create a Proof Request are not correct",
    INVALID_AGENCY_REQUEST: 1029 => "The message submitted on the Agency has an invalid format or field value",
    NO_POOL_OPEN: 1030 => "No Pool open. Can't return handle.",
    INVALID_SCHEMA: 1031 => "Schema was invalid or corrupt",
    CREATE_CREDENTIAL_DEF_ERR: 1034 => "Call to create Credential Definition failed",
    UNKNOWN_LIBINDY_ERROR: 1035 => "Unknown libindy error",
    CREDENTIAL_DEFINITION_NOT_FOUND: 1036 => "Credential Def not in valid json",
    INVALID_CREDENTIAL_DEF_HANDLE: 1037 => "Invalid Credential Definition handle",
    TIMEOUT_LIBINDY_ERROR: 1038 => "Waiting for callback timed out",
    CREDENTIAL_DEF_ALREADY_CREATED: 1039 => "Can't create, Credential Def already exists in wallet",
    INVALID_SCHEMA_SEQ_NO: 1040 => "No Schema for that schema sequence number",
    INVALID_SCHEMA_CREATION: 1041 => "Could not create schema",
    INVALID_SCHEMA_HANDLE: 1042 => "Invalid Schema Handle",
    INVALID_CREDENTIAL_OFFER: 1043 => "Invalid Credential Offerz",
    ALREADY_INITIALIZED: 1044 => "Library already initialized",
    INVALID_INVITE_DETAILS: 1045 => "Invalid invite details structure",
    INVALID_OBJ_HANDLE: 1048 => "Obj was not found with handle",
    INVALID_DISCLOSED_PROOF_HANDLE: 1049 => "Obj was not found with handle",
    SERIALIZATION_ERROR: 1050 => "Unable to serialize",
    WALLET_ALREADY_EXISTS: 1051 => "Indy wallet already exists",
    WALLET_ALREADY_OPEN: 1052 => "Indy wallet already open",
    INVALID_CREDENTIAL_HANDLE: 1053 => "Invalid credential handle",
    INVALID_CREDENTIAL_JSON: 1054 => "Invalid credential json",
    CREATE_PROOF_ERROR: 1056 => "could not create proof",
    INVALID_WALLET_HANDLE: 1057 => "Invalid Wallet or Search Handle",
    INVALID_WALLET_CREATION: 1058 => "Error Creating a wallet",
    CANNOT_DELETE_CONNECTION: 1060 => "Cannot Delete Connection. Check status of connection is appropriate to be deleted from agency.",
    CREATE_CONNECTION_ERROR: 1061 => "Could not store Connection object into the Object Cache",
    CONNECTION_ALREADY_EXISTS: 1062 => "Connection invitation has been already accepted. You have to use another invitation to set up a new connection.",
    CONNECTION_DOES_NOT_EXIST: 1063 => "Connection does not exist.",
    INSUFFICIENT_TOKEN_AMOUNT: 1064 => "Insufficient amount of tokens to process request",
    INVALID_PAYMENT_ADDRESS: 1066 => "Invalid payment address",
    INVALID_LIBINDY_PARAM: 1067 => "Parameter passed to libindy was invalid",
    MISSING_WALLET_KEY: 1069 => "Configuration is missing wallet key",
    OBJECT_CACHE_ERROR: 1070 => "Object cache error",
    NO_PAYMENT_INFORMATION: 1071 => "No payment information associated with object",
    DUPLICATE_WALLET_RECORD: 1072 => "Record already exists in the wallet",
    WALLET_RECORD_NOT_FOUND: 1073 => "Wallet record not found",
    IOERROR: 1074 => "IO Error, possibly creating a backup wallet",
    WALLET_ACCESS_FAILED: 1075 => "Attempt to open wallet with invalid credentials",
    INVALID_WALLET_IMPORT_CONFIG: 1076 => "Invalid wallet import config",
    MISSING_BACKUP_KEY: 1078 => "Missing exported backup key in config",
    WALLET_NOT_FOUND: 1079 => "Wallet Not Found",
    LIBINDY_INVALID_STRUCTURE: 1080 => "Object (json, config, key, credential and etc...) passed to libindy has invalid structure",
    INVALID_STATE: 1081 => "Object is in invalid state for requested operation",
    INVALID_LEDGER_RESPONSE: 1082 => "Invalid response from ledger for paid transaction",
    DID_ALREADY_EXISTS_IN_WALLET: 1083 => "Attempted to add a DID to wallet when that DID already exists in wallet" ,
    DUPLICATE_MASTER_SECRET: 1084 => "Attempted to add a Master Secret that already existed in wallet",
    INVALID_PROOF_REQUEST: 1086 => "Proof Request Passed into Libindy Call Was Invalid",
    MISSING_PAYMENT_METHOD: 1087 => "Configuration is missing the Payment Method parameter",
    DUPLICATE_SCHEMA: 1088 => "Duplicate Schema: Ledger Already Contains Schema For Given DID, Version, and Name Combination",
    LOGGING_ERROR: 1090 => "Logging Error",
    INVALID_REVOCATION_DETAILS: 1091 => "Invalid Revocation Details",
    INVALID_REV_ENTRY: 1092 => "Unable to Update Revocation Delta On Ledger",
    INVALID_REVOCATION_TIMESTAMP: 1093 => "Invalid Credential Revocation timestamp",
    UNKNOWN_SCHEMA_REJECTION: 1094 => "Unknown Rejection of Schema Creation, refer to libindy documentation",
    INVALID_REV_REG_DEF_CREATION: 1095 => "Failed to create Revocation Registration Definition",
    CREATE_WALLET_BACKUP: 1096 => "Failed to create Wallet Backup",
    RETRIEVE_EXPORTED_WALLET: 1097 => "Failed to retrieve exported wallet",
    RETRIEVE_DEAD_DROP: 1099 => "Failed to retrieve Dead Drop payload",
    INVALID_ATTACHMENT_ENCODING: 1100 => "Failed to decode attachment",
    ACTION_NOT_SUPPORTED: 1103 => "Action is not supported",
    INVALID_REDIRECT_DETAILS: 1104 => "Invalid redirect details structure",
    INVALID_PROOF_PROPOSAL: 1110 => "Invalid proof proposal",
    // EC 1105-1107 is reserved for proprietary forks of libVCX
    MAX_BACKUP_SIZE: 1105 => "Cloud Backup exceeds max size limit",
    NO_AGENT_INFO: 1106 => "Agent pairwise information not found",
    INVALID_PROVISION_TOKEN: 1107 => "Token provided by sponsor is invalid",
    INVALID_DID_DOC: 1108 => "The format of DIDDoc is invalid",
    MESSAGE_IS_OUT_OF_THREAD: 1109 => "The format of DIDDoc is invalid",
}

pub fn error_message(code_num: u32) -> &'static str {
    let msg = internal_error_c_message(code_num);
    &msg[..msg.len() - 1]
}


#[derive(Clone, Copy)]
pub struct Error {
    pub code_num: u32,
    pub message: &'static str,
}


pub fn error_c_message(code_num: u32) -> &'static CStr {
    let msg = internal_error_c_message(code_num);
    // SAFETY: the macro guarantees that the string literals have a null terminator;
    unsafe { CStr::from_bytes_with_nul_unchecked(msg.as_bytes()) }
}

impl Error {
    /// Returns the associated error message _without_ the null terminator
    pub fn as_str(&self) -> &'static str {
        &self.message[..self.message.len() - 1]
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = error_message(self.code_num);
        write!(f, "{}: (Error Num:{})", msg, self.code_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_error(){
        let e = &UNKNOWN_ERROR;
        assert_eq!(e.code_num, 1001);
    }

    #[test]
    fn test_display_error(){
        let msg = format!("{}",UNKNOWN_ERROR);
        assert_eq!(msg, "Unknown Error: (Error Num:1001)")
    }

    #[test]
    fn test_error_message(){
        let msg = error_message(1);
        assert_eq!(msg, "Unknown Error");

        let msg = error_message(1070);
        assert_eq!(msg, "Object cache error");
    }

    #[test]
    fn test_unknown_error(){
        assert_eq!(error_message(UNKNOWN_ERROR.code_num), UNKNOWN_ERROR.as_str());
    }

    #[test]
    fn test_success_error(){
        assert_eq!(error_message(SUCCESS.code_num), SUCCESS.as_str());
    }

    #[test]
    fn test_invalid_option_error(){
        assert_eq!(error_message(INVALID_OPTION.code_num), INVALID_OPTION.as_str());
    }

    #[test]
    fn test_error_retrieving_messages(){
        assert_eq!(error_message(INVALID_AGENCY_RESPONSE.code_num), INVALID_AGENCY_RESPONSE.as_str());
    }

    #[test]
    fn test_malformed_attributes_for_credential_offer(){
        assert_eq!(error_message(INVALID_ATTRIBUTES_STRUCTURE.code_num), INVALID_ATTRIBUTES_STRUCTURE.as_str());
    }

    #[test]
    fn test_invalid_proof_handle_error(){
        assert_eq!(error_message(INVALID_PROOF_HANDLE.code_num), INVALID_PROOF_HANDLE.as_str());
    }

    #[test]
    fn test_credential_request_incorrect_json_format_error(){
        assert_eq!(error_message(INVALID_CREDENTIAL_REQUEST.code_num), INVALID_CREDENTIAL_REQUEST.as_str());
    }

    #[test]
    fn test_error_invalid_proof() {
        assert_eq!(error_message(INVALID_PROOF.code_num), INVALID_PROOF.as_str());
    }
    #[test]
    fn test_error_genesis() {
        assert_eq!(error_message(INVALID_GENESIS_TXN_PATH.code_num), INVALID_GENESIS_TXN_PATH.as_str());
    }
    #[test]
    fn test_error_config() {
        assert_eq!(error_message(POOL_LEDGER_CONNECT.code_num), POOL_LEDGER_CONNECT.as_str());
    }
    #[test]
    fn test_error_pool_config() {
        assert_eq!(error_message(CREATE_POOL_CONFIG.code_num), CREATE_POOL_CONFIG.as_str());
    }
    #[test]
    fn test_error_big_number() {
        assert_eq!(error_message(BIG_NUMBER_ERROR.code_num), BIG_NUMBER_ERROR.as_str());
        assert_eq!(error_message(INVALID_PROOF_CREDENTIAL_DATA.code_num), INVALID_PROOF_CREDENTIAL_DATA.as_str());
        assert_eq!(error_message(NO_POOL_OPEN.code_num), NO_POOL_OPEN.as_str());
    }

    #[test]
    fn test_proof_incorrect_json_format_error(){
        assert_eq!(error_message(INVALID_PROOF.code_num), INVALID_PROOF.as_str());
    }

    #[test]
    fn test_error_credential_data() {
        assert_eq!(error_message(INVALID_PROOF_CREDENTIAL_DATA.code_num), INVALID_PROOF_CREDENTIAL_DATA.as_str());
    }

    #[test]
    fn test_credential_def_err() {
        assert_eq!(error_message(CREATE_CREDENTIAL_DEF_ERR.code_num), CREATE_CREDENTIAL_DEF_ERR.as_str());
    }

    #[test]
    fn test_unknown_libindy_error() {
        assert_eq!(error_message(UNKNOWN_LIBINDY_ERROR.code_num), UNKNOWN_LIBINDY_ERROR.as_str());
    }

    #[test]
    fn test_timeout_libindy_error() {
        assert_eq!(error_message(TIMEOUT_LIBINDY_ERROR.code_num), TIMEOUT_LIBINDY_ERROR.as_str());
    }

    #[test]
    fn test_credential_def_not_found() {
        assert_eq!(error_message(CREDENTIAL_DEFINITION_NOT_FOUND.code_num), CREDENTIAL_DEFINITION_NOT_FOUND.as_str());
    }

    #[test]
    fn test_credential_def_handle_err() {
        assert_eq!(error_message(INVALID_CREDENTIAL_DEF_HANDLE.code_num), INVALID_CREDENTIAL_DEF_HANDLE.as_str());
    }

    #[test]
    fn test_credential_def_already_on_ledger_err() {
        assert_eq!(error_message(CREDENTIAL_DEF_ALREADY_CREATED.code_num), CREDENTIAL_DEF_ALREADY_CREATED.as_str());
    }

    #[test]
    fn test_schema_err() {
        assert_eq!(error_message(INVALID_SCHEMA.code_num), INVALID_SCHEMA.as_str());
        assert_eq!(error_message(INVALID_SCHEMA_SEQ_NO.code_num), INVALID_SCHEMA_SEQ_NO.as_str());
        assert_eq!(error_message(INVALID_SCHEMA_CREATION.code_num), INVALID_SCHEMA_CREATION.as_str());
        assert_eq!(error_message(INVALID_SCHEMA_HANDLE.code_num), INVALID_SCHEMA_HANDLE.as_str());
    }

    #[test]
    fn test_already_initialized() {
        assert_eq!(error_message(ALREADY_INITIALIZED.code_num), ALREADY_INITIALIZED.as_str());
    }

    #[test]
    fn test_invalid_invite_details() {
        assert_eq!(error_message(INVALID_INVITE_DETAILS.code_num), INVALID_INVITE_DETAILS.as_str());
    }

    #[test]
    fn test_invalid_redirect_details() {
        assert_eq!(error_message(INVALID_REDIRECT_DETAILS.code_num), INVALID_REDIRECT_DETAILS.as_str());
    }
}
