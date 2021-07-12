extern crate regex;
extern crate chrono;

use crate::command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use crate::commands::*;

use crate::libindy::verim_pool::VerimPool;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("verim-pool", "Verim pool management commands"));
}

pub mod add_command {
    use super::*;

    command!(CommandMetadata::build("add", "Add new pool.")
                .add_required_param("alias", "Alias for pool.")
                .add_required_param("rpc_address", "RPC address of pool. Nodes need of RPC pool`s address for connection.")
                .add_required_param("chain_id", "It marks unique id of network where pool will be created.")
                .add_example("verim-pool add alias=my_pool rpc_address=http://127.0.0.1:26657 chain_id=verim")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let alias = get_str_param("alias", params).map_err(error_err!())?;
        let rpc_address = get_str_param("rpc_address", params).map_err(error_err!())?;
        let chain_id = get_str_param("chain_id", params).map_err(error_err!())?;

        let res = match VerimPool::add(alias, rpc_address, chain_id) {
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

pub mod get_config_command {
    use super::*;

    command!(CommandMetadata::build("get-config", "Get pool`s config.")
                .add_required_param("alias", "Alias for pool.")
                .add_example("verim-pool get-config alias=my_pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let alias = get_str_param("alias", params).map_err(error_err!())?;

        let res = match VerimPool::get_config(alias) {
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
                .add_example("verim-pool abci-query alias=my_pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);
        let alias = get_str_param("alias", params).map_err(error_err!())?;

        let res = match VerimPool::abci_info(alias) {
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

    const POOL: &'static str = "pool";
    const RPC_ADDRESS: &'static str = "http://127.0.0.1:26657";
    const CHAIN_ID: &'static str = "verimnode";

    mod verim_pool {
        use super::*;

        #[test]
        pub fn add_pool() {
            let ctx = setup();
            {
                let cmd = add_command::new();
                let mut params = CommandParams::new();
                params.insert("rpc_address", RPC_ADDRESS.to_string());
                params.insert("alias", POOL.to_string());
                params.insert("chain_id", CHAIN_ID.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down();
        }

        #[test]
        pub fn get_config() {
            let ctx = setup_with_wallet_and_verim_pool();
            {
                let cmd = get_config_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet_and_verim_pool(&ctx);
        }

        #[test]
        pub fn abci_info() {
            let ctx = setup_with_wallet_and_verim_pool();
            {
                let cmd = abci_info_command::new();
                let mut params = CommandParams::new();
                params.insert("alias", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            assert!(true);

            tear_down_with_wallet_and_verim_pool(&ctx);
        }
    }

    pub fn create_pool(ctx: &CommandContext) {
        {
            let cmd = add_command::new();
            let mut params = CommandParams::new();
            params.insert("rpc_address", RPC_ADDRESS.to_string());
            params.insert("alias", POOL.to_string());
            params.insert("chain_id", CHAIN_ID.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }
}