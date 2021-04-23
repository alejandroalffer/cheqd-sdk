//! Cosmos key management service

use crate::services::KeysService;
use async_std::sync::Arc;

pub(crate) struct KeysController {
    keys_service: Arc<KeysService>,
}

impl KeysController {
    pub(crate) fn new(keys_service: Arc<KeysService>) -> Self {
        Self { keys_service }
    }
}
