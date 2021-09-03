#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use indyrs::ErrorCode;

#[cfg(feature = "cheqd")]
use utils::{constants::*, types::ResponseType, test, cheqd_keys, cheqd_setup, cheqd_ledger, wallet};
use serde_json::Value;

mod utils;

#[cfg(feature = "cheqd")]
mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add_random {
        use super::*;

        #[test]
        fn test_add_random() {
            let alias = "some_alias";
            let setup = cheqd_setup::CheqdSetup::new();
            let result = cheqd_keys::add_random(setup.wallet_handle, alias).unwrap();
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod add_from_mnemonic {
        use super::*;

        #[test]
        fn test_add_from_mnemonic() {
            let alias = "some_alias_2";
            let mnemonic = "some_mnemonic";
            let setup = cheqd_setup::CheqdSetup::new();
            let result = cheqd_keys::add_from_mnemonic(setup.wallet_handle, alias, mnemonic).unwrap();
            println!("Mnemonic: {:?}, Data: {:?}", mnemonic, result);
        }
    }

    mod key_info {
        use super::*;

        #[test]
        fn test_key_info() {
            let alias = "some_alias";
            let setup = cheqd_setup::CheqdSetup::new();
            cheqd_keys::add_random(setup.wallet_handle, alias).unwrap();
            let result = cheqd_keys::get_info(setup.wallet_handle, alias).unwrap();
            println!("Data: {:?} ", result);
        }

        #[test]
        fn test_get_list_keys() {
            let alias_1 = "some_alias_1";
            let alias_2 = "some_alias_2";

            let setup = cheqd_setup::CheqdSetup::new();

            let key_1 = cheqd_keys::add_random(setup.wallet_handle, alias_1).unwrap();
            let key_2 = cheqd_keys::add_random(setup.wallet_handle, alias_2).unwrap();

            let result = cheqd_keys::get_list_keys(setup.wallet_handle).unwrap();
            let result: Vec<Value> = serde_json::from_str(&result).unwrap();

            let expect_key_1: Value = serde_json::from_str(&key_1).unwrap();
            let expect_key_2: Value = serde_json::from_str(&key_2).unwrap();

            assert!(result.contains(&expect_key_1));
            assert!(result.contains(&expect_key_2));

            println!("Data: {:?} ", result);
        }
    }

    mod sign {
        use super::*;

        #[test]
        fn test_sign() {
            let setup = cheqd_setup::CheqdSetup::new();

            // Message
            let msg = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = cheqd_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, 0, 0, 300000, 0, "cheq", setup.get_timeout_height(), "memo",
            ).unwrap();

            let result = cheqd_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();
            println!("Data: {:?} ", result);
        }
    }
}
