#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use indyrs::ErrorCode;

use utils::{constants::*, types::ResponseType, verim_keys, verim_ledger, verim_ledger::auth, verim_pool, verim_setup};

#[macro_use]
mod utils;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod config {
        use super::*;

        #[test]
        fn test_add_get_config() {
            verim_pool::add("pool1", "rpc_address", "chain_id").unwrap();
            let result = verim_pool::get_config("pool1").unwrap();
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod broadcast_tx_commit {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_broadcast_tx_commit() {
            let setup = verim_setup::VerimSetup::new();

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.alice_account_id).unwrap();

            // Message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.alice_account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = auth::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg, account_number, account_sequence, 300000, 0, "token", 100000, "memo",
            ).unwrap();

            // Sign
            let signed = verim_keys::sign(&setup.alice_key_alias, &tx).unwrap();

            // Broadcast
            let resp = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci_query {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_abci_query() {
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod abci_info {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_abci_info() {
            unimplemented!()
        }
    }
}
