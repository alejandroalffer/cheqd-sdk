//! Ledger service for Cosmos back-end

use crate::domain::crypto::did::DidValue;
use crate::domain::verim_ledger::cosmos_ext::CosmosMsgExt;
use crate::services::{CosmosKeysService, CosmosPoolService, VerimLedgerService};
use async_std::sync::Arc;
use indy_api_types::errors::IndyResult;

pub(crate) struct VerimLedgerController {
    verim_ledger_service: Arc<VerimLedgerService>,
    cosmos_pool_service: Arc<CosmosPoolService>,
    cosmos_keys_service: Arc<CosmosKeysService>,
}

impl VerimLedgerController {
    pub(crate) fn new(
        verim_ledger_service: Arc<VerimLedgerService>,
        cosmos_pool_service: Arc<CosmosPoolService>,
        cosmos_keys_service: Arc<CosmosKeysService>,
    ) -> Self {
        Self {
            verim_ledger_service,
            cosmos_pool_service,
            cosmos_keys_service,
        }
    }

    pub fn build_msg_create_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "add_random > did {:?} creator {:?} verkey {:?} alias {:?} role {:?}",
            did,
            creator,
            verkey,
            alias,
            role
        );
        let msg = self
            .verim_ledger_service
            .build_msg_create_nym(did, creator, verkey, alias, role)?;
        trace!("add_random < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub fn build_msg_update_nym(
        &self,
        creator: &str,
        id: u64,
        verkey: &str,
        alias: &str,
        did: &str,
        role: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "add_random > creator {:?} id {:?} verkey {:?} alias {:?} did {:?} role {:?}",
            creator,
            id,
            verkey,
            alias,
            did,
            role
        );
        let msg = self
            .verim_ledger_service
            .build_msg_update_nym(creator, id, verkey, alias, did, role)?;
        trace!("add_random < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub fn build_msg_delete_nym(
        &self,
        creator: &str,
        id: u64,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "add_random > creator {:?} id {:?}",
            creator,
            id,
        );
        let msg = self
            .verim_ledger_service
            .build_msg_delete_nym(creator, id)?;
        trace!("add_random < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub fn parse_msg_create_nym_resp() {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::controllers::{CosmosKeysController, CosmosPoolController, VerimLedgerController};
    use crate::domain::cosmos_pool::CosmosPoolConfig;
    use crate::services::{CosmosKeysService, CosmosPoolService, VerimLedgerService};
    use async_std::sync::Arc;

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

            let ledger_controller = VerimLedgerController::new(
                ledger_service.clone(),
                pool_service.clone(),
                keys_service.clone(),
            );

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

    #[async_std::test]
    async fn test_msg_create_nym() {
        let harness = TestHarness::new();

        // Keys
        let alice = harness
            .keys_controller
            .add_from_mnemonic("alice", "alice")
            .await
            .unwrap();

        println!("Alice's account id: {}", alice.account_id);

        // Pool
        let pool_alias = harness
            .pool_controller
            .add("test_pool", "http://localhost:26657", "verimcosmos")
            .await
            .unwrap();

        let msg = harness
            .ledger_controller
            .build_msg_create_nym("did", &alice.account_id, "verkey", "bob", "role")
            .unwrap();

        let tx = harness
            .pool_controller
            .build_tx(
                "test_pool",
                "alice",
                &msg,
                9,
                2,
                300000,
                3u64,
                "token",
                39090,
                "memo",
            )
            .await
            .unwrap();

        let signed = harness.keys_controller.sign("alice", &tx).await.unwrap();

        harness
            .pool_controller
            .broadcast_tx_commit("test_pool", &signed)
            .await
            .unwrap();

        assert!(true)
    }

    #[async_std::test]
    async fn test_msg_update_nym() {
        let harness = TestHarness::new();

        // Keys
        let alice = harness
            .keys_controller
            .add_from_mnemonic("alice", "alice")
            .await
            .unwrap();

        println!("Alice's account id: {}", alice.account_id);

        // Pool
        let pool_alias = harness
            .pool_controller
            .add("test_pool", "http://localhost:26657", "verimcosmos")
            .await
            .unwrap();

        // let msg_create = harness
        //     .ledger_controller
        //     .build_msg_create_nym("did", &alice.account_id, "verkey", "bob", "role")
        //     .unwrap();

        let msg = harness
            .ledger_controller
            .build_msg_update_nym(&alice.account_id, 0u64, "verkey", "bob", "newdid", "role")
            .unwrap();

        let tx = harness
            .pool_controller
            .build_tx(
                "test_pool",
                "alice",
                &msg,
                9,
                1,
                300000,
                0,
                "token",
                39090,
                "memo",
            )
            .await
            .unwrap();

        let signed = harness.keys_controller.sign("alice", &tx).await.unwrap();

        let response = harness
            .pool_controller
            .broadcast_tx_commit("test_pool", &signed)
            .await
            .unwrap();

        // let msg = harness
        //     .ledger_controller
        //     .build_msg_update_nym(&alice.account_id, 0u64, "verkey", "bob", "newdid", "role")
        //     .unwrap();

        assert!(true)
    }

    #[async_std::test]
    async fn test_msg_delete_nym() {
        let harness = TestHarness::new();

        // Keys
        let alice = harness
            .keys_controller
            .add_from_mnemonic("alice", "alice")
            .await
            .unwrap();

        println!("Alice's account id: {}", alice.account_id);

        // Pool
        let pool_alias = harness
            .pool_controller
            .add("test_pool", "http://localhost:26657", "verimcosmos")
            .await
            .unwrap();

        // let msg_create = harness
        //     .ledger_controller
        //     .build_msg_create_nym("did", &alice.account_id, "verkey", "bob", "role")
        //     .unwrap();

        let msg = harness
            .ledger_controller
            .build_msg_delete_nym(&alice.account_id, 0u64)
            .unwrap();

        let tx = harness
            .pool_controller
            .build_tx(
                "test_pool",
                "alice",
                &msg,
                9,
                2,
                300000,
                3u64,
                "token",
                39090,
                "memo",
            )
            .await
            .unwrap();

        let signed = harness.keys_controller.sign("alice", &tx).await.unwrap();

        let response = harness
            .pool_controller
            .broadcast_tx_commit("test_pool", &signed)
            .await
            .unwrap();

        // let msg = harness
        //     .ledger_controller
        //     .build_msg_delete_nym(&alice.account_id, 0u64)
        //     .unwrap();

        assert!(true)
    }

}