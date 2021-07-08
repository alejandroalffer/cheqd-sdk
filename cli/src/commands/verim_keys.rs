extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata, DynamicCompletionType};
use crate::commands::*;

use indy::{ErrorCode, IndyError};
use crate::libindy::payment::Payment;
use crate::libindy::verim_keys::VerimKeys;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use crate::utils::table::print_list_table;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("verim-keys", "Verim keys management commands"));
}

pub mod add_random_command {
    use super::*;

    command!(CommandMetadata::build("add-random", "Add random key to wallet.")
                .add_required_param("alias", "Alias of wallet.")
                .add_example("verim-keys add-random alias=my_wallet")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
        let alias = get_str_param("alias", params).map_err(error_err!())?;

        let res = match VerimKeys::add_random(wallet_handle, alias) {
            Ok(resp) => {
                println_succ!("Random key has been added \"{}\".", resp);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, None, None);
                Err(())
            },
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod add_from_mnemonic_command {
    use super::*;

    command!(CommandMetadata::build("add-from-mnemonic", "Add key by mnemonic to wallet.")
                .add_required_param("alias", "Alias of wallet.")
                .add_required_param("mnemonic", "Mnemonic phrase for creation key.")
                .add_example("verim-keys add-from-mnemonic alias=my_wallet mnemonic=my_mnemonic")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
        let alias = get_str_param("alias", params).map_err(error_err!())?;
        let mnemonic = get_str_param("mnemonic", params).map_err(error_err!())?;

        let res = match VerimKeys::add_from_mnemonic(wallet_handle, alias, mnemonic) {
            Ok(resp) => {
                println_succ!("The Key has been added by mnemonic \"{}\" .", resp);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, None, None);
                Err(())
            },
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_info_command {
    use super::*;

    command!(CommandMetadata::build("get-info", "Get info about wallet.")
                .add_required_param("alias", "Alias of wallet.")
                .add_example("verim-keys get-info alias=my_wallet")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;
        let alias = get_str_param("alias", params).map_err(error_err!())?;

        let res = match VerimKeys::get_info(wallet_handle, alias) {
            Ok(resp) => {
                println_succ!("Get follow info \"{}\" ", resp);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, None, None);
                Err(())
            },
        };

        trace!("execute << {:?}", res);
        res
    }
}

#[cfg(test)]
#[cfg(feature = "nullpay_plugin")]
pub mod tests {
    use super::*;
    use crate::commands::common::tests::{load_null_payment_plugin, NULL_PAYMENT_METHOD};

    const POOL: &'static str = "pool";
    const RPC_ADDRESS: &'static str = "http://127.0.0.1:26657";
    const CHAIN_ID: &'static str = "verim";
    const WALLET: &str = "wallet";
    const MNEMONIC: &str = "mnemonic";

    mod verim_keys {
        use super::*;
        use crate::commands::ledger::tests::create_address_and_mint_sources;
        use crate::commands::pool::tests::create_pool;
        use crate::commands::verim_ledger::query_account_command;
        use crate::commands::verim_pool::add_command;

        #[test]
        pub fn add_random() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = add_random_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn add_from_mnemonic() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = add_from_mnemonic_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", WALLET.to_string());
                params.insert("mnemonic", MNEMONIC.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_info() {
            let ctx = setup_with_wallet_and_pool();
            {
                let cmd = get_info_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }
    }
}