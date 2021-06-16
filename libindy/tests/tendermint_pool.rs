#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use indyrs::ErrorCode;

use utils::{constants::*, cosmos_keys, cosmos_ledger, tendermint_pool, verim_setup, verim_ledger, types::ResponseType};
use rand::prelude::*;
use rand::Rng;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add_pool {
        use super::*;

        #[test]
        fn test_add() {
            let pool_number: u16 = rand::thread_rng().gen();
            let pool_name = format!("pool{}", pool_number);
            let result = tendermint_pool::add(&pool_name, "rpc_address", "chain_id").unwrap();
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod get_pool {
        use super::*;

        #[test]
        fn test_get() {
            let pool_number: u16 = rand::thread_rng().gen();
            let pool_name = format!("pool{}", pool_number);

            tendermint_pool::add(&pool_name, "rpc_address", "chain_id").unwrap();
            let result = tendermint_pool::get_config(&pool_name).unwrap();
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod broadcast_tx_commit {
        use super::*;

        #[test]
        fn test_broadcast_tx_commit() {
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.alice_account_id).unwrap();

            // Message
            let msg = verim_ledger::build_msg_create_nym(
                "test-did",
                &setup.alice_account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = cosmos_ledger::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature
            let signed = cosmos_keys::sign(&setup.alice_key_alias, &tx).unwrap();
            let resp = tendermint_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();
            tendermint_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci{
        use super::*;

        #[test]
        fn test_abci_query() {
            unimplemented!();
        }

        #[test]
        fn test_abci_info() {
            unimplemented!();
        }
    }

}
