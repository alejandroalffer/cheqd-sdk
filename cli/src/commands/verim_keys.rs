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

    command_group!(CommandGroupMetadata::new("verim-key", "Verim key management commands"));
}

pub mod add_random_command {
    use super::*;

    command!(CommandMetadata::build("add-random", "Add random key to wallet handle.")
                .add_required_param("alias", "Alias for pool.")
                .add_example("verim-pool add-random alias=my_pool")
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

    command!(CommandMetadata::build("add-from-mnemonic", "Add key by mnemonic to wallet handle.")
                .add_required_param("alias", "Alias for key.")
                .add_required_param("mnemonic", "Mnemonic phrase for creation key.")
                .add_example("verim-pool add-from-mnemonic alias=my_pool mnemonic=my_mnemonic")
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

    command!(CommandMetadata::build("get-info", "Get info about key by mnemonic to wallet handle.")
                .add_required_param("alias", "Alias for key.")
                .add_example("verim-pool get-info alias=my_pool")
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