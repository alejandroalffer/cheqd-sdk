//! Ledger service for Verim back-end

use crate::domain::verim_ledger::cosmos_ext::ProstMessageExt;
use crate::domain::verim_ledger::verimcosmos::messages::MsgUpdateNym;
use crate::domain::verim_ledger::verimcosmos::messages::{MsgCreateNym, MsgCreateNymResponse};
use crate::domain::verim_ledger::verimcosmos::messages::{
    MsgDeleteNym, MsgDeleteNymResponse, MsgUpdateNymResponse,
};
use crate::domain::verim_ledger::VerimMessage;
use cosmos_sdk::bank::MsgSend;
use cosmos_sdk::proto::cosmos::base::abci::v1beta1::TxMsgData;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::{Msg, MsgType};
use cosmos_sdk::Coin;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use log_derive::logfn;

pub(crate) struct VerimLedgerService {}

impl VerimLedgerService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_bank_send(
        &self,
        sender_account_id: &str,
        recipient_account_id: &str,
        amount: u64,
        denom: &str,
    ) -> IndyResult<Msg> {
        let amount = Coin {
            amount: amount.into(),
            denom: denom.parse()?,
        };

        let msg_send = MsgSend {
            from_address: sender_account_id.parse()?,
            to_address: recipient_account_id.parse()?,
            amount: vec![amount.clone()],
        };

        Ok(msg_send.to_msg()?)
    }

    pub(crate) fn parse_msg_create_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgCreateNymResponse> {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let msg = crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgCreateNymResponse::from_bytes(&tx_msg.data[0].data)?;
        let result = MsgCreateNymResponse::from_proto(&msg);

        return Ok(result);
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_create_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
    ) -> IndyResult<Msg> {
        let msg_send = MsgCreateNym {
            creator: creator.to_string(),
            alias: alias.to_string(),
            verkey: verkey.to_string(),
            did: did.to_string(),
            role: role.to_string(),
        };

        Ok(msg_send.to_msg()?)
    }

    pub(crate) fn parse_msg_update_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgUpdateNymResponse> {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let msg = crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgUpdateNymResponse::from_bytes(&tx_msg.data[0].data)?;
        let result = MsgUpdateNymResponse::from_proto(&msg);

        return Ok(result);
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_update_nym(
        &self,
        creator: &str,
        id: u64,
        verkey: &str,
        alias: &str,
        did: &str,
        role: &str,
    ) -> IndyResult<Msg> {
        let msg_send = MsgUpdateNym {
            creator: creator.to_string(),
            id,
            alias: alias.to_string(),
            verkey: verkey.to_string(),
            did: did.to_string(),
            role: role.to_string(),
        };

        Ok(msg_send.to_msg()?)
    }

    pub(crate) fn parse_msg_delete_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgDeleteNymResponse> {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let msg = crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgDeleteNymResponse::from_bytes(&tx_msg.data[0].data)?;
        let result = MsgDeleteNymResponse::from_proto(&msg);

        return Ok(result);
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_delete_nym(&self, creator: &str, id: u64) -> IndyResult<Msg> {
        let msg_send = MsgDeleteNym {
            creator: creator.to_string(),
            id,
        };

        Ok(msg_send.to_msg()?)
    }
}
