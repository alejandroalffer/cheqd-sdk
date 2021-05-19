use cosmos_sdk::proto::cosmos::tx::v1beta1::TxBody as TxBodyProto;

/// TxBody is the body of a transaction that all signers sign over.
pub struct TxBody {
    /// messages is a list of messages to be executed. The required signers of
    /// those messages define the number and order of elements in AuthInfo's
    /// signer_infos and Tx's signatures. Each required signer address is added to
    /// the list only the first time it occurs.
    /// By convention, the first required signer (usually from the first message)
    /// is referred to as the primary signer and pays the fee for the whole
    /// transaction.
    pub messages: Vec<prost_types::Any>, // TODO
    /// memo is any arbitrary memo to be added to the transaction
    pub memo: String,
    /// timeout is the block height after which this transaction will not
    /// be processed by the chain
    pub timeout_height: u64,
    /// extension_options are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, the transaction will be rejected
    pub extension_options: Vec<prost_types::Any>, // TODO
    /// extension_options are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, they will be ignored
    pub non_critical_extension_options: Vec<::prost_types::Any>, // TODO
}
