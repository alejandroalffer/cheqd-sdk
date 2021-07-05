use indy::IndyError;
use indy::future::Future;
use indy::verim_ledger;

pub struct VerimLedger {}

impl VerimLedger {
    pub fn parse_query_account_resp(query_resp: &str) -> Result<String, IndyError> {
        verim_ledger::auth::parse_query_account_resp(query_resp).wait()
    }

    pub fn build_query_account(address: &str) -> Result<String, IndyError> {
        verim_ledger::auth::build_query_account(address).wait()
    }

    pub fn build_tx(pool_alias: &str,
                    sender_public_key: &str,
                    msg: &[u8],
                    account_number: u64,
                    sequence_number: u64,
                    max_gas: u64,
                    max_coin_amount: u64,
                    max_coin_denom: &str,
                    timeout_height: u64,
                    memo: &str) -> Result<Vec<u8>, IndyError> {
        verim_ledger::auth::build_tx(pool_alias, sender_public_key, msg, account_number, sequence_number, max_gas, max_coin_amount, max_coin_denom, timeout_height, memo).wait()
    }

    pub fn build_msg_create_nym(did: &str,
                                creator: &str,
                                verkey: &str,
                                alias: &str,
                                role: &str) -> Result<Vec<u8>, IndyError> {
        verim_ledger::verim::build_msg_create_nym(did, creator, verkey, alias, role).wait()
    }

    pub fn parse_msg_create_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
        verim_ledger::verim::parse_msg_create_nym_resp(commit_resp).wait()
    }

    pub fn build_msg_update_nym(did: &str,
                                creator: &str,
                                verkey: &str,
                                alias: &str,
                                role: &str,
                                id: u64) -> Result<Vec<u8>, IndyError> {
        verim_ledger::verim::build_msg_update_nym(did, creator, verkey, alias, role, id).wait()
    }

    pub fn parse_msg_update_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
        verim_ledger::verim::parse_msg_update_nym_resp(commit_resp).wait()
    }

    pub fn build_msg_delete_nym(creator: &str, id: u64) -> Result<Vec<u8>, IndyError> {
        verim_ledger::verim::build_msg_delete_nym(creator, id).wait()
    }

    pub fn parse_msg_delete_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
        verim_ledger::verim::parse_msg_delete_nym_resp(commit_resp).wait()
    }

    pub fn build_query_get_nym(id: u64) -> Result<String, IndyError> {
        verim_ledger::verim::build_query_get_nym(id).wait()
    }

    pub fn parse_query_get_nym_resp(query_resp: &str) -> Result<String, IndyError> {
        verim_ledger::verim::parse_query_get_nym_resp(query_resp).wait()
    }

    pub fn build_query_all_nym() -> Result<String, IndyError> {
        verim_ledger::verim::build_query_all_nym().wait()
    }

    pub fn parse_query_all_nym_resp(query_resp: &str) -> Result<String, IndyError> {
        verim_ledger::verim::parse_query_all_nym_resp(query_resp).wait()
    }
}