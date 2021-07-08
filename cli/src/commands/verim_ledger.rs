extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata, DynamicCompletionType};
use crate::commands::*;

use indy::{ErrorCode, IndyError};
use crate::libindy::payment::Payment;
use crate::libindy::verim_ledger::VerimLedger;
use crate::libindy::verim_pool::VerimPool;
use crate::libindy::verim_keys::VerimKeys;

use serde_json::{Value as JSONValue, Value};
use serde_json::Map as JSONMap;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("verim-ledger", "Verim ledger management commands"));
}

pub mod query_account_command {
    use super::*;

    command!(CommandMetadata::build("query-account", "Query account for verim.")
                .add_required_param("address", "Address of account")
                .add_required_param("pool_alias", "Alias of pool")
                .add_example("verim-ledger query-account address=sov pool_alias=my_pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let address = get_str_param("address", params).map_err(error_err!())?;
        let pool_alias = get_str_param("pool_alias", params).map_err(error_err!())?;

        let query = VerimLedger::build_query_account(address)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = VerimPool::abci_query(pool_alias, &query)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = VerimLedger::parse_query_account_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println!("{}",parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

pub mod create_nym_command {
    use super::*;
    use crate::commands::ledger::{handle_transaction_response, parse_response_with_fees, print_response_receipts, Response};

    command!(CommandMetadata::build("create-nym", "Create nym.")
                .add_required_param("did", "DID of identity presented in Ledger")
                .add_required_param("verkey", "Verification key")
                .add_required_param("alias", "Alias of pool")
                .add_required_param("address", "Address of wallet")
                .add_required_param("pubkey", "Public key")
                .add_required_param("max_coin", "Max coin for transaction")
                .add_required_param("max_gas", "Max gas for transaction")
                .add_optional_param("role", "Role of identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, ENDORSER, NETWORK_MONITOR or associated number, or empty in case of blacklisting NYM")
                .add_example("verim-ledger build-query-account address=sov")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_str_param("verkey", params).map_err(error_err!())?;
        let alias = get_str_param("alias", params).map_err(error_err!())?;
        let address = get_str_param("address", params).map_err(error_err!())?;
        let role = get_str_param("role", params).map_err(error_err!())?;
        let pubkey = get_str_param("pubkey", params).map_err(error_err!())?;
        let max_coin = get_str_param("max_coin", params).map_err(error_err!())?;
        let max_gas = get_str_param("max_gas", params).map_err(error_err!())?;

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let mut request = VerimLedger::build_msg_create_nym(&did, "creator", verkey, alias, role)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let (account_number, account_sequence) = get_base_account_number_and_sequence(address, alias)?;
        let tx = VerimLedger::build_tx(
            alias,
            pubkey,
            &request,
            account_number,
            account_sequence,
            max_gas.parse::<u64>().unwrap(),
            max_coin.parse::<u64>().unwrap(),
            "token",
            39039,
            "memo"
        ).map_err(|err| handle_indy_error(err, None, None, None))?;

        let signed_tx = VerimKeys::sign(wallet_handle, alias, &tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = VerimPool::broadcast_tx_commit(alias, &signed_tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = VerimLedger::parse_msg_create_nym_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println!("{}", parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

fn get_base_account_number_and_sequence(address: &str, pool_alias: &str) -> Result<(u64, u64), ()> {
    let mut query = VerimLedger::build_query_account(address)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let mut response = VerimPool::abci_query(pool_alias, &query)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let mut parsed_response = VerimLedger::parse_query_account_resp(&response)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let parsed_response: Value = serde_json::from_str(&parsed_response).unwrap();
    let account = parsed_response["account"].as_object().unwrap();
    let base_account = account["base_account"].as_object().unwrap();
    let account_number = base_account["account_number"].as_u64().unwrap();
    let account_sequence = base_account["sequence"].as_u64().unwrap();

    Ok((account_number, account_sequence))
}

#[cfg(test)]
#[cfg(feature = "nullpay_plugin")]
pub mod tests {
    use super::*;
    use crate::commands::common::tests::{load_null_payment_plugin, NULL_PAYMENT_METHOD};
    use crate::commands::did::tests::SEED_MY1;

    const POOL: &'static str = "pool";

    mod build {
        use super::*;
        use crate::commands::ledger::tests::create_address_and_mint_sources;
        use crate::commands::pool::tests::create_pool;

        #[test]
        pub fn build_account() {
            let ctx = setup_with_wallet_and_pool();
            let payment_address = create_address_and_mint_sources(&ctx);
            create_pool(&ctx);
            {
                let cmd = query_account_command::new();
                let mut params = CommandParams::new();
                params.insert("address", payment_address.to_string());
                params.insert("pool_alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }
    }
}