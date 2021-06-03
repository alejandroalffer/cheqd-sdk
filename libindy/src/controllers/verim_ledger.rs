//! Ledger service for Cosmos back-end

use crate::domain::crypto::did::DidValue;
use crate::domain::verim_ledger::cosmos_ext::CosmosMsgExt;
use crate::domain::verim_ledger::verimcosmos::messages::{
    MsgCreateNymResponse, MsgDeleteNymResponse, MsgUpdateNymResponse,
};
use crate::domain::verim_ledger::verimcosmos::queries::QueryGetNymResponse;
use crate::domain::verim_ledger::VerimProto;
use crate::services::{CosmosKeysService, CosmosPoolService, VerimLedgerService};
use async_std::sync::Arc;
use cosmos_sdk::rpc::endpoint::abci_query::Response as QueryResponse;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::rpc::Request;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use crate::domain::verim_ledger::cosmos::base::query::PageRequest;

pub(crate) struct VerimLedgerController {
    verim_ledger_service: Arc<VerimLedgerService>,
}

impl VerimLedgerController {
    pub(crate) fn new(verim_ledger_service: Arc<VerimLedgerService>) -> Self {
        Self {
            verim_ledger_service,
        }
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

#[cfg(test)]
mod test {
    use crate::controllers::{CosmosKeysController, CosmosPoolController, VerimLedgerController};
    use crate::domain::cosmos_pool::CosmosPoolConfig;
    use crate::services::{CosmosKeysService, CosmosPoolService, VerimLedgerService};
    use async_std::sync::Arc;
    use std::str::FromStr;

    struct TestHarness {
        ledger_controller: VerimLedgerController,
        pool_controller: CosmosPoolController,
        keys_controller: CosmosKeysController,
    }

    impl TestHarness {
        fn new() -> Self {
            let ledger_service = Arc::new(VerimLedgerService::new());
            let pool_service = Arc::new(CosmosPoolService::new());
            let keys_service = Arc::new(CosmosKeysService::new());

            let ledger_controller = VerimLedgerController::new(ledger_service.clone());

            let pool_controller =
                CosmosPoolController::new(pool_service.clone(), keys_service.clone());

            let keys_controller = CosmosKeysController::new(keys_service.clone());

            Self {
                ledger_controller,
                pool_controller,
                keys_controller,
            }
        }
    }

    #[async_std::test]
    async fn test_msg_bank_send() {
        unimplemented!()
    }
    //
    // #[async_std::test]
    // async fn test_msg_create_nym() {
    //     let harness = TestHarness::new();
    //
    //     // Keys
    //     let alice = harness
    //         .keys_controller
    //         .add_from_mnemonic("alice", "alice")
    //         .await
    //         .unwrap();
    //
    //     let alice = serde_json::Value::from_str(&alice).unwrap();
    //
    //     println!(
    //         "Alice's account id: {}",
    //         alice["account_id"].as_str().unwrap()
    //     );
    //
    //     // Pool
    //     harness
    //         .pool_controller
    //         .add("test_pool", "http://localhost:26657", "verim-cosmos-chain")
    //         .await
    //         .unwrap();
    //
    //     // Msg for create transaction
    //     let msg = harness
    //         .ledger_controller
    //         .build_msg_create_nym("did", &alice.account_id, "verkey", "bob", "role")
    //         .unwrap();
    //
    //     // Transaction of create
    //     let tx = harness
    //         .pool_controller
    //         .build_tx(
    //             "test_pool",
    //             "alice",
    //             &msg,
    //             11,
    //             1,
    //             300000,
    //             300000u64,
    //             "token",
    //             39090,
    //             "memo",
    //         )
    //         .await
    //         .unwrap();
    //
    //     let signed = harness.keys_controller.sign("alice", &tx).await.unwrap();
    //
    //     // Broadcast transaction of create
    //     let resp = harness
    //         .pool_controller
    //         .broadcast_tx_commit("test_pool", &signed)
    //         .await
    //         .unwrap();
    //
    //     // Parse response of create transaction
    //     let result = harness
    //         .ledger_controller
    //         .parse_msg_create_nym_resp(&resp)
    //         .unwrap();
    //
    //     println!("result: {:?}", result);
    //
    //     assert!(true)
    // }
    //
    // #[async_std::test]
    // async fn test_msg_update_nym() {
    //     let harness = TestHarness::new();
    //
    //     // Keys
    //     let alice = harness
    //         .keys_controller
    //         .add_from_mnemonic("alice", "alice")
    //         .await
    //         .unwrap();
    //
    //     println!("Alice's account id: {}", alice.account_id);
    //
    //     // Pool
    //     let pool_alias = harness
    //         .pool_controller
    //         .add("test_pool", "http://localhost:26657", "verim-cosmos-chain")
    //         .await
    //         .unwrap();
    //
    //     // Msg for create transaction
    //     let msg_create = harness
    //         .ledger_controller
    //         .build_msg_create_nym("did", &alice.account_id, "verkey", "bob", "role")
    //         .unwrap();
    //
    //     // Transaction of create
    //     let tx_create = harness
    //         .pool_controller
    //         .build_tx(
    //             "test_pool",
    //             "alice",
    //             &msg_create,
    //             11,
    //             10,
    //             300000,
    //             300000,
    //             "token",
    //             39090,
    //             "memo",
    //         )
    //         .await
    //         .unwrap();
    //
    //     let signed_create = harness
    //         .keys_controller
    //         .sign("alice", &tx_create)
    //         .await
    //         .unwrap();
    //
    //     // Broadcast transaction of create
    //     let response_create = harness
    //         .pool_controller
    //         .broadcast_tx_commit("test_pool", &signed_create)
    //         .await
    //         .unwrap();
    //
    //     // Parse response of create transaction
    //     let result_create = harness
    //         .ledger_controller
    //         .parse_msg_create_nym_resp(&response_create)
    //         .unwrap();
    //
    //     // Msg for update transaction
    //     let msg = harness
    //         .ledger_controller
    //         .build_msg_update_nym(
    //             "newdid",
    //             &alice.account_id,
    //             "verkey",
    //             "bob",
    //             "role",
    //             result_create.id,
    //         )
    //         .unwrap();
    //
    //     // Transaction of update
    //     let tx = harness
    //         .pool_controller
    //         .build_tx(
    //             "test_pool",
    //             "alice",
    //             &msg,
    //             11,
    //             11,
    //             300000,
    //             300000,
    //             "token",
    //             39090,
    //             "memo",
    //         )
    //         .await
    //         .unwrap();
    //
    //     let signed = harness.keys_controller.sign("alice", &tx).await.unwrap();
    //
    //     // Broadcast transaction of update
    //     let response = harness
    //         .pool_controller
    //         .broadcast_tx_commit("test_pool", &signed)
    //         .await
    //         .unwrap();
    //
    //     // Parse response of update transaction
    //     let result = harness
    //         .ledger_controller
    //         .parse_msg_update_nym_resp(&response)
    //         .unwrap();
    //
    //     println!("result: {:?}", result);
    //
    //     assert!(true)
    // }
    //
    // #[async_std::test]
    // async fn test_msg_delete_nym() {
    //     let harness = TestHarness::new();
    //
    //     // Keys
    //     let alice = harness
    //         .keys_controller
    //         .add_from_mnemonic("alice", "alice")
    //         .await
    //         .unwrap();
    //
    //     println!("Alice's account id: {}", alice.account_id);
    //
    //     // Pool
    //     let pool_alias = harness
    //         .pool_controller
    //         .add("test_pool", "http://localhost:26657", "verim-cosmos-chain")
    //         .await
    //         .unwrap();
    //
    //     // Msg for create transaction
    //     let msg_create = harness
    //         .ledger_controller
    //         .build_msg_create_nym("did", &alice.account_id, "verkey", "bob", "role")
    //         .unwrap();
    //
    //     // Transaction of create
    //     let tx_create = harness
    //         .pool_controller
    //         .build_tx(
    //             "test_pool",
    //             "alice",
    //             &msg_create,
    //             11,
    //             8,
    //             300000,
    //             300000u64,
    //             "token",
    //             39090,
    //             "memo",
    //         )
    //         .await
    //         .unwrap();
    //
    //     let signed_create = harness
    //         .keys_controller
    //         .sign("alice", &tx_create)
    //         .await
    //         .unwrap();
    //
    //     // Broadcast transaction of create
    //     let response_create = harness
    //         .pool_controller
    //         .broadcast_tx_commit("test_pool", &signed_create)
    //         .await
    //         .unwrap();
    //
    //     // Parse response of create transaction
    //     let result_create = harness
    //         .ledger_controller
    //         .parse_msg_create_nym_resp(&response_create)
    //         .unwrap();
    //
    //     // Msg for delete transaction
    //     let msg = harness
    //         .ledger_controller
    //         .build_msg_delete_nym(&alice.account_id, result_create.id)
    //         .unwrap();
    //
    //     // Transaction of delete
    //     let tx = harness
    //         .pool_controller
    //         .build_tx(
    //             "test_pool",
    //             "alice",
    //             &msg,
    //             11,
    //             9,
    //             300000,
    //             300000u64,
    //             "token",
    //             39090,
    //             "memo",
    //         )
    //         .await
    //         .unwrap();
    //
    //     let signed = harness.keys_controller.sign("alice", &tx).await.unwrap();
    //
    //     // Broadcast transaction of delete
    //     let response = harness
    //         .pool_controller
    //         .broadcast_tx_commit("test_pool", &signed)
    //         .await
    //         .unwrap();
    //
    //     // Parse response of delete transaction
    //     let result = harness
    //         .ledger_controller
    //         .parse_msg_delete_nym_resp(&response)
    //         .unwrap();
    //
    //     println!("result: {:?}", result);
    //
    //     assert!(true)
    // }
    //
    //

    #[async_std::test]
    async fn test_query_get_nym() {
        let harness = TestHarness::new();
        let pool_alias = "test_pool";

        // Keys
        let alice = harness
            .keys_controller
            .add_from_mnemonic("alice", "alice")
            .await
            .unwrap();

        // println!("Alice's account id: {}", alice.account_id);

        // Pool
        let pool = harness
            .pool_controller
            .add(pool_alias, "http://localhost:26657", "verim-cosmos-chain")
            .await
            .unwrap();
        //
        // // Msg for create transaction
        // let msg_create = harness
        //     .ledger_controller
        //     .build_msg_create_nym("did", &alice.account_id, "verkey", "bob", "role")
        //     .unwrap();
        //
        // // Transaction of create
        // let tx_create = harness
        //     .pool_controller
        //     .build_tx(
        //         pool_alias,
        //         "alice",
        //         &msg_create,
        //         11,
        //         8,
        //         300000,
        //         300000u64,
        //         "token",
        //         39090,
        //         "memo",
        //     )
        //     .await
        //     .unwrap();
        //
        // let signed_create = harness
        //     .keys_controller
        //     .sign("alice", &tx_create)
        //     .await
        //     .unwrap();
        //
        // // Broadcast transaction of create
        // let response_create = harness
        //     .pool_controller
        //     .broadcast_tx_commit(pool_alias, &signed_create)
        //     .await
        //     .unwrap();
        //
        // // Parse response of create transaction
        // let result_create = harness
        //     .ledger_controller
        //     .parse_msg_create_nym_resp(&response_create)
        //     .unwrap();

        // Request created NYM
        // Build query request
        let req = harness
            .ledger_controller
            // .build_query_get_nym(result_create.id)
            .build_query_get_nym(1)
            .unwrap();
        println!("{:?}", req);

        // Send request
        let response_create = harness
            .pool_controller
            .abci_query(pool_alias, &req)
            .await
            .unwrap();

        // Parse response of getting transaction
        let result = harness
            .ledger_controller
            .parse_query_get_nym_resp(&response_create)
            .unwrap();

        println!("result: {:?}", result);
        assert!(true)
    }
}
