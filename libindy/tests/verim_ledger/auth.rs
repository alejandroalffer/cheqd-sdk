#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use indyrs::ErrorCode;

use utils::{constants::*, verim_keys, verim_ledger, verim_pool, verim_setup, verim_ledger, types::ResponseType};

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod build_tx {
        use super::*;

        #[test]
        fn verim_ledger_build_tx() {
            unimplemented!();
        }
    }

    #[cfg(test)]
    mod auth_account {
        use super::*;

        #[test]
        fn verim_ledger_build_query_cosmos_auth_account() {
            unimplemented!();
        }

        #[test]
        fn cosmos_ledger_parse_query_cosmos_auth_account_resp() {
            unimplemented!();
        }
    }
}
