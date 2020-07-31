package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class InvalidProofRequestException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Proof Request Passed into Libindy Call Was Invalid";


    public InvalidProofRequestException()
    {
        super(message, ErrorCode.INVALID_PROOF_REQUEST.value());
    }
}