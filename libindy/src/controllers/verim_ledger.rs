//! Ledger service for Cosmos back-end

use async_std::sync::Arc;
use cosmos_sdk::rpc::endpoint::abci_query::Response as QueryResponse;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::rpc::Request;
use cosmos_sdk::tx::Msg;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;

use crate::domain::crypto::did::DidValue;
use crate::domain::verim_ledger::cosmos::base::query::PageRequest;
use crate::domain::verim_ledger::cosmos_ext::{CosmosMsgExt, CosmosSignDocExt};
use crate::domain::verim_ledger::verim::messages::{
    MsgCreateNymResponse, MsgDeleteNymResponse, MsgUpdateNymResponse,
};
use crate::domain::verim_ledger::verim::queries::QueryGetNymResponse;
use crate::domain::verim_ledger::VerimProto;
use crate::services::{VerimKeysService, VerimLedgerService, VerimPoolService};

pub(crate) struct VerimLedgerController {
    verim_ledger_service: Arc<VerimLedgerService>,
    verim_pool_service: Arc<VerimPoolService>,
    verim_keys_service: Arc<VerimKeysService>,
}

impl VerimLedgerController {
    pub fn new(verim_ledger_service: Arc<VerimLedgerService>, verim_pool_service: Arc<VerimPoolService>, verim_keys_service: Arc<VerimKeysService>) -> Self {
        VerimLedgerController { verim_ledger_service, verim_pool_service, verim_keys_service }
    }

    pub(crate) async fn build_tx(
        &self,
        pool_alias: &str,
        sender_alias: &str,
        msg: &[u8],
        account_number: u64,
        sequence_number: u64,
        max_gas: u64,
        max_coin_amount: u64,
        max_coin_denom: &str,
        timeout_height: u64,
        memo: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!("build_tx > pool_alias {:?}, sender_alias {:?}, msg {:?}, account_number {:?}, sequence_number {:?}, max_gas {:?}, max_coin_amount {:?}, max_coin_denom {:?}, timeout_height {:?}, memo {:?}", pool_alias, sender_alias, msg, account_number, sequence_number, max_gas, max_coin_amount, max_coin_denom, timeout_height, memo);

        let pool = self.verim_pool_service.get_config(pool_alias).await?;
        let sender = self.verim_keys_service.get_info(sender_alias).await?;
        let msg = Msg::from_bytes(&msg)?;

        let sign_doc = self
            .verim_ledger_service
            .build_tx(
                &pool.chain_id,
                &sender.pub_key,
                msg,
                account_number,
                sequence_number,
                max_gas,
                max_coin_amount,
                max_coin_denom,
                timeout_height,
                memo,
            )
            .await?;

        trace!("build_tx <");

        Ok(sign_doc.to_bytes()?)
    }

    pub(crate) fn build_query_cosmos_auth_account(&self, address: &str) -> IndyResult<String> {
        trace!("build_query_cosmos_auth_account >");
        let query = self
            .verim_ledger_service
            .build_query_cosmos_auth_account(address)?;
        let json = serde_json::to_string(&query)?;
        trace!("build_query_cosmos_auth_account < {:?}", query);
        Ok(json)
    }

    pub(crate) fn parse_query_cosmos_auth_account_resp(
        &self,
        resp_json: &str,
    ) -> IndyResult<String> {
        trace!(
            "parse_query_cosmos_auth_account_resp > resp {:?}",
            resp_json
        );
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self
            .verim_ledger_service
            .parse_query_cosmos_auth_account_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("parse_query_cosmos_auth_account_resp < {:?}", json_result);
        Ok(json_result)
    }

    pub(crate) fn build_msg_create_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "build_msg_create_nym > did {:?} creator {:?} verkey {:?} alias {:?} role {:?}",
            did,
            creator,
            verkey,
            alias,
            role
        );
        let msg = self
            .verim_ledger_service
            .build_msg_create_nym(did, creator, verkey, alias, role)?;
        trace!("build_msg_create_nym < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub(crate) fn parse_msg_create_nym_resp(&self, resp: &str) -> IndyResult<String> {
        trace!("parse_msg_create_nym_resp > resp {:?}", resp);
        let resp: Response = serde_json::from_str(&resp)?;
        let res = self.verim_ledger_service.parse_msg_create_nym_resp(&resp)?;
        let res = serde_json::to_string(&res)?;
        trace!("parse_msg_create_nym_resp < {:?}", res);
        Ok(res)
    }

    pub(crate) fn build_msg_update_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
        id: u64,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "build_msg_update_nym > creator {:?} verkey {:?} alias {:?} did {:?} role {:?} id {:?}",
            did,
            creator,
            verkey,
            alias,
            role,
            id
        );
        let msg = self
            .verim_ledger_service
            .build_msg_update_nym(did, creator, verkey, alias, role, id)?;
        trace!("build_msg_update_nym < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub(crate) fn parse_msg_update_nym_resp(&self, resp: &str) -> IndyResult<String> {
        trace!("parse_msg_update_nym_resp > resp {:?}", resp);
        let resp: Response = serde_json::from_str(resp)?;
        let res = self.verim_ledger_service.parse_msg_update_nym_resp(&resp)?;
        let res = serde_json::to_string(&res)?;
        trace!("parse_msg_update_nym_resp < {:?}", res);
        Ok(res)
    }

    pub(crate) fn build_msg_delete_nym(&self, creator: &str, id: u64) -> IndyResult<Vec<u8>> {
        trace!("build_msg_delete_nym > creator {:?} id {:?}", creator, id,);
        let msg = self
            .verim_ledger_service
            .build_msg_delete_nym(creator, id)?;
        trace!("build_msg_delete_nym < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub(crate) fn parse_msg_delete_nym_resp(&self, resp: &str) -> IndyResult<String> {
        trace!("parse_msg_delete_nym_resp > resp {:?}", resp);
        let resp: Response = serde_json::from_str(resp)?;
        let res = self.verim_ledger_service.parse_msg_delete_nym_resp(&resp)?;
        let res = serde_json::to_string(&res)?;
        trace!("parse_msg_delete_nym_resp < {:?}", res);
        Ok(res)
    }

    pub(crate) fn build_query_get_nym(&self, id: u64) -> IndyResult<String> {
        trace!("build_query_get_nym > id {:?}", id);
        let query = self.verim_ledger_service.build_query_get_nym(id)?;
        let json = serde_json::to_string(&query)?;
        trace!("build_query_get_nym < {:?}", query);
        Ok(json)
    }

    pub(crate) fn parse_query_get_nym_resp(&self, resp_json: &str) -> IndyResult<String> {
        trace!("parse_query_get_nym_resp > resp {:?}", resp_json);
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self.verim_ledger_service.parse_query_get_nym_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("parse_query_get_nym_resp < {:?}", json_result);
        Ok(json_result)
    }

    pub(crate) fn build_query_all_nym(&self) -> IndyResult<String> {
        trace!("build_query_all_nym >");
        let query = self.verim_ledger_service.build_query_all_nym(None)?;
        let json = serde_json::to_string(&query)?;
        trace!("build_query_all_nym < {:?}", query);
        Ok(json)
    }

    pub(crate) fn parse_query_all_nym_resp(&self, resp_json: &str) -> IndyResult<String> {
        trace!("parse_query_all_nym_resp > resp {:?}", resp_json);
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self.verim_ledger_service.parse_query_all_nym_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("parse_query_all_nym_resp < {:?}", json_result);
        Ok(json_result)
    }
}
