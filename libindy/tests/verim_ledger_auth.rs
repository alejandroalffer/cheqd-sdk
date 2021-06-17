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

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod build_tx {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_build_tx() {
            unimplemented!();
        }
    }

    #[cfg(test)]
    mod query_account {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_query_account() {
            unimplemented!();
        }
    }
}
