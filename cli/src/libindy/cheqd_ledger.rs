use indy::IndyError;
use indy::future::Future;
use indy::cheqd_ledger;

pub struct CheqdLedger {}

impl CheqdLedger {
    pub fn parse_query_account_resp(query_resp: &str) -> Result<String, IndyError> {
        cheqd_ledger::auth::parse_query_account_resp(query_resp).wait()
    }

    pub fn build_query_account(address: &str) -> Result<String, IndyError> {
        cheqd_ledger::auth::build_query_account(address).wait()
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
        cheqd_ledger::auth::build_tx(pool_alias, sender_public_key, msg, account_number, sequence_number, max_gas, max_coin_amount, max_coin_denom, timeout_height, memo).wait()
    }

    pub fn build_msg_create_nym(did: &str,
                                creator: &str,
                                verkey: &str,
                                alias: &str,
                                role: &str) -> Result<Vec<u8>, IndyError> {
        cheqd_ledger::cheqd::build_msg_create_nym(did, creator, verkey, alias, role).wait()
    }

    pub fn parse_msg_create_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
        cheqd_ledger::cheqd::parse_msg_create_nym_resp(commit_resp).wait()
    }

    pub fn build_msg_send(from: &str,
                          to: &str,
                          amount: &str,
                          denom: &str) -> Result<Vec<u8>, IndyError> {
        cheqd_ledger::bank::build_msg_send(from, to, amount, denom).wait()
    }

    pub fn parse_msg_send_resp(resp: &str) -> Result<String, IndyError> {
        cheqd_ledger::bank::parse_msg_send_resp(resp).wait()
    }

    pub fn build_query_balance(address: &str,
                               denom: &str) -> Result<String, IndyError> {
        cheqd_ledger::bank::build_query_balance(address, denom).wait()
    }

    pub fn parse_query_balance_resp(resp: &str) -> Result<String, IndyError> {
        cheqd_ledger::bank::parse_query_balance_resp(resp).wait()
    }

    pub fn build_query_get_nym(id: u64) -> Result<String, IndyError> {
        cheqd_ledger::cheqd::build_query_get_nym(id).wait()
    }

    pub fn parse_query_get_nym_resp(query_resp: &str) -> Result<String, IndyError> {
        cheqd_ledger::cheqd::parse_query_get_nym_resp(query_resp).wait()
    }
    
    pub fn build_query_all_nym() -> Result<String, IndyError> {
        cheqd_ledger::cheqd::build_query_all_nym().wait()
    }
    
    pub fn parse_query_all_nym_resp(query_resp: &str) -> Result<String, IndyError> {
        cheqd_ledger::cheqd::parse_query_all_nym_resp(query_resp).wait()
    }
}