extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use crate::commands::*;

use crate::libindy::cheqd_ledger::CheqdLedger;
use crate::libindy::cheqd_pool::CheqdPool;
use crate::libindy::cheqd_keys::CheqdKeys;

use serde_json::{Value};

const RESPONSE: &str = "response";
const LAST_BLOCK_HEIGHT: &str = "last_block_height";

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("cheqd-ledger", "Cheqd ledger management commands"));
}

pub mod get_account_command {
    use super::*;

    command!(CommandMetadata::build("get-account", "Query cheqd account.")
                .add_required_param("address", "Address of account")
                .add_example("cheqd-ledger get-account address=cosmos1mhl8w0xvdl3r6xf67utnqna77q0vjqgzenk7yv")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let address = get_str_param("address", params).map_err(error_err!())?;
        let pool_alias = ensure_cheqd_connected_pool(ctx)?;

        let query = CheqdLedger::build_query_account(address)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = CheqdPool::abci_query(&pool_alias, &query)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = CheqdLedger::parse_query_account_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println!("Account info: {}",parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

pub mod create_nym_command {
    use super::*;

    command!(CommandMetadata::build("create-nym", "Create nym.")
                .add_required_param("did", "DID of identity presented in Ledger")
                .add_required_param("verkey", "Verification key")
                .add_required_param("key_alias", "Alias of key")
                .add_required_param("max_coin", "Max amount coins for transaction")
                .add_required_param("max_gas", "Max amount gas for transaction")
                .add_required_param("denom", "Denom is currency for transaction")
                .add_optional_param("role", "Role of identity.")
                .add_optional_param("memo", "Memo is optional param. It has any arbitrary memo to be added to the transaction")
                .add_example("cheqd-ledger create-nym did=my_did verkey=my_verkey key_alias=my_key max_coin=500 max_gas=10000000 denom=cheq role=role memo=memo")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let did = get_str_param("did", params).map_err(error_err!())?;
        let verkey = get_str_param("verkey", params).map_err(error_err!())?;
        let key_alias = get_str_param("key_alias", params).map_err(error_err!())?;
        let max_coin = get_str_param("max_coin", params).map_err(error_err!())?
            .parse::<u64>().map_err(|_| println_err!("Invalid format of input data: max_coin must be integer"))?;
        let max_gas = get_str_param("max_gas", params).map_err(error_err!())?
            .parse::<u64>().map_err(|_| println_err!("Invalid format of input data: max_gas must be integer"))?;
        let denom = get_str_param("denom", params).map_err(error_err!())?;
        let role = get_opt_str_param("role", params).map_err(error_err!())?.unwrap_or("");
        let memo = get_opt_str_param("memo", params).map_err(error_err!())?.unwrap_or("");

        let pool_alias = ensure_cheqd_connected_pool(ctx)?;
        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
        let timeout_height = get_timeout_height(&pool_alias)?;
        let key_info = CheqdKeys::get_info(wallet_handle, key_alias)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let key_info_json: Value = serde_json::from_str(&key_info)
            .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;
        let account_id = key_info_json["account_id"].as_str().unwrap();
        let pubkey = key_info_json["pub_key"].as_str().unwrap();

        let request = CheqdLedger::build_msg_create_nym(&did, account_id, verkey, &pool_alias, role)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let (account_number, account_sequence) = get_base_account_number_and_sequence(account_id, &pool_alias)?;

        let tx = CheqdLedger::build_tx(
            &pool_alias,
            pubkey,
            &request,
            account_number,
            account_sequence,
            max_gas,
            max_coin,
            denom,
            timeout_height,
            memo
        ).map_err(|err| handle_indy_error(err, None, None, None))?;

        let signed_tx = CheqdKeys::sign(wallet_handle, key_alias, &tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = CheqdPool::broadcast_tx_commit(&pool_alias, &signed_tx)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = CheqdLedger::parse_msg_create_nym_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println!("Created NYM: {}", parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

pub mod get_nym_command {
    use super::*;

    command!(CommandMetadata::build("get-nym", "Get nym from Ledger.")
                .add_required_param("id", "Unique identifier for NYM")
                .add_example("cheqd-ledger get-nym id=0")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let id = get_str_param("id", params).map_err(error_err!())?
            .parse::<u64>().map_err(|_| println_err!("Invalid format of input data: id must be integer"))?;
        let pool_alias = ensure_cheqd_connected_pool(ctx)?;

        let query = CheqdLedger::build_query_get_nym(id)
            .map_err(|err| handle_indy_error(err, None,
                                             Some(pool_alias.as_str()), None))?;
        let response = CheqdPool::abci_query(&pool_alias, &query)
            .map_err(|err| handle_indy_error(err, None,
                                             Some(pool_alias.as_str()), None))?;
        let parsed_response = CheqdLedger::parse_query_get_nym_resp(&response)
            .map_err(|err| handle_indy_error(err, None,
                                             Some(pool_alias.as_str()), None))?;

        println_succ!("NYM info: {}",parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

pub mod bank_send_command {
    use super::*;

    command!(CommandMetadata::build("bank-send", "Send coins between accounts.")
                .add_required_param("from", "Address for sending coins")
                .add_required_param("to", "Address for getting coins")
                .add_required_param("amount", "Amount coins for send transaction")
                .add_required_param("denom", "Denom of coins")
                .add_required_param("key_alias", "Key alias")
                .add_required_param("max_coin", "Max amount coins for transaction")
                .add_required_param("max_gas", "Max amount gas for transaction")
                .add_optional_param("memo", "Memo is optional param. It has any arbitrary memo to be added to the transaction")
                .add_example("cheqd-ledger bank-send from=sender_address to=getter_address amount=100 denom=cheq key_alias=my_key max_coin=100 max_gas=10000000")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let from = get_str_param("from", params).map_err(error_err!())?;
        let to = get_str_param("to", params).map_err(error_err!())?;
        let amount = get_str_param("amount", params).map_err(error_err!())?;
        let denom = get_str_param("denom", params).map_err(error_err!())?;
        let key_alias = get_str_param("key_alias", params).map_err(error_err!())?;
        let max_coin = get_str_param("max_coin", params).map_err(error_err!())?
            .parse::<u64>().map_err(|_| println_err!("Invalid format of input data: max_coin must be integer"))?;
        let max_gas = get_str_param("max_gas", params).map_err(error_err!())?
            .parse::<u64>().map_err(|_| println_err!("Invalid format of input data: max_gas must be integer"))?;
        let memo = get_opt_str_param("memo", params).map_err(error_err!())?.unwrap_or("");
        let pool_alias = ensure_cheqd_connected_pool(ctx)?;

        let request = CheqdLedger::build_msg_send(from, to, amount, denom)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        let response = build_and_sign_and_broadcast_tx(ctx, &pool_alias, &request, key_alias, denom, max_gas, max_coin, memo)?;
        let parsed_response = CheqdLedger::parse_msg_send_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}


pub mod get_balance_command {
    use super::*;

    command!(CommandMetadata::build("get-balance", "Get balance from Ledger.")
                .add_required_param("address", "Account identifier.")
                .add_required_param("denom", "Account balance denom")
                .add_example("cheqd-ledger get-balance address=cosmos1mhl8w0xvdl3r6xf67utnqna77q0vjqgzenk7yv")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let address = get_str_param("address", params).map_err(error_err!())?;
        let denom = get_str_param("denom", params).map_err(error_err!())?;
        let pool_alias = ensure_cheqd_connected_pool(ctx)?;

        let query = CheqdLedger::build_query_balance(address, denom)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let response = CheqdPool::abci_query(&pool_alias, &query)
            .map_err(|err| handle_indy_error(err, None, None, None))?;
        let parsed_response = CheqdLedger::parse_query_balance_resp(&response)
            .map_err(|err| handle_indy_error(err, None, None, None))?;

        println_succ!("Balance info: {}",parsed_response);
        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

pub mod get_all_nym_command {
    use super::*;
    use crate::utils::table::print_list_table;

    #[derive(Debug, Serialize, Deserialize)]
    struct AllNymResponse {
        nym: Vec<Value>
    }

    command!(CommandMetadata::build("get-all-nym", "Get list of NYM transactions")
                .add_example("cheqd-ledger get-all-nym")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let pool_alias = ensure_cheqd_connected_pool(ctx)?;

        let query = CheqdLedger::build_query_all_nym()
            .map_err(|err| handle_indy_error(err, None,
                                             Some(pool_alias.as_str()), None))?;
        let response = CheqdPool::abci_query(&pool_alias, &query)
            .map_err(|err| handle_indy_error(err, None,
                                             Some(pool_alias.as_str()), None))?;
        let parsed_response = match CheqdLedger::parse_query_all_nym_resp(&response) {
                Ok(resp) => {
                    let resp: AllNymResponse = serde_json::from_str(&resp)
                        .map_err(|_| println_err!("{}", format!("Wrong data has been received from the ledger: {}", resp)))?;
                    
                    print_list_table(resp.nym.as_slice(),
                                      &[("creator", "creator"),
                                          ("id", "id"),
                                          ("alias", "alias"),
                                          ("verkey", "verkey"),
                                          ("did", "did"),
                                          ("role", "role"),],
                                      "There are no nyms");

                Ok(())
            },
                Err(err) => {
                handle_indy_error(err, None, None, None);
                Err(())
            },
        };

        trace!("execute << {:?}", parsed_response);

        Ok(())
    }
}

pub fn build_and_sign_and_broadcast_tx(ctx: &CommandContext,
                                       pool_alias: &str,
                                       request: &[u8],
                                       key_alias: &str,
                                       denom: &str,
                                       max_gas: u64,
                                       max_coin: u64,
                                       memo: &str) -> Result<String, ()> {
    let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
    let timeout_height = get_timeout_height(pool_alias)?;

    let key_info = CheqdKeys::get_info(wallet_handle, key_alias)
        .map_err(|err| handle_indy_error(err, None, None, None))?;
    let key_info_json: Value = serde_json::from_str(&key_info)
        .map_err(|err| println_err!("Invalid data has been received: {:?}", err))?;
    let pubkey = key_info_json["pub_key"].as_str().unwrap();

    let account_id = key_info_json["account_id"].as_str().unwrap();
    let (account_number, account_sequence) = get_base_account_number_and_sequence(account_id, pool_alias)?;

    let tx = CheqdLedger::build_tx(
        &pool_alias,
        pubkey,
        &request,
        account_number,
        account_sequence,
        max_gas,
        max_coin,
        denom,
        timeout_height,
        memo
    ).map_err(|err| handle_indy_error(err, None, None, None))?;

    let signed  = CheqdKeys::sign(wallet_handle, key_alias, &tx)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let resp = CheqdPool::broadcast_tx_commit(pool_alias, &signed)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    Ok(resp)
}

fn get_base_account_number_and_sequence(address: &str, pool_alias: &str) -> Result<(u64, u64), ()> {
    let query = CheqdLedger::build_query_account(address)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let response = CheqdPool::abci_query(pool_alias, &query)
        .map_err(|err| handle_indy_error(err, None, None, None))?;

    let parsed_response = CheqdLedger::parse_query_account_resp(&response)
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

pub fn get_timeout_height(pool_alias: &str) -> Result<u64, ()> {
    let info = match CheqdPool::abci_info(&pool_alias) {
        Ok(resp) => {
            println_succ!("Abci-info request result \"{}\"", resp);
            Ok(resp)
        },
        Err(err) => {
            handle_indy_error(err, None, Some(&pool_alias), None);
            Err(())
        },
    };
    let info: Value = serde_json::from_str(&info?)
        .map_err(|_| println_err!("Wrong data of Abci-info has been received"))?;

    let current_height = info[RESPONSE][LAST_BLOCK_HEIGHT].as_str().unwrap_or_default()
        .parse::<u64>().map_err(|_| println_err!("Invalid getting abci-info response. Height must be integer."))?;

    const TIMEOUT: u64 = 20;

    return Ok(current_height + TIMEOUT);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::cheqd_keys::tests::KEY_ALIAS_WITH_BALANCE;
    use crate::utils::environment::EnvironmentUtils;

    const DID: &str = "did";
    const VERKEY: &str = "verkey";
    const MAX_GAS: &str = "1000000";
    const MAX_COIN: &str = "100";
    const AMOUNT: &str = "100";
    const ROLE: &str = "TRUSTEE";
    const MEMO: &str = "memo";

    mod cheqd_ledger {
        use super::*;
        use crate::commands::cheqd_keys::tests::get_key;
        use crate::utils::environment::EnvironmentUtils;

        #[test]
        pub fn get_account() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            let key_info = get_key(&ctx);
            let account_id = key_info["account_id"].as_str().unwrap().to_string();
            {
                let cmd = get_account_command::new();
                let mut params = CommandParams::new();
                params.insert("address", account_id);
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn create_nym() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = create_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("did", DID.to_string());
                params.insert("verkey", VERKEY.to_string());
                params.insert("key_alias", KEY_ALIAS_WITH_BALANCE.to_string());
                params.insert("max_gas", MAX_GAS.to_string());
                params.insert("max_coin", MAX_COIN.to_string());
                params.insert("denom", EnvironmentUtils::cheqd_denom());
                params.insert("role", ROLE.to_string());
                params.insert("memo", MEMO.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_nym() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = get_nym_command::new();
                let mut params = CommandParams::new();
                params.insert("id", "9999999".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn bank_send() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            let key_info: Value = get_key(&ctx);
            let account_id = key_info["account_id"].as_str().unwrap().to_string();
            let key_alias = key_info["alias"].as_str().unwrap().to_string();
            {
                let cmd = bank_send_command::new();
                let mut params = CommandParams::new();
                params.insert("from", account_id.clone());
                params.insert("to", account_id);
                params.insert("amount", AMOUNT.to_string());
                params.insert("denom", EnvironmentUtils::cheqd_denom());
                params.insert("key_alias",  key_alias);
                params.insert("max_gas", MAX_GAS.to_string());
                params.insert("max_coin", MAX_COIN.to_string());
                params.insert("memo", MEMO.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_balance() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            let key_info = get_key(&ctx);
            let account_id = key_info["account_id"].as_str().unwrap().to_string();
            {
                let cmd = get_balance_command::new();
                let mut params = CommandParams::new();
                params.insert("address", account_id);
                params.insert("denom", EnvironmentUtils::cheqd_denom());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_all_nym() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            create_new_nym(&ctx);
            {
                let cmd = get_all_nym_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }
    }

    pub fn create_new_nym(ctx: &CommandContext) {
        {
            let cmd = create_nym_command::new();
            let mut params = CommandParams::new();
            params.insert("did", DID.to_string());
            params.insert("verkey", VERKEY.to_string());
            params.insert("key_alias", KEY_ALIAS_WITH_BALANCE.to_string());
            params.insert("max_gas", MAX_GAS.to_string());
            params.insert("max_coin", MAX_COIN.to_string());
            params.insert("denom", EnvironmentUtils::cheqd_denom());
            params.insert("role", ROLE.to_string());
            params.insert("memo", MEMO.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }
}