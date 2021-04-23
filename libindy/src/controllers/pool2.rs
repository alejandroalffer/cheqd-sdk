//! Cosmos pool management service

use crate::services::Pool2Service;
use async_std::sync::Arc;

pub(crate) struct Pool2Controller {
    pool2_service: Arc<Pool2Service>,
}

impl Pool2Controller {
    pub(crate) fn new(pool2_service: Arc<Pool2Service>) -> Self {
        Self { pool2_service }
    }
}
