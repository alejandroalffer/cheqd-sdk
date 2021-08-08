//! Pool service for Tendermint back-end

use std::fs;
use std::io::Write;

use http_client::HttpClient;
use http_client::http_types::{Method,
                              Request as HttpRequest,
                              Response as HttpResponse,
                              Body};
use http_client::h1::H1Client;

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

// ToDo: it can be used only in "unstable-config".
// const CLIENT_TIMEOUT: u64 = 10;

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
        let req_json = req.into_json();
        let mut req = HttpRequest::new( Method::Post,
                                    rpc_address);
        req.append_header("Content-Type", "application/json");
        req.append_header("User-Agent", format!("indy-sdk/{}", env!("CARGO_PKG_VERSION")));
        req.set_body(Body::from_string(req_json));

        let client = H1Client::new();
        // ToDo: it can be changed only in "unstable-config".
        // let config = client.config();
        // config.timeout(std::time::Duration::from_secs(CLIENT_TIMEOUT));
        // client.set_config(config);
        println!("Before sending request");

        let mut resp: HttpResponse = client.send(req).await?;
        let resp_str = resp.body_string().await?;
        let resp = R::Response::from_string(resp_str)?;

        Ok(resp)
    }
}

#[cfg(test)]
mod send_req {
    use crate::CheqdPoolService;
    use cosmos_sdk::rpc::endpoint::abci_info::Request;

    #[async_std::test]
    async fn client_close_if_connection_refused() {
        let pool_service = CheqdPoolService::new();
        let req = Request {};
        pool_service.send_req(req, "http://127.0.0.1:26657").await.map_err(|err| {
            assert!(err.to_string().contains("Connection refused"))
        });
    }
}