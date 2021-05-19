use crate::domain::verim_ledger::cosmos::tx::signing::SignMode;
use cosmos_sdk::proto::cosmos::tx::v1beta1::ModeInfo as ModeInfoProto;

/// ModeInfo describes the signing mode of a single or nested multisig signer.
/// TODO: Implement Multi mode if necessary
pub enum ModeInfo {
    Single(SignMode),
}
