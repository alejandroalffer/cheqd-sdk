//! Pool service for Cosmos back-end

use cosmos_sdk::rpc::endpoint::broadcast;
use cosmos_sdk::rpc::{Request, Response};
use cosmos_sdk::tendermint::abci;
use cosmos_sdk::tx::{Fee, Msg, Raw, SignDoc, SignerInfo};
use cosmos_sdk::{rpc, tx, Coin};
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use std::convert::TryInto;

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

        let chain_id = chain_id.try_into()?;

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;

        Ok(sign_doc)
    }

    // Send and wait for commit
    pub async fn send_tx_commit(
        &self,
        tx: Raw,
        rpc_address: &str,
    ) -> IndyResult<rpc::endpoint::broadcast::tx_commit::Response> {
        let tx_bytes = tx.to_bytes()?;
        let req = broadcast::tx_commit::Request::new(tx_bytes.into());
        let resp = self.send_req(req, rpc_address).await?;

        if let abci::Code::Err(code) = resp.check_tx.code {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidState,
                format!(
                    "check_tx: error code: {}, log: {}",
                    code,
                    serde_json::to_string(&resp.check_tx)?
                ),
            ));
        }

        if let abci::Code::Err(code) = resp.deliver_tx.code {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidState,
                format!(
                    "deliver_tx: error code: {}, log: {}",
                    code,
                    serde_json::to_string(&resp.deliver_tx)?
                ),
            ));
        }

        Ok(resp)
        // TODO: Commit proof
        // TODO: Error handling
        // TODO: Return?
    }

    async fn send_req<R>(&self, req: R, rpc_address: &str) -> IndyResult<R::Response>
    where
        R: Request,
    {
        let req_bytes = req.into_json().into_bytes();

        let mut resp = surf::post(rpc_address)
            .body(surf::Body::from_bytes(req_bytes))
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                format!("indy-sdk/{}", env!("CARGO_PKG_VERSION")),
            )
            .await?;

        let resp_str = resp.body_string().await?;
        let resp = R::Response::from_string(resp_str)?;

        Ok(resp)
    }
}
