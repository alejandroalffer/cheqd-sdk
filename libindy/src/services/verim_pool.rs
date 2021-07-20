//! Pool service for Tendermint back-end

use std::fs;
use std::io::Write;

use cosmos_sdk::rpc;
use cosmos_sdk::rpc::{Request, Response};
use cosmos_sdk::rpc::endpoint::broadcast;
use cosmos_sdk::tendermint::abci;
use cosmos_sdk::tx::Raw;
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use indy_api_types::errors::*;
use indy_api_types::IndyError;

use crate::domain::verim_pool::PoolConfig;
use crate::utils::environment;

pub(crate) struct VerimPoolService {}

impl VerimPoolService {
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

        let mut path = environment::verim_pool_path(alias);

        if path.as_path().exists() {
            return Err(err_msg(
                IndyErrorKind::PoolConfigAlreadyExists,
                format!(
                    "Verim pool ledger config file with alias \"{}\" already exists",
                    alias
                ),
            ));
        }

        fs::create_dir_all(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create verim pool config directory")?;

        path.push("config");
        path.set_extension("json");

        let mut f: fs::File = fs::File::create(path.as_path())
            .to_indy(IndyErrorKind::IOError, "Can't create verim pool config file")?;

        f.write_all({
            serde_json::to_string(&config)
                .to_indy(IndyErrorKind::InvalidState, "Can't serialize verim pool config")?
                .as_bytes()
        })
            .to_indy(IndyErrorKind::IOError, "Can't write to verim pool config file")?;

        f.flush()
            .to_indy(IndyErrorKind::IOError, "Can't write to verim pool config file")?;

        Ok(config)
    }

    pub(crate) async fn get_config(&self, alias: &str) -> IndyResult<PoolConfig> {
        let mut path = environment::verim_pool_path(alias);

        if !path.exists() {
            return Err(IndyError::from_msg(IndyErrorKind::IOError, format!("Can't find verim pool config file: {}", alias)));
        }

        path.push("config");
        path.set_extension("json");

        let config = fs::read_to_string(path)
            .to_indy(IndyErrorKind::IOError, format!("Can't open verim pool config file: {}", alias))?;

        let result: PoolConfig = serde_json::from_str(&config)
            .to_indy(IndyErrorKind::IOError, "Invalid data of verim pool config file")?;

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
        self.check_proofs(req, resp);
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



    async fn check_proofs(
        &self,
        req: rpc::endpoint::abci_query::Request,
        result: rpc::endpoint::abci_query::Response,
    ) -> IndyResult<rpc::endpoint::abci_query::Response> {

        //////////////////////////// 0st proof

        println!("verifying iavl proof");

        let proof_op_0 = &result.response.proof.clone().unwrap().ops[0];
        assert_eq!(&proof_op_0.key, &req.data);
        assert_eq!(&proof_op_0.field_type, "ics23:iavl");

        let proof_0_data_decoded =
            ics23::CommitmentProof::decode(proof_op_0.data.as_slice()).unwrap();

        let proof_op_1 = &result.response.proof.unwrap().ops[1];
        assert_eq!(&proof_op_1.key, "verimcosmos".as_bytes());
        assert_eq!(&proof_op_1.field_type, "ics23:simple");

        let proof_1_data_decoded =
            ics23::CommitmentProof::decode(proof_op_1.data.as_slice()).unwrap();

        let proof_0_root = if let Some(ics23::commitment_proof::Proof::Exist(ex)) =
        proof_1_data_decoded.proof.clone()
        {
            ex.value
        } else {
            panic!()
        };

        let proof_0_is_ok = ics23::verify_membership(
            &proof_0_data_decoded,
            &ics23::iavl_spec(),
            &proof_0_root,
            &proof_op_0.key,
            &result.response.value,
        );

        assert!(proof_0_is_ok);

        // Should be output from light client
        let proof_1_root = if let Some(ics23::commitment_proof::Proof::Exist(ex)) =
        proof_1_data_decoded.proof.clone()
        {
            ics23::calculate_existence_root(&ex).unwrap()
        } else {
            panic!()
        };

        let proof_1_is_ok = ics23::verify_membership(
            &proof_1_data_decoded,
            &ics23::tendermint_spec(),
            &proof_1_root,
            &proof_op_1.key,
            &proof_0_root,
        );

        Ok(resp)
    }

}