extern crate indyrs as indy;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;

#[macro_use]
mod utils;
use utils::wallet::Wallet;
use async_std::sync::Arc;


// use crate::domain::crypto::did::DidValue;
// use crate::domain::verim_ledger::cosmos_ext::CosmosMsgExt;
// use crate::domain::verim_ledger::verimcosmos::messages::{MsgCreateNymResponse, MsgUpdateNymResponse, MsgDeleteNymResponse};
// use crate::services::{CosmosKeysService, CosmosPoolService, VerimLedgerService};
// use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
// use indy_api_types::errors::{IndyErrorKind, IndyResult};
// use indy_api_types::IndyError;

// mod low_tests {
//     use super::*;
//
//     #[test]
//     fn create_payment_address_works () {
//         let _handle = Wallet::new();
//     }
// }


#[cfg(test)]
mod test_transactions {
    use super::*;

    #[test]
    fn test_msg_create_nym() {
        
    }

}