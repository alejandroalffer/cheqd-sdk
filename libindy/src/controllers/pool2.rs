//! Cosmos pool management service

use crate::services::{Pool2Service, KeysService};
use async_std::sync::Arc;
use indy_api_types::errors::IndyResult;
use cosmos_sdk::tx::Msg;
use cosmos_sdk::rpc;

pub(crate) struct Pool2Controller {
    pool2_service: Arc<Pool2Service>,
    keys_service: Arc<KeysService>,
    pool_url: String
}

impl Pool2Controller {
    pub(crate) fn new(pool2_service: Arc<Pool2Service>, keys_service: Arc<KeysService>) -> Self {
        Self {
            pool2_service,
            keys_service,
            pool_url: String::new()
        }
    }

    pub(crate) async fn set_pool(
        &mut self,
        pool_url: &str
    ) -> IndyResult<()> {
        self.pool_url = String::from(pool_url);
        Ok(())
    }

    // pub(crate) async fn build_sign_broadcast_tx_commit(
    //     &self,
    //     alias: &str,
    //     pub_key: &str,
    //     msg: &Msg,
    //     pool_url: &str,
    // ) -> IndyResult<IndyResult<rpc::endpoint::broadcast::tx_commit::Response> > {
    //     trace!(
    //         "build_sign_broadcast_tx_commit > alias {:?}",
    //         alias
    //     );
    //     let tx = self.pool2_service
    //         .build_tx(
    //             pub_key,
    //             vec![msg],
    //             "verimcosmos",
    //             9, // What is it?
    //             0,
    //             300000,
    //             0u64,
    //             "stake",
    //             39090,
    //             "memo",
    //         )
    //         .unwrap();
    //
    //     let signed = self.keys_service.sign(alias, tx).await.unwrap();
    //
    //     // Broadcast
    //     let res = self.pool2_service
    //         // .broadcast_tx_commit(signed, "http://localhost:26657")
    //         .broadcast_tx_commit(signed, pool_url)
    //         .await;
    //     trace!("build_sign_broadcast_tx_commit < {:?}", res);
    //     res
    // }

}
