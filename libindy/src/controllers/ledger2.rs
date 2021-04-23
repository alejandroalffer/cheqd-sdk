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
}
