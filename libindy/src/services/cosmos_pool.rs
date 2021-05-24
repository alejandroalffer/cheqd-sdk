//! Pool service for Cosmos back-end

use crate::domain::cosmos_pool::CosmosPoolConfig;
use crate::domain::pool::PoolConfig;
use crate::domain::verim_ledger::cosmos_ext::CosmosMsgExt;
use cosmos_sdk::crypto::PublicKey;
use cosmos_sdk::rpc::endpoint::broadcast;
use cosmos_sdk::rpc::{Request, Response};
use cosmos_sdk::tendermint::abci;
use cosmos_sdk::tendermint::block::Height;
use cosmos_sdk::tx::{AuthInfo, Fee, Msg, Raw, SignDoc, SignerInfo};
use cosmos_sdk::{rpc, tx, Coin};
use futures::lock::Mutex as MutexF;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use std::collections::HashMap;
use std::convert::TryInto;

pub(crate) struct CosmosPoolService {
    // TODO: Persistence
    pools: MutexF<HashMap<String, CosmosPoolConfig>>,
}

impl CosmosPoolService {
    pub(crate) fn new() -> Self {
        Self {
            pools: MutexF::new(HashMap::new()),
        }
    }

    pub(crate) async fn add(
        &self,
        alias: &str,
        rpc_address: &str,
        chain_id: &str,
    ) -> IndyResult<CosmosPoolConfig> {
        let mut pools = self.pools.lock().await;

        if pools.contains_key(alias) {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidState,
                "Pool already exists",
            ));
        }

        let config = CosmosPoolConfig::new(
            alias.to_string(),
            rpc_address.to_string(),
            chain_id.to_string(),
        );

        pools.insert(alias.to_string(), config.clone());
        Ok(config)
    }

    pub(crate) async fn get_config(&self, alias: &str) -> IndyResult<CosmosPoolConfig> {
        let pools = self.pools.lock().await;

        let config = pools.get(alias).ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Pool not found",
        ))?;

        Ok(config.clone())
    }

    pub(crate) async fn build_tx(
        &self,
        chain_id: &str,
        sender_public_key: &str,
        msg: Msg,
        account_number: u64,
        sequence_number: u64,
        max_gas: u64,
        max_coin_amount: u64,
        max_coin_denom: &str,
        timeout_height: u64,
        memo: &str,
    ) -> IndyResult<SignDoc> {
        let timeout_height: Height = timeout_height.try_into()?;

        let tx_body = tx::Body::new(vec![msg], memo, timeout_height);

        let signer_info = Self::build_signer_info(sender_public_key, sequence_number)?;

        let auth_info =
            Self::build_auth_info(max_gas, max_coin_amount, max_coin_denom, signer_info)?;

        let chain_id = chain_id.try_into()?;

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;

        Ok(sign_doc)
    }

    fn build_auth_info(
        max_gas: u64,
        max_coin: u64,
        max_coin_denom: &str,
        signer_info: SignerInfo,
    ) -> IndyResult<AuthInfo> {
        let amount = Coin {
            denom: max_coin_denom.parse()?,
            amount: max_coin.into(),
        };

        let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(amount, max_gas));

        Ok(auth_info)
    }

    fn build_signer_info(public_key: &str, sequence_number: u64) -> IndyResult<SignerInfo> {
        let public_key = rust_base58::FromBase58::from_base58(public_key)?;
        let public_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(&public_key)?;
        let public_key: PublicKey = public_key.into();

        let signer_info = SignerInfo::single_direct(Some(public_key), sequence_number);
        Ok(signer_info)
    }

    // Send and wait for commit
    pub(crate) async fn broadcast_tx_commit(
        &self,
        pool_alias: &str,
        tx: Raw,
    ) -> IndyResult<rpc::endpoint::broadcast::tx_commit::Response> {
        let pool = self.get_config(pool_alias).await?;

        let tx_bytes = tx.to_bytes()?;
        let req = broadcast::tx_commit::Request::new(tx_bytes.into());
        let resp = self.send_req(req, &pool.rpc_address).await?;

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

    pub(crate) async fn abci_query(
        &self,
        pool_alias: &str,
        req: rpc::endpoint::abci_query::Request,
    ) -> IndyResult<rpc::endpoint::abci_query::Response> {
        let pool = self.pool_config(pool_alias).await?;
        let resp = self.send_req(req, pool.rpc_address.as_str()).await?;
        Ok(resp)
        // TODO: State proof
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
