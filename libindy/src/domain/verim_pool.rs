#[derive(Clone, Debug, Serialize)]
pub struct PoolConfig {
    pub alias: String,
    pub rpc_address: String,
    pub chain_id: String,
}

impl PoolConfig {
    pub fn new(alias: String, rpc_address: String, chain_id: String) -> Self {
        PoolConfig {
            alias,
            rpc_address,
            chain_id,
        }
    } 
}
