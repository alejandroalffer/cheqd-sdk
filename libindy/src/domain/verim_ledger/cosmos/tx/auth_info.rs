use crate::domain::verim_ledger::cosmos::tx::signer_info::SignerInfo;
use crate::domain::verim_ledger::cosmos::tx::Fee;
use cosmos_sdk::proto::cosmos::tx::v1beta1::AuthInfo as AuthInfoProto;

/// AuthInfo describes the fee and signer modes that are used to sign a
/// transaction.
pub struct AuthInfo {
    /// signer_infos defines the signing modes for the required signers. The number
    /// and order of elements must match the required signers from TxBody's
    /// messages. The first element is the primary signer and the one which pays
    /// the fee.
    pub signer_infos: Vec<SignerInfo>,
    /// Fee is the fee and gas limit for the transaction. The first signer is the
    /// primary signer and the one which pays the fee. The fee can be calculated
    /// based on the cost of evaluating the body and doing signature verification
    /// of the signers. This can be estimated via simulation.
    pub fee: Option<Fee>,
}

impl AuthInfo {
    pub fn new(signer_infos: Vec<SignerInfo>, fee: Option<Fee>) -> Self {
        AuthInfo { signer_infos, fee }
    }
}
