//! Pool service for Tendermint back-end

use crate::domain::tendermint_pool::TendermintPoolConfig;
use crate::domain::pool::PoolConfig;
use cosmos_sdk::crypto::PublicKey;
use cosmos_sdk::rpc::endpoint::broadcast;
use cosmos_sdk::rpc::{Request, Response};
use cosmos_sdk::tendermint::abci;
use cosmos_sdk::tendermint::block::Height;
use cosmos_sdk::tx::{AuthInfo, Fee, Msg, Raw, SignDoc, SignerInfo};
use cosmos_sdk::{rpc, tx, Coin};
use futures::lock::Mutex as MutexF;
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use indy_api_types::IndyError;
use std::collections::HashMap;
use std::convert::TryInto;
use crate::utils::environment;
use indy_api_types::errors::*;
use std::io::{Write, Read};
use std::fs;
use serde_json::Value;

pub(crate) struct TendermintPoolService {
    // TODO: Persistence
    pools: MutexF<HashMap<String, TendermintPoolConfig>>,
}

impl TendermintPoolService {
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
    ) -> IndyResult<TendermintPoolConfig> {
        let config = TendermintPoolConfig::new(
            alias.to_string(),
            rpc_address.to_string(),
            chain_id.to_string(),
        );

        let mut path = environment::verim_pool_path(alias);

        if path.as_path().exists() {
            return Err(err_msg(
                IndyErrorKind::PoolConfigAlreadyExists,
                format!(
                    "Pool ledger config file with alias \"{}\" already exists",
                    alias
                ),
            ));
        }

        fs::create_dir_all(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create pool config directory")?;

        path.push("config");
        path.set_extension("json");

        let mut f: fs::File = fs::File::create(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create pool config file")?;

        f.write_all({
            serde_json::to_string(&config)
                .to_indy(IndyErrorKind::InvalidState, "Can't serialize pool config")?
                .as_bytes()
        })
            .to_indy(IndyErrorKind::IOError, "Can't write to pool config file")?;

        f.flush()
            .to_indy(IndyErrorKind::IOError, "Can't write to pool config file")?;

        Ok(config)
    }

    pub(crate) async fn get_config(&self, alias: &str) -> IndyResult<TendermintPoolConfig> {
        let mut pool_home_path = environment::verim_pool_path(alias);

        if let Ok(entries) = fs::read_dir(pool_home_path) {
            for entry in entries {

                let dir_entry = if let Ok(dir_entry) = entry {
                    dir_entry
                } else {
                    continue;
                };

                if let Some(pool_name) = dir_entry
                    .path()
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                {
                    let mut f: fs::File = fs::File::open(dir_entry.path())
                        .to_indy(IndyErrorKind::IOError, format!("Can't open pool config file: {}", alias))?;
                    let mut out = String::new();
                    f.read_to_string(&mut out)?;
                    let pool_json: Value = serde_json::from_str(&out)
                        .to_indy(IndyErrorKind::IOError, "Invalid data of pool config file")?;
                    let pool_obj = pool_json.as_object().unwrap();

                    let result = TendermintPoolConfig::new(
                        pool_obj["alias"].to_string(),
                        pool_obj["rpc_address"].to_string(),
                        pool_obj["chain_id"].to_string(),
                    );

                    return Ok(result);
                }
            }
        }

        return Err(IndyError::from_msg(
            IndyErrorKind::PoolNotCreated,
            format!("Can't find pool config file: {}", alias),
        ));
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
    }

    pub(crate) async fn abci_query(
        &self,
        pool_alias: &str,
        req: rpc::endpoint::abci_query::Request,
    ) -> IndyResult<rpc::endpoint::abci_query::Response> {
        let pool = self.get_config(pool_alias).await?;
        let resp = self.send_req(req, pool.rpc_address.as_str()).await?;
        Ok(resp)
    }

    pub(crate) async fn abci_info(
        &self,
        pool_alias: &str,
    ) -> IndyResult<rpc::endpoint::abci_info::Response> {
        let pool = self.get_config(pool_alias).await?;
        let req = rpc::endpoint::abci_info::Request {};
        let resp = self.send_req(req, pool.rpc_address.as_str()).await?;
        Ok(resp)
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