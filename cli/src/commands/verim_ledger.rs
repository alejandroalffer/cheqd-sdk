extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata, DynamicCompletionType};
use crate::commands::*;

use indy::{ErrorCode, IndyError};
use crate::libindy::payment::Payment;
use crate::libindy::verim_ledger::VerimLedger;
use crate::libindy::verim_pool::VerimPool;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use crate::utils::table::print_list_table;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("verim-ledger", "Verim ledger management commands"));
}

// pub mod parse_query_command {
//     use super::*;
//
//     command!(CommandMetadata::build("parse-query-account-resp", "Create the payment address for specified payment method.")
//                 .add_required_param("query_resp", "Query response")
//                 .add_example("payment-address new query_resp=sov")
//                 .finalize()
//     );
//
//     fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
//         trace!("execute >> ctx {:?} params {:?}", ctx, params);
//         let query_resp = get_str_param("query_resp", params).map_err(error_err!())?;
//
//         let res = match VerimLedger::parse_query_account_resp(query_resp) {
//             Ok(query_resp) => {
//                 println_succ!("Query response has been taken \"{}\"", query_resp);
//                 Ok(())
//             },
//             Err(err) => {
//                 handle_indy_error(err, None, None, None);
//                 Err(())
//             },
//         };
//
//         trace!("execute << {:?}", res);
//         res
//     }
//
// }

pub mod build_query_account_command {
    use super::*;

    command!(CommandMetadata::build("build-query-account", "Build query account for verim.")
                .add_required_param("address", "Address for account")
                .add_required_param("pool_alias", "Alias of pool")
                .add_example("verim-ledger build-query-account address=sov")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let address = get_str_param("address", params).map_err(error_err!())?;
        let pool_alias = get_str_param("pool_alias", params).map_err(error_err!())?;

        let res = match VerimLedger::build_query_account(address) {
            Ok(query) =>  {
                let response = match VerimPool::abci_query(pool_alias, &query) {
                    Ok(abci_query_response) => {
                        let parse_query_account_resp = match VerimLedger::parse_query_account_resp(&abci_query_response) {
                            Ok(_) => {}
                            Err(err) => {
                                handle_indy_error(err, None, None, None);
                                ()
                            }
                        };
                        Ok(parse_query_account_resp)
                    }
                    Err(err) => {
                        handle_indy_error(err, None, None, None);
                        Err(())
                    }
                };
                Ok(response)
            },
            Err(err) => {
                handle_indy_error(err, None, None, None);
                Err(())
            },
        }.unwrap();

        trace!("execute << {:?}", res);
        res
    }

}