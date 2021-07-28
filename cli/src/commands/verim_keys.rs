extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use crate::commands::*;

use crate::libindy::verim_keys::VerimKeys;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("verim-keys", "Verim keys management commands"));
}

pub mod add_random_command {
    use super::*;

    command!(CommandMetadata::build("add-random", "Add random key to wallet.")
                .add_required_param("alias", "Alias of key.")
                .add_example("verim-keys add-random alias=my_key")
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
                .add_required_param("alias", "Alias of key.")
                .add_required_param("mnemonic", "Mnemonic phrase for creation key.")
                .add_example("verim-keys add-from-mnemonic alias=my_key mnemonic=my_mnemonic")
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

    command!(CommandMetadata::build("get-info", "Get info about key.")
                .add_required_param("alias", "Alias of key.")
                .add_example("verim-keys get-info alias=my_key")
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
pub mod tests {
    use super::*;

    const KEY_ALIAS: &str = "key_alias";
    const MNEMONIC: &str = "mnemonic";

    mod verim_keys {
        use super::*;

        #[test]
        pub fn add_random() {
            let ctx = setup_with_wallet();
            {
                let cmd = add_random_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", KEY_ALIAS.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn add_from_mnemonic() {
            let ctx = setup_with_wallet();
            {
                let cmd = add_from_mnemonic_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", KEY_ALIAS.to_string());
                params.insert("mnemonic", MNEMONIC.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_info() {
            let ctx = setup_with_wallet_and_verim_pool();
            {
                let cmd = get_info_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", KEY_ALIAS.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }
    }

    pub fn add(ctx: &CommandContext) {
        {
            let cmd = add_random_command::new();
            let mut params = CommandParams::new();
            params.insert("alias", KEY_ALIAS.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    pub fn get_key(ctx: &CommandContext) -> serde_json::Value {
        let wallet_handle = ensure_opened_wallet_handle(ctx).unwrap();
        let key = VerimKeys::get_info(wallet_handle, KEY_ALIAS).unwrap();
        serde_json::from_str(&key).unwrap()
    }
}