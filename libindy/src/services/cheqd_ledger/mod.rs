//! Ledger service for Cheqd back-end

use cosmos_sdk::proto::cosmos::base::abci::v1beta1::TxMsgData;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use log_derive::logfn;

use crate::domain::cheqd_ledger::prost_ext::ProstMessageExt;
use crate::domain::cheqd_ledger::CheqdProto;

mod auth;
mod cheqd;

pub(crate) struct CheqdLedgerService {}

impl CheqdLedgerService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    #[logfn(Info)]
    fn parse_msg_resp<R>(&self, resp: &Response) -> IndyResult<R>
        where
            R: CheqdProto,
    {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let result = R::from_proto_bytes(&tx_msg.data[0].data)?;

        return Ok(result);
    }
}
