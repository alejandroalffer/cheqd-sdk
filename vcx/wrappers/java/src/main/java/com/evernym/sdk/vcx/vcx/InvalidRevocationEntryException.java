package com.evernym.sdk.vcx.vcx;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class InvalidRevocationEntryException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Unable to Update Revocation Delta On Ledger";


    public InvalidRevocationEntryException()
    {
        super(message, ErrorCode.INVALID_REV_ENTRY.value());
    }
}