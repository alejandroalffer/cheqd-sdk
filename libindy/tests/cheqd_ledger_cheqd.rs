#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use utils::{cheqd_ledger, cheqd_pool, cheqd_setup};
use serde_json::Value;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod create_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_create_nym() {
            let setup = cheqd_setup::CheqdSetup::new();

            // Msg
            let msg = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Build, sign, broadcast tx
            let resp = setup.build_and_sign_and_broadcast_tx(&msg).unwrap();

            // Parse
            let tx_resp_parsed = cheqd_ledger::cheqd::parse_msg_create_nym_resp(&resp).unwrap();
            let tx_resp_parsed: Value = serde_json::from_str(&tx_resp_parsed).unwrap();
            println!("Tx resp: {:?}", tx_resp_parsed);

            // Query
            let query = cheqd_ledger::cheqd::build_query_get_nym(tx_resp_parsed["id"].as_u64().unwrap()).unwrap();
            let query_resp = cheqd_pool::abci_query(&setup.pool_alias, &query).unwrap();
            println!("Query response before parse: {:?}", query_resp);
            let query_resp_parsed = cheqd_ledger::cheqd::parse_query_get_nym_resp(&query_resp).unwrap();
            println!("Query response: {:?}", query_resp_parsed);

            let result: Value = serde_json::from_str(&query_resp_parsed).unwrap();
            let expected_result: Value = json!({

                    "creator": setup.account_id,
                    "id": tx_resp_parsed["id"].as_u64().unwrap(),
                    "alias": "test-alias",
                    "verkey": "test-verkey",
                    "did": "test-did",
                    "role": "test-role",

            });

            assert_eq!(expected_result, result);
        }
    }

    #[cfg(test)]
    mod update_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_update_nym() {
            let setup = cheqd_setup::CheqdSetup::new();

            ///// Create nym tx

            // Msg
            let msg_create = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Build, sign, broadcast tx
            let resp_create = setup.build_and_sign_and_broadcast_tx(&msg_create).unwrap();

            // Parse the response
            let resp_create = cheqd_ledger::cheqd::parse_msg_create_nym_resp(&resp_create).unwrap();
            let resp_create: Value = serde_json::from_str(&resp_create).unwrap();
            println!("First tx response: {:?}", resp_create);

            ///// Update nym tx

            // Message for updating
            let msg_update = cheqd_ledger::cheqd::build_msg_update_nym(
                "test-did-update",
                &setup.account_id,
                "test-verkey-update",
                "test-alias-update",
                "test-role-update",
                resp_create["id"].as_u64().unwrap()
            ).unwrap();

            // Build, sign, broadcast tx
            let resp_update = setup.build_and_sign_and_broadcast_tx(&msg_update).unwrap();

            // Parse the response of updating
            let resp_update = cheqd_ledger::cheqd::parse_msg_update_nym_resp(&resp_update).unwrap();
            println!("Update tx response: {:?}", resp_update);
            assert_eq!("{}", resp_update);

            ///// Query + assert

            let query = cheqd_ledger::cheqd::build_query_get_nym(resp_create["id"].as_u64().unwrap()).unwrap();
            let query_resp = cheqd_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let query_resp = cheqd_ledger::cheqd::parse_query_get_nym_resp(&query_resp).unwrap();
            println!("Query response: {:?}", query_resp);

            let result: Value = serde_json::from_str(&query_resp).unwrap();
            let expected_result: Value = json!({
                    "creator": setup.account_id,
                    "id": resp_create["id"].as_u64().unwrap(),
                    "alias": "test-alias-update",
                    "verkey": "test-verkey-update",
                    "did": "test-did-update",
                    "role": "test-role-update"
            });

            assert_eq!(expected_result, result);
        }
    }

    #[cfg(test)]
    mod delete_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_delete_nym() {
            let setup = cheqd_setup::CheqdSetup::new();

            ///// Create nym tx

            // Msg
            let msg_create = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Build, sign, broadcast tx
            let resp_create = setup.build_and_sign_and_broadcast_tx(&msg_create).unwrap();

            // Parse the response
            let resp_create = cheqd_ledger::cheqd::parse_msg_create_nym_resp(&resp_create).unwrap();
            let resp_create: Value = serde_json::from_str(&resp_create).unwrap();
            println!("First tx response: {:?}", resp_create);

            ///// Delete nym tx

            // Msg
            let msg_update = cheqd_ledger::cheqd::build_msg_delete_nym(
                &setup.account_id,
                resp_create["id"].as_u64().unwrap()
            ).unwrap();

            // Build, sign, broadcast tx
            let resp_update = setup.build_and_sign_and_broadcast_tx(&msg_update).unwrap();

            // Parse the resp
            let resp_update = cheqd_ledger::cheqd::parse_msg_update_nym_resp(&resp_update).unwrap();
            println!("Delete tx response: {:?}", resp_update);
            assert_eq!("{}", resp_update);

            ///// Query + assert

            let query = cheqd_ledger::cheqd::build_query_get_nym(resp_create["id"].as_u64().unwrap()).unwrap();
            let query_resp = cheqd_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let query_resp = cheqd_ledger::cheqd::parse_query_get_nym_resp(&query_resp).unwrap();
            println!("Query response: {:?}", query_resp);

            let result: Value = serde_json::from_str(&query_resp).unwrap();
            let expected_result: Value = json!(null);

            assert_eq!(expected_result, result);
        }
    }

    #[cfg(test)]
    mod get_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_get_nym() {
            // Tested in create, update, delete nym
        }
    }

    #[cfg(test)]
    mod all_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_all_nym() {
            let setup = cheqd_setup::CheqdSetup::new();

            ///// Create the first nym

            // Msg
            let msg_1 = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did-1",
                &setup.account_id,
                "test-verkey-1",
                "test-alias-1",
                "test-role-1",
            ).unwrap();

            // Build, sign, broadcast tx
            let resp_1 = setup.build_and_sign_and_broadcast_tx(&msg_1).unwrap();

            // Resp
            let resp_1 = cheqd_ledger::cheqd::parse_msg_create_nym_resp(&resp_1).unwrap();
            let resp_1: Value = serde_json::from_str(&resp_1).unwrap();
            println!("Tx 1 response: {:?}", resp_1);

            ///// Create the second nym

            // Second message
            let msg_2 = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did-2",
                &setup.account_id,
                "test-verkey-2",
                "test-alias-2",
                "test-role-2",
            ).unwrap();

            // Build, sign, broadcast tx
            let resp_2 = setup.build_and_sign_and_broadcast_tx(&msg_2).unwrap();

            // Second parse the response
            let resp_2 = cheqd_ledger::cheqd::parse_msg_create_nym_resp(&resp_2).unwrap();
            let resp_2: Value = serde_json::from_str(&resp_2).unwrap();
            println!("Tx 2 response: {:?}", resp_2);

            ///// Query + Assert

            let query = cheqd_ledger::cheqd::build_query_all_nym().unwrap();
            let query_resp = cheqd_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let query_resp = cheqd_ledger::cheqd::parse_query_all_nym_resp(&query_resp).unwrap();
            println!("Query resp: {:?}", query_resp);

            let result: Value = serde_json::from_str(&query_resp).unwrap();
            let result_nym = result["nym"].as_array().unwrap();

            let expected_nym_1: Value = json!({
                    "creator": setup.account_id,
                    "id": resp_1["id"].as_u64().unwrap(),
                    "alias": "test-alias-1",
                    "verkey": "test-verkey-1",
                    "did": "test-did-1",
                    "role": "test-role-1",
            });

            let expected_nym_2: Value = json!({
                    "creator": setup.account_id,
                    "id": resp_2["id"].as_u64().unwrap(),
                    "alias": "test-alias-2",
                    "verkey": "test-verkey-2",
                    "did": "test-did-2",
                    "role": "test-role-2",
            });

            assert!(result_nym.contains(&expected_nym_1));
            assert!(result_nym.contains(&expected_nym_2));
        }
    }
}
