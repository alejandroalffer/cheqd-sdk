package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class RetriveDeadDropException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Failed to retrieve dead drop payload";


    public RetriveDeadDropException()
    {
        super(message, ErrorCode.RETRIEVE_DEAD_DROP.value());
    }
}