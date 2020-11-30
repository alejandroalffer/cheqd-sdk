package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class InvalidProofProposalException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "VCX Exception";


    public InvalidProofProposalException()
    {
        super(message, ErrorCode.INVALID_PROOF_PROPOSAL.value());
    }
}