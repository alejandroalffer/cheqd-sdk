#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use utils::{constants::*, verim_ledger};

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod verim_ledger_transactions {
        use super::*;

        #[test]
        fn build_msg_create_nym() {
            let did = "0";
            let creator = "1";
            let role = "some_role";
            let alias = "some_alias";
            let verkey = "some_verkey";
            verim_ledger::build_msg_create_nym(did, creator, verkey, alias, role);
        }

        #[test]
        fn build_msg_update_nym() {
            let did = "0";
            let creator = "1";
            let role = "some_role";
            let alias = "some_alias";
            let verkey = "some_verkey";
            let id = 2;
            verim_ledger::build_msg_update_nym(did, creator, verkey, alias, role, id);
        }

        #[test]
        fn build_msg_delete_nym() {
            let creator = "1";
            let id = 2;
            verim_ledger::build_msg_delete_nym(creator, id);
        }
    }

    #[cfg(test)]
    mod e2e {
        use super::*;

        #[test]
        fn test_create_nym() {
            let msg = verim_ledger::build_msg_create_nym(
                "test-did",
                "test-creator",
                "test-verkey",
                "test-alias",
                "test-role",
            )
            .unwrap();

            // let tx = verim_ledger::
        }
    }
}
