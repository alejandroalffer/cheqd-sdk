#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod utils;

use indyrs::ErrorCode;

use utils::{constants::*, cosmos_keys, cosmos_pool, types::ResponseType};

mod add_pool {
    use super::*;

    #[test]
    fn cosmos_pool_add() {
        let result = cosmos_pool::add("pool1", "rpc_address", "chain_id").unwrap();
        println!("Data: {:?} ", result);
    }
}

mod get_pool {
    use super::*;

    #[test]
    fn cosmos_pool_get() {
        cosmos_pool::add("pool1", "rpc_address", "chain_id").unwrap();
        let result = cosmos_pool::get_config("pool1").unwrap();
        println!("Data: {:?} ", result);
    }
}
