use indy_api_types::IndyError;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use cosmos_sdk::rpc;
use prost::Message;

pub fn check_proofs(
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
