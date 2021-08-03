extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use crate::commands::*;

use crate::libindy::cheqd_pool::CheqdPool as CheqdPoolLibindy;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("cheqd-pool", "Cheqd pool management commands"));
}

pub mod add_command {
    use super::*;

    command!(CommandMetadata::build("add", "Add new pool.")
                .add_required_param("alias", "Alias for pool.")
                .add_required_param("rpc_address", "RPC address of pool. Nodes need of RPC pool`s address for connection.")
                .add_required_param("chain_id", "It marks unique id of network where pool will be created.")
                .add_example("cheqd-pool add alias=my_pool rpc_address=http://127.0.0.1:26657 chain_id=cheqdnode")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let alias = get_str_param("alias", params).map_err(error_err!())?;
        let rpc_address = get_str_param("rpc_address", params).map_err(error_err!())?;
        let chain_id = get_str_param("chain_id", params).map_err(error_err!())?;

        let res = match CheqdPoolLibindy::add(alias, rpc_address, chain_id) {
            Ok(pool) => {
                println_succ!("Pool \"{}\" has been created \"{}\"", alias, pool);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, Some(alias), None);
                Err(())
            },
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod open_command {
    use super::*;
    use crate::libindy::pool::Pool;

    command!(CommandMetadata::build("open", "Open pool.")
                .add_required_param("alias", "Alias for pool.")
                .add_example("cheqd-pool open alias=my_pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let alias = get_str_param("alias", params).map_err(error_err!())?;

        match ctx.get_active_pool() {
            ActivePool::None | ActivePool::Cheqd(_) => Ok(()),   // We can safely set new pool name
            _ => {
                println_err!("Pool of other type is open. Please close it first.");
                Err(())
            }
        }?;

        // Check for existence
        match CheqdPoolLibindy::get_config(alias) {
            Ok(_handle) => {
                set_cheqd_active_pool(ctx, alias.to_string());
                println_succ!("Pool \"{}\" has been opened", alias);
                Ok(())
            }
            Err(err) => {
                handle_indy_error(err, None, Some(alias), None);
                Err(())
            }
        };

        trace!("execute <<");
        Ok(())
    }
}

pub mod close_command {
    use super::*;
    use crate::libindy::pool::Pool;

    command!(CommandMetadata::build("close", "Close pool.")
                .add_example("cheqd-pool close")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        match ctx.get_active_pool() {
            ActivePool::Cheqd(_) => {
                set_none_active_pool(ctx);
                Ok(())
            }
            ActivePool::None => {
                println_err!("There is no opened pool.");
                Err(())
            },
            _ => {
                println_err!("Pool of other type is open. Please close it using corresponding command.");
                Err(())
            }
        }?;

        trace!("execute <<");
        Ok(())
    }
}

pub mod get_config_command {
    use super::*;

    command!(CommandMetadata::build("get-config", "Get pool`s config.")
                .add_required_param("alias", "Alias for pool.")
                .add_example("cheqd-pool get-config alias=my_pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let alias = get_str_param("alias", params).map_err(error_err!())?;

        let res = match CheqdPoolLibindy::get_config(alias) {
            Ok(config) => {
                println_succ!("Pool config has been get \"{}\"", config);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, Some(alias), None);
                Err(())
            },
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod abci_info_command {
    use super::*;

    command!(CommandMetadata::build("abci-info", "The request return the application's name, version and the hash of the last commit.")
                .add_required_param("alias", "Alias for pool.")
                .add_example("cheqd-pool abci-query alias=my_pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let alias = get_str_param("alias", params).map_err(error_err!())?;

        let res = match CheqdPoolLibindy::abci_info(alias) {
            Ok(resp) => {
                println_succ!("Abci-info request has been do \"{}\"", resp);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, Some(alias), None);
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
    use crate::utils::environment::EnvironmentUtils;

    const POOL: &'static str = "pool";
    const CHAIN_ID: &'static str = "cheqdnode";

    mod cheqd_pool {
        use super::*;

        #[test]
        pub fn add_pool() {
            let ctx = setup();
            {
                let cmd = add_command::new();
                let mut params = CommandParams::new();
                params.insert("rpc_address", EnvironmentUtils::cheqd_test_pool_ip());
                params.insert("alias", POOL.to_string());
                params.insert("chain_id", CHAIN_ID.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down();
        }

        #[test]
        pub fn open_pool() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_config() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = get_config_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn abci_info() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = abci_info_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet(&ctx);
        }
    }

    pub fn create_pool(ctx: &CommandContext) {
        {
            let cmd = add_command::new();
            let mut params = CommandParams::new();
            params.insert("rpc_address", EnvironmentUtils::cheqd_test_pool_ip());
            params.insert("alias", POOL.to_string());
            params.insert("chain_id", CHAIN_ID.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }
}