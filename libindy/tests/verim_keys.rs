#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use indyrs::ErrorCode;

use utils::{constants::*, types::ResponseType, test, verim_keys, verim_setup, verim_ledger, wallet};

mod utils;

fn config(name: &str) -> String {
    json!({ "id": name }).to_string()
}

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add_random {
        use super::*;

        #[test]
        fn test_add_random() {
            let alias = "some_alias";
            let setup = verim_setup::VerimSetup::new();
            let config = config(&setup.name);
            let (wallet_handle, _) = wallet::create_and_open_default_wallet(&config).unwrap();
            let result = verim_keys::add_random(wallet_handle, alias).unwrap();
            println!("Data: {:?} ", result);

            wallet::close_wallet(wallet_handle).unwrap();
            test::cleanup_storage("test_add_random");
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
            let config = config(&setup.name);
            let (wallet_handle, _) = wallet::create_and_open_default_wallet(&config).unwrap();
            let result = verim_keys::add_from_mnemonic(wallet_handle, alias, mnemonic).unwrap();
            println!("Mnemonic: {:?}, Data: {:?}", mnemonic, result);

            wallet::close_wallet(wallet_handle).unwrap();
            test::cleanup_storage("test_add_from_mnemonic");
        }
    }

    mod key_info {
        use super::*;

        #[test]
        fn test_key_info() {
            let alias = "some_alias";
            let setup = verim_setup::VerimSetup::new();
            let config = config(&setup.name);
            let (wallet_handle, _) = wallet::create_and_open_default_wallet(&config).unwrap();
            verim_keys::add_random(wallet_handle, alias).unwrap();
            let result = verim_keys::get_info(wallet_handle, alias).unwrap();
            println!("Data: {:?} ", result);

            wallet::close_wallet(wallet_handle).unwrap();
            test::cleanup_storage("test_key_info");
        }
    }

    mod sign {
        use super::*;

        #[test]
        fn test_sign() {
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
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg, account_number, account_sequence, 300000, 0, "token", 100000, "memo",
            ).unwrap();

            let alias = "some_alias";
            let setup = verim_setup::VerimSetup::new();
            let config = config(&setup.name);
            let (wallet_handle, _) = wallet::create_and_open_default_wallet(&config).unwrap();
            verim_keys::add_random(wallet_handle, alias).unwrap();
            let result = verim_keys::sign(wallet_handle, alias, &tx).unwrap();
            println!("Data: {:?} ", result);

            wallet::close_wallet(wallet_handle).unwrap();
            test::cleanup_storage("test_sign");
        }
    }
}
