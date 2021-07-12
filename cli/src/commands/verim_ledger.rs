extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use crate::commands::*;

use crate::libindy::verim_ledger::VerimLedger;
use crate::libindy::verim_pool::VerimPool;
use crate::libindy::verim_keys::VerimKeys;

use serde_json::{Value};

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

    command!(CommandMetadata::build("create-nym", "Create nym.")
                .add_required_param("did", "DID of identity presented in Ledger")
                .add_required_param("verkey", "Verification key")
                .add_required_param("pool_alias", "Alias of pool")
                .add_required_param("key_alias", "Alias of key")
                .add_required_param("max_coin", "Max coin for transaction")
                .add_optional_param("role", "Role of identity. One of: STEWARD, TRUSTEE, TRUST_ANCHOR, ENDORSER, NETWORK_MONITOR or associated number, or empty in case of blacklisting NYM")
                .add_example("verim-ledger create-nym did=my_did verkey=my_verkey pool_alias=my_pool max_coin=500 key_alias=my_key role=TRUSTEE")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_str_param("verkey", params).map_err(error_err!())?;
        let pool_alias = get_str_param("pool_alias", params).map_err(error_err!())?;
        let key_alias = get_str_param("key_alias", params).map_err(error_err!())?;
        let role = get_str_param("role", params).map_err(error_err!())?;
        let max_coin = get_str_param("max_coin", params).map_err(error_err!())?;

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
        let key_info = VerimKeys::get_info(wallet_handle, key_alias)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let key_info_json: Value = serde_json::from_str(&key_info).unwrap();
        let account_id = key_info_json["account_id"].as_str().unwrap();
        let pubkey = key_info_json["pub_key"].as_str().unwrap();

        let request = VerimLedger::build_msg_create_nym(&did, account_id, verkey, pool_alias, role)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let (account_number, account_sequence) = get_base_account_number_and_sequence(account_id, pool_alias)?;

        let tx = VerimLedger::build_tx(
            pool_alias,
            pubkey,
            &request,
            account_number,
            account_sequence,
            10000000,
            max_coin.parse::<u64>().unwrap(),
            "token",
            39039,
            "memo"
        ).map_err(|err| handle_indy_error(err, None, None, None))?;

        let signed_tx = VerimKeys::sign(wallet_handle, key_alias, &tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = VerimPool::broadcast_tx_commit(pool_alias, &signed_tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = VerimLedger::parse_msg_create_nym_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println!("{}", parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

fn get_base_account_number_and_sequence(address: &str, pool_alias: &str) -> Result<(u64, u64), ()> {
    let query = VerimLedger::build_query_account(address)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let response = VerimPool::abci_query(pool_alias, &query)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let parsed_response = VerimLedger::parse_query_account_resp(&response)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let parsed_response: Value = match serde_json::from_str(&parsed_response) {
        Ok(json) => json,
        Err(_) => {
            println_err!("Invalid json response. Can't parse response.");
            return Err(())
        }
    };

    if parsed_response["account"].is_null() {
        println_err!("Invalid json response. Can't get account from response.");
        return Err(());
    }
    let account = parsed_response["account"].as_object().unwrap();

    if !account.contains_key("base_account") {
        println_err!("Invalid account. Can't get base account from account.");
        return Err(());
    }
    let base_account = account["base_account"].as_object().unwrap();

    if !base_account.contains_key("account_number") {
        println_err!("Invalid base account. Can't get account number from base account.");
        return Err(());
    }
    let account_number = base_account["account_number"].as_u64().unwrap();

    if !base_account.contains_key("sequence") {
        println_err!("Invalid base account. Can't get sequence from base account.");
        return Err(());
    }
    let account_sequence = base_account["sequence"].as_u64().unwrap();

    Ok((account_number, account_sequence))
}

#[cfg(test)]
#[cfg(feature = "nullpay_plugin")]
pub mod tests {
    use super::*;

    const POOL: &'static str = "pool";

    mod build {
        use super::*;
        use crate::commands::ledger::tests::create_address_and_mint_sources;
        use crate::commands::pool::tests::create_pool;

        #[test]
        pub fn build_account() {
            let ctx = setup_with_wallet_and_verim_pool();
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