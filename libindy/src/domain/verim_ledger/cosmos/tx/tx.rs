use crate::domain::verim_ledger::cosmos::tx::auth_info::AuthInfo;
use crate::domain::verim_ledger::cosmos::tx::tx_body::TxBody;
use cosmos_sdk::proto::cosmos::tx::v1beta1::Tx as TxProto;

/// Tx is the standard type used for broadcasting transactions.
pub struct Tx {
    /// body is the processable content of the transaction
    pub body: Option<TxBody>,
    /// auth_info is the authorization related content of the transaction,
    /// specifically signers, signer modes and fee
    pub auth_info: Option<AuthInfo>,
    /// signatures is a list of signatures that matches the length and order of
    /// AuthInfo's signer_infos to allow connecting signature meta information like
    /// public key and signing mode by position.
    pub signatures: Vec<Vec<u8>>,
}
