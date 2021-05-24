#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use utils::{constants::*, cosmos_keys, cosmos_pool, verim_ledger};

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod verim_ledger_transactions {
        use super::*;

        #[test]
        fn build_msg_create_nym() {
            let did = "0";
            let creator = "1";
            let role = "some_role";
            let alias = "some_alias";
            let verkey = "some_verkey";
            verim_ledger::build_msg_create_nym(did, creator, verkey, alias, role);
        }

        #[test]
        fn build_msg_update_nym() {
            let did = "0";
            let creator = "1";
            let role = "some_role";
            let alias = "some_alias";
            let verkey = "some_verkey";
            let id = 2;
            verim_ledger::build_msg_update_nym(did, creator, verkey, alias, role, id);
        }

        #[test]
        fn build_msg_delete_nym() {
            let creator = "1";
            let id = 2;
            verim_ledger::build_msg_delete_nym(creator, id);
        }
    }

    #[cfg(test)]
    mod e2e {
        use super::*;
        use serde_json::Value;

        #[test]
        fn test_create_nym() {
            // Create key
            let alice = cosmos_keys::add_from_mnemonic("alice", "alice").unwrap();
            println!("Alice's account: {}", alice);
            let alice: Value = serde_json::from_str(&alice).unwrap();

            // Pool
            let pool =
                cosmos_pool::add("test_pool", "http://localhost:26657", "verimcosmos").unwrap();
            println!("Pool config: {}", pool);

            // Message
            let msg = verim_ledger::build_msg_create_nym(
                "test-did",
                alice["account_id"].as_str().unwrap(),
                "test-verkey",
                "test-alias",
                "test-role",
            )
            .unwrap();

            // Transaction
            let tx = cosmos_pool::build_tx(
                "test_pool",
                "alice",
                &msg,
                9,
                1,
                300000,
                0u64,
                "token",
                39090,
                "memo",
            )
            .unwrap();

            // Signature
            let signed = cosmos_keys::sign("alice", &tx).unwrap();

            // Broadcast
            let resp = cosmos_pool::broadcast_tx_commit("test_pool", &signed).unwrap();

            // // Parse response of create transaction
            // let result = harness
            //     .ledger_controller
            //     .parse_msg_create_nym_resp(&resp)
            //     .unwrap();
            //
            // println!("result: {:?}", result);
            //
            // assert!(true)
        }
    }
}
