#[derive(Clone, Debug, Serialize)]
pub struct VerimPoolConfig {
    pub alias: String,
    pub rpc_address: String,
    pub chain_id: String,
}

impl VerimPoolConfig {
    pub fn new(alias: String, rpc_address: String, chain_id: String) -> Self {
        VerimPoolConfig {
            alias,
            rpc_address,
            chain_id,
        }
    } 
}
