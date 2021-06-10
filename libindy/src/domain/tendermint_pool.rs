#[derive(Clone, Debug, Serialize)]
pub struct TendermintPoolConfig {
    pub alias: String,
    pub rpc_address: String,
    pub chain_id: String,
}

impl TendermintPoolConfig {
    pub fn new(alias: String, rpc_address: String, chain_id: String) -> Self {
        TendermintPoolConfig {
            alias,
            rpc_address,
            chain_id,
        }
    } 
}
