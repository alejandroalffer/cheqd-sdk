//! Ledger service for Cosmos back-end

use crate::services::{Ledger2Service, Pool2Service, KeysService};
use async_std::sync::Arc;

pub(crate) struct Ledger2Controller {
    ledger2_service: Arc<Ledger2Service>,
    pool2_service: Arc<Pool2Service>,
    keys_service: Arc<KeysService>
}

impl Ledger2Controller {
    pub(crate) fn new(ledger2_service: Arc<Ledger2Service>,
                      pool2_service: Arc<Pool2Service>,
                      keys_service: Arc<KeysService>) -> Self {
        Self { ledger2_service, pool2_service, keys_service }
    }

    pub fn sign_request(&self) {


        let alice = keys_service
            .add_from_mnemonic("alice", "alice")
            .await
            .unwrap();
        let bob = keys_service.add_from_mnemonic("bob", "bob").await.unwrap();

        println!("Alice's account id: {}", alice.account_id);
        println!("Bob's account id: {}", bob.account_id);
        let msg = ledger2_service
            .build_msg_create_nym("alias", "verkey", "did", "role", &*alice.account_id)
            .unwrap();

        let tx = pool2_service
            .build_tx(
                &alice.pub_key,
                vec![msg],
                "verimcosmos",
                9, // What is it?
                0,
                300000,
                0u64,
                "stake",
                39090,
                "memo",
            )
            .unwrap();

        let signed = keys_service.sign("alice", tx).await.unwrap();

        // Broadcast

        pool2_service
            .send_tx_commit(signed, "http://localhost:26657")
            .await
            .unwrap();

        assert!(true)
    }

    pub fn submit_request(&self) {
        unimplemented!()
    }

    pub fn sign_and_submit_request(&self) {
        unimplemented!()
    }

    pub fn build_x_request(&self) {
        unimplemented!()
    }
}
