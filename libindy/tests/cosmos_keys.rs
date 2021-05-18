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
        cosmos_keys::add_random(alias).unwrap();
    }

}