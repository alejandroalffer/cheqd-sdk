use crate::domain::verim_ledger::ProtoEnum;
use cosmos_sdk::proto::cosmos::tx::signing::v1beta1::SignMode as SignModeProto;
use prost::Message;

/// SignMode represents a signing mode with its own security guarantees.
#[repr(i32)]
pub enum SignMode {
    /// SIGN_MODE_UNSPECIFIED specifies an unknown signing mode and will be
    /// rejected
    Unspecified = 0,
    /// SIGN_MODE_DIRECT specifies a signing mode which uses SignDoc and is
    /// verified with raw bytes from Tx
    Direct = 1,
    /// SIGN_MODE_TEXTUAL is a future signing mode that will verify some
    /// human-readable textual representation on top of the binary representation
    /// from SIGN_MODE_DIRECT
    Textual = 2,
    /// SIGN_MODE_LEGACY_AMINO_JSON is a backwards compatibility mode which uses
    /// Amino JSON and will be removed in the future
    LegacyAminoJson = 127,
}

impl ProtoEnum for SignMode {
    type Proto = SignModeProto;

    fn to_proto(&self) -> Self::Proto {
        match self {
            SignMode::Unspecified => Self::Proto::Unspecified,
            SignMode::Direct => Self::Proto::Direct,
            SignMode::Textual => Self::Proto::Textual,
            SignMode::LegacyAminoJson => Self::Proto::LegacyAminoJson,
        }
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        match proto {
            Self::Proto::Unspecified => Self::Unspecified,
            Self::Proto::Direct => Self::Direct,
            Self::Proto::Textual => Self::Textual,
            Self::Proto::LegacyAminoJson => Self::LegacyAminoJson,
        }
    }
}
