#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod utils;

use indyrs::{ErrorCode};

use utils::{constants::*, cosmos_keys, types::ResponseType};

mod add_keys {
    use super::*;

    #[test]
    fn indy_add_random() {
        let alias = "some_alias";
        let result = cosmos_keys::add_random(alias).unwrap();
        println!("Data: {:?} ", result);
    }

    #[test]
    fn indy_add_from_mnemonic() {
        let alias = "some_alias";
        let mnemonic = "some_mnemonic";
        let result = cosmos_keys::add_from_mnemonic(alias, mnemonic).unwrap();
        println!("Mnemonic: {:?}, Data: {:?}", mnemonic, result);
    }
}

mod get_keys {
    use super::*;

    #[test]
    fn indy_key_info() {
        let alias = "some_alias";
        cosmos_keys::add_random(alias).unwrap();
        let result = cosmos_keys::key_info(alias).unwrap();
        println!("Data: {:?} ", result);
    }
}
