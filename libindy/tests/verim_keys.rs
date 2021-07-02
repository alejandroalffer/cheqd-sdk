#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use indyrs::ErrorCode;

use utils::{constants::*, types::ResponseType, test, verim_keys, verim_setup, verim_ledger, wallet};

mod utils;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add_random {
        use super::*;

        #[test]
        fn test_add_random() {
            let alias = "some_alias";
            let setup = verim_setup::VerimSetup::new();
            let result = verim_keys::add_random(setup.wallet_handle, alias).unwrap();
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
            let setup = verim_setup::VerimSetup::new();
            let result = verim_keys::add_from_mnemonic(setup.wallet_handle, alias, mnemonic).unwrap();
            println!("Mnemonic: {:?}, Data: {:?}", mnemonic, result);
        }
    }

    mod key_info {
        use super::*;

        #[test]
        fn test_key_info() {
            let alias = "some_alias";
            let setup = verim_setup::VerimSetup::new();
            verim_keys::add_random(setup.wallet_handle, alias).unwrap();
            let result = verim_keys::get_info(setup.wallet_handle, alias).unwrap();
            println!("Data: {:?} ", result);
        }
    }

    mod sign {
        use super::*;

        #[test]
        fn test_sign() {
            let setup = verim_setup::VerimSetup::new();

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
                &setup.pool_alias, &setup.pub_key, &msg, 0, 0, 300000, 0, "token", setup.get_timeout_height(), "memo",
            ).unwrap();

            let result = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();
            println!("Data: {:?} ", result);
        }
    }
}
