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
        let res = match CheqdPoolLibindy::get_config(alias) {
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

        trace!("execute << {:?}", res);
        res
    }
}

pub mod close_command {
    use super::*;

    command!(CommandMetadata::build("close", "Close pool.")
                .add_example("cheqd-pool close")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let res = match ctx.get_active_pool() {
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
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_config_command {
    use super::*;

    command!(CommandMetadata::build("get-config", "Get pool`s config.")
                .add_example("cheqd-pool get-config")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let pool_alias = ensure_cheqd_connected_pool(ctx)?;

        let res = match CheqdPoolLibindy::get_config(&pool_alias) {
            Ok(config) => {
                println_succ!("Available pools: \"{}\"", config);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, Some(&pool_alias), None);
                Err(())
            },
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod get_all_config_command {
    use super::*;
    use crate::utils::table::print_list_table;

    command!(CommandMetadata::build("get-all-config", "Get list configs of pools.")
                .add_example("cheqd-pool get-all-config")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let res = match CheqdPoolLibindy::get_all_config() {
            Ok(resp) => {
                let resp: Vec<serde_json::Value> = serde_json::from_str(&resp)
                    .map_err(|_| println_err!("{}", format!("Wrong data has been received: {}", resp)))?;

                print_list_table(&resp,
                                 &[("alias", "Alias"),
                                     ("chain_id", "Chain id"),
                                     ("rpc_address", "RPC address")],
                                 "There are no configs");
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

pub mod abci_info_command {
    use super::*;

    command!(CommandMetadata::build("abci-info", "The request return the application's name, version and the hash of the last commit.")
                .add_example("cheqd-pool abci-query")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let pool_alias = ensure_cheqd_connected_pool(ctx)?;

        let res = match CheqdPoolLibindy::abci_info(&pool_alias) {
            Ok(resp) => {
                println_succ!("Abci-info request result \"{}\"", resp);
                Ok(())
            },
            Err(err) => {
                handle_indy_error(err, None, Some(&pool_alias), None);
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
    const CHAIN_ID: &'static str = "cheqd";

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
            tear_down();
        }

        #[test]
        pub fn open_pool() {
            let ctx = setup_with_wallet();
            create_pool(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn close_pool() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = close_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_config() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = get_config_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn get_all_config() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = get_all_config_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn abci_info() {
            let ctx = setup_with_wallet_and_cheqd_pool();
            {
                let cmd = abci_info_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
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

    pub fn create_and_open_pool(ctx: &CommandContext) {
        {
            let cmd = add_command::new();
            let mut params = CommandParams::new();
            params.insert("rpc_address", EnvironmentUtils::cheqd_test_pool_ip());
            params.insert("alias", POOL.to_string());
            params.insert("chain_id", CHAIN_ID.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
        {
            let cmd = open_command::new();
            let mut params = CommandParams::new();
            params.insert("alias", POOL.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }
}