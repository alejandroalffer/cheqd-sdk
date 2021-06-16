//! Ledger service for Cosmos back-end

use async_std::sync::Arc;

use crate::services::{VerimKeysService, VerimLedgerService, VerimPoolService};

mod verim;
mod auth;

pub(crate) struct VerimLedgerController {
    verim_ledger_service: Arc<VerimLedgerService>,
    verim_pool_service: Arc<VerimPoolService>,
    verim_keys_service: Arc<VerimKeysService>,
}

impl VerimLedgerController {
    pub fn new(verim_ledger_service: Arc<VerimLedgerService>, verim_pool_service: Arc<VerimPoolService>, verim_keys_service: Arc<VerimKeysService>) -> Self {
        VerimLedgerController { verim_ledger_service, verim_pool_service, verim_keys_service }
    }
}
