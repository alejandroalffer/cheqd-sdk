//! Ledger service for Cosmos back-end

use crate::services::Ledger2Service;
use async_std::sync::Arc;

pub(crate) struct Ledger2Controller {
    ledger2_service: Arc<Ledger2Service>,
}

impl Ledger2Controller {
    pub(crate) fn new(ledger2_service: Arc<Ledger2Service>) -> Self {
        Self { ledger2_service }
    }

    pub fn sign_request(&self) {
        unimplemented!()
    }

    pub fn submit_request(&self) {
        unimplemented!()
    }

    pub fn sign_and_submit_request(&self) {
        unimplemented!()
    }

    pub fn build_x_request(&self) {
        unimplemented!()
    }

    pub fn build_msg_bank_send(&self) {
        unimplemented!()
    }
    pub fn build_msg_create_nym(&self) {
        unimplemented!()
    }

}
