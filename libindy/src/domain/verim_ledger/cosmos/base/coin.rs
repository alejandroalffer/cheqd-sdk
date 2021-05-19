use cosmos_sdk::proto::cosmos::base::v1beta1::Coin as CoinProto;

/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an String in Proto and implements the custom method
/// signatures required by gogoproto.
pub struct Coin {
    pub denom: String,
    pub amount: u64,
}
