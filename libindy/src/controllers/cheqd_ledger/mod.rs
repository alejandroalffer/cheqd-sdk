//! Ledger service for Cosmos back-end

use async_std::sync::Arc;

use crate::services::{CheqdLedgerService, CheqdPoolService};

mod cheqd;
mod auth;

pub(crate) struct CheqdLedgerController {
    cheqd_ledger_service: Arc<CheqdLedgerService>,
    cheqd_pool_service: Arc<CheqdPoolService>,
}

impl CheqdLedgerController {
    pub fn new(cheqd_ledger_service: Arc<CheqdLedgerService>, cheqd_pool_service: Arc<CheqdPoolService>) -> Self {
        CheqdLedgerController { cheqd_ledger_service, cheqd_pool_service }
    }
}
