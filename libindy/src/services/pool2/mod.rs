//! Pool service for Cosmos back-end

mod http_client;

use cosmos_sdk::tx::{Fee, Msg, Raw, SignDoc, SignerInfo};
use cosmos_sdk::{dev, rpc, tx, Coin};
use failure::_core::cmp::max;
use futures::TryFutureExt;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;

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
        max_gas: u64,
        max_coin: u64,
        max_coin_denom: &str,
        timeout_height: u16,
        memo: &str,
    ) -> IndyResult<SignDoc> {
        let tx_body = tx::Body::new(msgs, memo, timeout_height);

        let sender_public_key = rust_base58::FromBase58::from_base58(sender_public_key)?;
        let sender_public_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(&sender_public_key)?;

        let signer_info =
            SignerInfo::single_direct(Some(sender_public_key.into()), sequence_number);

        let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(
            Coin {
                denom: max_coin_denom.parse()?,
                amount: max_coin.into(),
            },
            max_gas,
        ));

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id.parse()?, account_number)?;

        Ok(sign_doc)
    }

    // Send and wait for commit
    pub async fn send_tx_commit(
        &self,
        rpc_address: &str,
        tx_raw: Raw,
    ) -> IndyResult<tendermint_rpc::endpoint::broadcast::tx_commit::Response> {
        let rpc_client = rpc::HttpClient::new(rpc_address)?;
        let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await?;

        if let tendermint::abci::Code::Err(code) = tx_commit_response.check_tx.code {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidState,
                format!("check_tx: error code: {}", code),
            ));
        }

        if let tendermint::abci::Code::Err(code) = tx_commit_response.deliver_tx.code {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidState,
                format!("deliver_tx: error code: {}", code),
            ));
        }

        Ok(tx_commit_response)
        // TODO: Commit proof?
    }
}
