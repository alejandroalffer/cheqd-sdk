extern crate zeroize;

use self::zeroize::Zeroize;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Key {
    pub alias: String,
    // SEC1-encoded secp256k1 ECDSA priv key
    #[cfg(not(test))]
    #[derivative(Debug = "ignore")]
    pub priv_key: Vec<u8>,
    #[cfg(test)]
    pub priv_key: Vec<u8>,
}

impl Key {
    pub fn new(alias: String, priv_key: Vec<u8>) -> Self {
        Key {
            alias,
            priv_key,
        }
    }
}

impl Zeroize for Key {
    fn zeroize(&mut self) {
        self.signkey.zeroize();
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        self.signkey.zeroize();
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
