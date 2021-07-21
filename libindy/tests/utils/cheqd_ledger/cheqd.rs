use indyrs::{future::Future, cheqd_ledger, IndyError};

pub fn build_msg_create_nym(
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
) -> Result<Vec<u8>, IndyError> {
    cheqd_ledger::cheqd::build_msg_create_nym(did, creator, verkey, alias, role).wait()
}

pub fn parse_msg_create_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
    cheqd_ledger::cheqd::parse_msg_create_nym_resp(commit_resp).wait()
}

pub fn build_msg_update_nym(
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
    id: u64,
) -> Result<Vec<u8>, IndyError> {
    cheqd_ledger::cheqd::build_msg_update_nym(did, creator, verkey, alias, role, id).wait()
}

pub fn parse_msg_update_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
    cheqd_ledger::cheqd::parse_msg_update_nym_resp(commit_resp).wait()
}

pub fn build_msg_delete_nym(creator: &str, id: u64) -> Result<Vec<u8>, IndyError> {
    cheqd_ledger::cheqd::build_msg_delete_nym(creator, id).wait()
}

pub fn parse_msg_delete_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
    cheqd_ledger::cheqd::parse_msg_delete_nym_resp(commit_resp).wait()
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
