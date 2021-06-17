#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use indyrs::ErrorCode;

use utils::{constants::*, types::ResponseType, verim_keys};

mod utils;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add_random {
        use super::*;

        #[test]
        fn test_add_random() {
            let alias = "some_alias";
            let result = verim_keys::add_random(alias).unwrap();
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
            let result = verim_keys::add_from_mnemonic(alias, mnemonic).unwrap();
            println!("Mnemonic: {:?}, Data: {:?}", mnemonic, result);
        }
    }

    mod key_info {
        use super::*;

        #[test]
        fn test_key_info() {
            let alias = "some_alias";
            verim_keys::add_random(alias).unwrap();
            let result = verim_keys::get_info(alias).unwrap();
            println!("Data: {:?} ", result);
        }
    }
}