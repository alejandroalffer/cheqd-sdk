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

        let alice = serde_json::Value::from_str(&alice).unwrap();

        println!(
            "Alice's account id: {}",
            alice["account_id"].as_str().unwrap()
        );

        // Pool
        let pool_alias = harness
            .pool_controller
            .add("test_pool", "http://localhost:26657", "verimcosmos")
            .await
            .unwrap();

        let msg = harness
            .ledger_controller
            .build_msg_create_nym(
                "did",
                &alice["account_id"].as_str().unwrap(),
                "verkey",
                "bob",
                "role",
            )
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
}
