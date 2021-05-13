#[derive(Clone, Debug)]
pub struct CosmosPoolConfig {
    pub alias: String,
    pub rpc_address: String,
    pub chain_id: String,
}

impl CosmosPoolConfig {
    pub fn new(alias: String, rpc_address: String, chain_id: String) -> Self {
        CosmosPoolConfig {
            alias,
            rpc_address,
            chain_id,
        }
    }
}
