use std::str::FromStr;

use cosmos_sdk::rpc;
use cosmos_sdk::rpc::endpoint::abci_query;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::Msg;
use cosmos_sdk::tx::MsgType;
use indy_api_types::IndyError;
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use log_derive::logfn;
use prost::Message;

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
use crate::services::CheqdLedgerService;
use crate::domain::cheqd_ledger::cheqd::models::Nym;
use ics23::CommitmentProof;
use ics23::commitment_proof::Proof;

impl CheqdLedgerService {
    #[logfn(Info)]
    pub(crate) fn cheqd_build_msg_create_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
    ) -> IndyResult<Msg> {
        let msg_send = MsgCreateNym::new(
            creator.to_string(),
            alias.to_string(),
            verkey.to_string(),
            did.to_string(),
            role.to_string(),
        );

        Ok(msg_send.to_proto().to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_msg_create_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgCreateNymResponse> {
        self.parse_msg_resp(resp)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_build_msg_update_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
        id: u64,
    ) -> IndyResult<Msg> {
        let msg_send = MsgUpdateNym::new(
            creator.to_string(),
            id,
            alias.to_string(),
            verkey.to_string(),
            did.to_string(),
            role.to_string(),
        );

        Ok(msg_send.to_proto().to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_msg_update_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgUpdateNymResponse> {
        self.parse_msg_resp(resp)
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_delete_nym(&self, creator: &str, id: u64) -> IndyResult<Msg> {
        let msg_send = MsgDeleteNym {
            creator: creator.to_string(),
            id,
        };

        Ok(msg_send.to_proto().to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_msg_delete_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgDeleteNymResponse> {
        self.parse_msg_resp(resp)
    }

    #[logfn(Info)]
    pub(crate) fn build_query_get_nym_without_proof(&self, id: u64) -> IndyResult<abci_query::Request> {
        let query_data = QueryGetNymRequest::new(id);
        let path = format!("/cheqdid.cheqdnode.cheqd.Query/Nym");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    #[logfn(Info)]
    pub(crate) fn build_query_get_nym(
        &self,
        id: u64,
    ) -> IndyResult<abci_query::Request> {
        let mut query_data = "Nym-value-".as_bytes().to_vec();
        query_data.extend_from_slice(&id.to_be_bytes());
        let path = format!("/store/cheqd/key");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req = abci_query::Request::new(Some(path), query_data, None, true);
        Ok(req)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_query_get_nym_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<Option<Nym>> {
        let result = if !resp.response.value.is_empty() {
            Some(Nym::from_proto_bytes(&resp.response.value)?)
        } else { None };
        match self.check_proofs(resp.clone()) {
            Ok(()) => Ok(result),
            Err(ex) => Err(ex)
        }
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_build_query_all_nym(
        &self,
        pagination: Option<PageRequest>,
    ) -> IndyResult<abci_query::Request> {
        let query_data = QueryAllNymRequest::new(pagination);
        let path = format!("/cheqdid.cheqdnode.cheqd.Query/NymAll");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_query_all_nym_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryAllNymResponse> {
        let result = QueryAllNymResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }


    fn check_proofs(
        &self,
        result: rpc::endpoint::abci_query::Response,
    ) -> IndyResult<()> {
        //////////////////////////// 0st proof

        let proof_op_0 = &result.response.proof.clone().unwrap().ops[0];
        let proof_0_data_decoded =
            ics23::CommitmentProof::decode(proof_op_0.data.as_slice()).unwrap();

        let proof_op_1 = &result.response.proof.unwrap().ops[1];
        let proof_1_data_decoded =
            ics23::CommitmentProof::decode(proof_op_1.data.as_slice()).unwrap();

        let proof_0_root = if let Some(ics23::commitment_proof::Proof::Exist(ex)) =
        proof_1_data_decoded.proof.clone()
        {
            ex.value
        } else {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidStructure,
                format!(
                    "Commitment proof has an incorrect format {}",
                    serde_json::to_string(proof_op_1)?
                ),
            ));
        };

        let is_proof_correct = match proof_0_data_decoded.proof {
            Some(ics23::commitment_proof::Proof::Exist(_)) => {
                ics23::verify_membership(
                    &proof_0_data_decoded,
                    &ics23::iavl_spec(),
                    &proof_0_root,
                    &proof_op_0.key,
                    &result.response.value,
                )
            }
            Some(ics23::commitment_proof::Proof::Nonexist(_)) => {
                ics23::verify_non_membership(
                    &proof_0_data_decoded,
                    &ics23::iavl_spec(),
                    &proof_0_root,
                    &proof_op_0.key
                )
            }
            _ => {false}
        };

        if !is_proof_correct {
            return Err(IndyError::from_msg(
                IndyErrorKind::ProofRejected,
                format!(
                    "Commitment proof 0 is incorrect {}",
                    serde_json::to_string(proof_op_0)?
                ),
            ));
        }

        // Should be output from light client
        let proof_1_root = if let Some(ics23::commitment_proof::Proof::Exist(ex)) =
        proof_1_data_decoded.proof.clone()
        {
            ics23::calculate_existence_root(&ex).unwrap()
        } else {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidStructure,
                format!(
                    "Commitment proof has an incorrect format {}",
                    serde_json::to_string(proof_op_1)?
                ),
            ));
        };

        if !ics23::verify_membership(
            &proof_1_data_decoded,
            &ics23::tendermint_spec(),
            &proof_1_root,
            &proof_op_1.key,
            &proof_0_root,
        ) {
            return Err(IndyError::from_msg(
                IndyErrorKind::ProofRejected,
                format!(
                    "Commitment proof 1 is incorrect {}",
                    serde_json::to_string(proof_op_1)?
                ),
            ));
        }

        Ok(())
    }

}
