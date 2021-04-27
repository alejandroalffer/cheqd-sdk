pub struct KeyInfo {
    pub alias: String,
    pub account_id: String,
    // Base58-encoded SEC1-encoded secp256k1 ECDSA key
    pub pub_key: String,
}

impl KeyInfo {
    pub fn new(alias: String, account_id: String, pub_key: String) -> Self {
        KeyInfo {
            alias,
            account_id,
            pub_key,
        }
    }
}
