use crate::domain::verim_ledger::cosmos::tx::mode_info::ModeInfo;
use cosmos_sdk::proto::cosmos::tx::v1beta1::SignerInfo as SignerInfoProto;
use serde::Serialize;

/// SignerInfo describes the public key and signing mode of a single top-level
/// signer.
#[derive(Serialize, Deserialize)]
pub struct SignerInfo {
    /// public_key is the public key of the signer. It is optional for accounts
    /// that already exist in state. If unset, the verifier can use the required \
    /// signer address for this position and lookup the public key.
    pub public_key: Option<prost_types::Any>,
    /// mode_info describes the signing mode of the signer and is a nested
    /// structure to support nested multisig pubkey's
    pub mode_info: Option<ModeInfo>,
    /// sequence is the sequence of the account, which describes the
    /// number of committed transactions signed by a given address. It is used to
    /// prevent replay attacks.
    pub sequence: u64,
}

impl SignerInfo {
    pub fn new(
        public_key: Option<prost_types::Any>,
        mode_info: Option<ModeInfo>,
        sequence: u64,
    ) -> Self {
        SignerInfo {
            public_key,
            mode_info,
            sequence,
        }
    }
}
