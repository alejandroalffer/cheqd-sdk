#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use indyrs::ErrorCode;

use utils::{constants::*, verim_keys, verim_pool, verim_setup, verim_ledger, types::ResponseType};
use serde_json::Value;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod build_tx {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_build_tx() {
            let setup = verim_setup::VerimSetup::new();

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence, 300000, 0, "token", 100000, "memo",
            ).unwrap();

            println!("Tx response: {:?}", tx);
            assert_ne!(tx.len(), 0);
        }
    }

    #[cfg(test)]
    mod query_account {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_query_account() {
            let setup = verim_setup::VerimSetup::new();
            let tx_resp = verim_ledger::auth::build_query_account(&setup.account_id).unwrap();
            let result: Value = serde_json::from_str(&tx_resp).unwrap();
            println!("Tx response: {:?}", tx_resp);

            assert!(result["prove"].as_bool().unwrap());
        }
    }
}
