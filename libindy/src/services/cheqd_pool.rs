//! Pool service for Tendermint back-end

use std::fs;
use std::io::Write;
use std::time::Duration;
use tokio;

use cosmos_sdk::rpc;
use cosmos_sdk::rpc::{Request, Response};
use cosmos_sdk::rpc::endpoint::broadcast;
use cosmos_sdk::tendermint::abci;
use cosmos_sdk::tx::Raw;
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use indy_api_types::errors::*;
use indy_api_types::IndyError;

use crate::domain::cheqd_pool::PoolConfig;
use crate::utils::environment;

pub(crate) struct CheqdPoolService {}

const CLIENT_TIMEOUT: u64 = 10;

impl CheqdPoolService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn add(
        &self,
        alias: &str,
        rpc_address: &str,
        chain_id: &str,
    ) -> IndyResult<PoolConfig> {
        let config = PoolConfig::new(
            alias.to_string(),
            rpc_address.to_string(),
            chain_id.to_string(),
        );

        let mut path = environment::cheqd_pool_path(alias);

        if path.as_path().exists() {
            return Err(err_msg(
                IndyErrorKind::PoolConfigAlreadyExists,
                format!(
                    "Cheqd pool ledger config file with alias \"{}\" already exists",
                    alias
                ),
            ));
        }

        fs::create_dir_all(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create cheqd pool config directory")?;

        path.push("config");
        path.set_extension("json");

        let mut f: fs::File = fs::File::create(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create cheqd pool config file")?;

        f.write_all({
            serde_json::to_string(&config)
                .to_indy(IndyErrorKind::InvalidState, "Can't serialize cheqd pool config")?
                .as_bytes()
        })
            .to_indy(IndyErrorKind::IOError, "Can't write to cheqd pool config file")?;

        f.flush()
            .to_indy(IndyErrorKind::IOError, "Can't write to cheqd pool config file")?;

        Ok(config)
    }

    pub(crate) async fn get_config(&self, alias: &str) -> IndyResult<PoolConfig> {
        let mut path = environment::cheqd_pool_path(alias);

        if !path.exists() {
            return Err(IndyError::from_msg(IndyErrorKind::IOError, format!("Can't find cheqd pool config file: {}", alias)));
        }

        path.push("config");
        path.set_extension("json");

        let config = fs::read_to_string(path)
            .to_indy(IndyErrorKind::IOError, format!("Can't open cheqd pool config file: {}", alias))?;

        let result: PoolConfig = serde_json::from_str(&config)
            .to_indy(IndyErrorKind::IOError, "Invalid data of cheqd pool config file")?;

        Ok(result)
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
        let resp = self.send_req(req, &pool.rpc_address)?;

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
        let resp = self.send_req(req, pool.rpc_address.as_str())?;
        Ok(resp)
    }

    pub(crate) async fn abci_info(
        &self,
        pool_alias: &str,
    ) -> IndyResult<rpc::endpoint::abci_info::Response> {
        let pool = self.get_config(pool_alias).await?;
        let req = rpc::endpoint::abci_info::Request {};
        let resp = self.send_req(req, pool.rpc_address.as_str())?;
        Ok(resp)
    }

    #[tokio::main]
    async fn send_req<R>(&self, req: R, rpc_address: &str) -> IndyResult<R::Response>
        where
            R: Request,
    {
        let timeout = Duration::new(CLIENT_TIMEOUT, 0);
        let req_json = req.into_json();

        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()?;
        let resp = client.post(rpc_address)
            .body(req_json)
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                format!("indy-sdk/{}", env!("CARGO_PKG_VERSION")),
            )
            .send()
            .await?;
        let resp_str = resp.text().await?;
        let resp = R::Response::from_string(resp_str)?;

        Ok(resp)
    }
}