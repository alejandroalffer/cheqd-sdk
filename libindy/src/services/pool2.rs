//! Pool service for Cosmos back-end

use cosmos_sdk::tx::{Fee, Msg, SignDoc, SignerInfo};
use cosmos_sdk::{tx, Coin};
use indy_api_types::errors::IndyResult;

pub struct Pool2Service {}

impl Pool2Service {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add(&self) {
        unimplemented!()
    }

    pub fn pool_info(&self) {
        unimplemented!()
    }

    pub fn build_tx(
        &self,
        sender_public_key: &str,
        msgs: Vec<Msg>,
        chain_id: &str,
        account_number: u64,
        sequence_number: u64,
        gas_limit: u64,
        fee_limit: &Coin,
        timeout_height: u16,
        memo: &str,
    ) -> IndyResult<SignDoc> {
        let tx_body = tx::Body::new(msgs, memo, timeout_height);

        let sender_public_key = rust_base58::FromBase58::from_base58(sender_public_key)?;
        let sender_public_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(&sender_public_key)?;

        let signer_info =
            SignerInfo::single_direct(Some(sender_public_key.into()), sequence_number);

        let auth_info =
            signer_info.auth_info(Fee::from_amount_and_gas(fee_limit.clone(), gas_limit));

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id.parse()?, account_number)?;

        Ok(sign_doc)
    }

    pub fn send_tx() {
        unimplemented!()
    }
}
