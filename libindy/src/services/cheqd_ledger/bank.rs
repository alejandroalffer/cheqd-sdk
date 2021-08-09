use std::str::FromStr;

use cosmos_sdk::rpc::endpoint::abci_query;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::Msg;
use cosmos_sdk::tx::MsgType;
use indy_api_types::errors::IndyResult;
use log_derive::logfn;

use crate::domain::cheqd_ledger::base::query::PageRequest;
use crate::domain::cheqd_ledger::prost_ext::ProstMessageExt;
use crate::domain::cheqd_ledger::cheqd::messages::MsgCreateNym;
use crate::domain::cheqd_ledger::cheqd::messages::MsgCreateNymResponse;
use crate::domain::cheqd_ledger::cheqd::messages::MsgDeleteNym;
use crate::domain::cheqd_ledger::cheqd::messages::MsgDeleteNymResponse;
use crate::domain::cheqd_ledger::cheqd::messages::MsgUpdateNym;
use crate::domain::cheqd_ledger::cheqd::messages::MsgUpdateNymResponse;
use crate::domain::cheqd_ledger::cheqd::queries::QueryAllNymRequest;
use crate::domain::cheqd_ledger::cheqd::queries::QueryAllNymResponse;
use crate::domain::cheqd_ledger::cheqd::queries::QueryGetNymRequest;
use crate::domain::cheqd_ledger::cheqd::queries::QueryGetNymResponse;
use crate::domain::cheqd_ledger::CheqdProto;
use crate::domain::cheqd_ledger::bank::MsgSend;

pub(crate) struct CheqdBankService {}

impl CheqdXferService {
    #[logfn(Info)]
    pub(crate) fn indy_cheqd_ledger_bank_build_msg_send(
        &self,
        from_address: &str,
        to_address: &str,
        amount: Coin
    ) -> IndyResult<Msg> {
        let msg_send = MsgSend::new(
            from_address.to_string(),
            to_address.to_string(),
            amount
        );

        Ok(msg_send.to_proto().to_msg()?)
    }
}
