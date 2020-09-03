package com.evernym.sdk.vcx.schema;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class DuplicateSchemaException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Duplicate Schema: Ledger Already Contains Schema For Given DID, Version, and Name Combination";


    public DuplicateSchemaException()
    {
        super(message, ErrorCode.DUPLICATE_SCHEMA.value());
    }
}