use std::str::FromStr;

use cosmos_sdk::proto::cosmos::base::abci::v1beta1::TxMsgData;
use cosmos_sdk::rpc::endpoint::abci_query;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::{Msg, SignDoc, SignerInfo, Fee, AuthInfo};
use cosmos_sdk::tx::MsgType;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use log_derive::logfn;

use crate::domain::verim_ledger::cosmos::base::query::PageRequest;
use crate::domain::verim_ledger::prost_ext::ProstMessageExt;
use crate::domain::verim_ledger::verim::messages::MsgCreateNym;
use crate::domain::verim_ledger::verim::messages::MsgCreateNymResponse;
use crate::domain::verim_ledger::verim::messages::MsgDeleteNym;
use crate::domain::verim_ledger::verim::messages::MsgDeleteNymResponse;
use crate::domain::verim_ledger::verim::messages::MsgUpdateNym;
use crate::domain::verim_ledger::verim::messages::MsgUpdateNymResponse;
use crate::domain::verim_ledger::verim::queries::QueryAllNymRequest;
use crate::domain::verim_ledger::verim::queries::QueryAllNymResponse;
use crate::domain::verim_ledger::verim::queries::QueryGetNymRequest;
use crate::domain::verim_ledger::verim::queries::QueryGetNymResponse;
use crate::domain::verim_ledger::VerimProto;
use crate::services::VerimLedgerService;
use crate::domain::verim_ledger::cosmos::auth::{QueryAccountResponse, QueryAccountRequest};
use cosmos_sdk::{tx, Coin};
use cosmos_sdk::tendermint::block::Height;
use std::convert::TryInto;
use cosmos_sdk::crypto::PublicKey;

impl VerimLedgerService {
    pub(crate) async fn build_tx(
        &self,
        chain_id: &str,
        sender_public_key: &str,
        msg: Msg,
        account_number: u64,
        sequence_number: u64,
        max_gas: u64,
        max_coin_amount: u64,
        max_coin_denom: &str,
        timeout_height: u64,
        memo: &str,
    ) -> IndyResult<SignDoc> {
        let timeout_height: Height = timeout_height.try_into()?;

        let tx_body = tx::Body::new(vec![msg], memo, timeout_height);

        let signer_info = Self::build_signer_info(sender_public_key, sequence_number)?;

        let auth_info =
            Self::build_auth_info(max_gas, max_coin_amount, max_coin_denom, signer_info)?;

        let chain_id = chain_id.try_into()?;

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;

        Ok(sign_doc)
    }

    fn build_auth_info(
        max_gas: u64,
        max_coin: u64,
        max_coin_denom: &str,
        signer_info: SignerInfo,
    ) -> IndyResult<AuthInfo> {
        let amount = Coin {
            denom: max_coin_denom.parse()?,
            amount: max_coin.into(),
        };

        let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(amount, max_gas));

        Ok(auth_info)
    }

    fn build_signer_info(public_key: &str, sequence_number: u64) -> IndyResult<SignerInfo> {
        let public_key = rust_base58::FromBase58::from_base58(public_key)?;
        let public_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(&public_key)?;
        let public_key: PublicKey = public_key.into();

        let signer_info = SignerInfo::single_direct(Some(public_key), sequence_number);
        Ok(signer_info)
    }

    pub(crate) fn build_query_cosmos_auth_account(
        &self,
        address: &str,
    ) -> IndyResult<abci_query::Request> {
        let query_data = QueryAccountRequest::new(address.to_string());
        let path = format!("/cosmos.auth.v1beta1.Query/Account");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    pub(crate) fn parse_query_cosmos_auth_account_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryAccountResponse> {
        let result = QueryAccountResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }
}
