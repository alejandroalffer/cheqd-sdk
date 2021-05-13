use indy::IndyError;
use indy::future::Future;
use indy::ledger2;
use indy::{PoolHandle, WalletHandle};

pub struct Ledger2 {}

impl Ledger2 {
    pub fn sign_and_submit_request(pool_handle: PoolHandle, wallet_handle: WalletHandle, submitter_did: &str, request_json: &str) -> Result<String, IndyError> {
        ledger2::sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json).wait()
    }

    pub fn submit_request(pool_handle: PoolHandle, request_json: &str) -> Result<String, IndyError> {
        ledger2::submit_request(pool_handle, request_json).wait()
    }

    pub fn submit_action(pool_handle: PoolHandle, request_json: &str, nodes: Option<&str>, timeout: Option<i32>) -> Result<String, IndyError> {
        ledger::submit_action(pool_handle, request_json, nodes, timeout).wait()
    }

    pub fn sign_request(wallet_handle: WalletHandle, submitter_did: &str, request_json: &str) -> Result<String, IndyError> {
        ledger2::sign_request(wallet_handle, submitter_did, request_json).wait()
    }
    //
    // pub fn multi_sign_request(wallet_handle: WalletHandle, submitter_did: &str, request_json: &str) -> Result<String, IndyError> {
    //     ledger::multi_sign_request(wallet_handle, submitter_did, request_json).wait()
    // }

    pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, IndyError> {
        ledger2::build_nym_request(submitter_did, target_did, verkey, data, role).wait()
    }

    pub fn build_get_nym_request(submitter_did: Option<&str>, target_did: &str) -> Result<String, IndyError> {
        ledger::build_get_nym_request(submitter_did, target_did).wait()
    }

}