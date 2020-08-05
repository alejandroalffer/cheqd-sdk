package com.evernym.sdk.vcx.schema;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class UnknownSchemaRejectionException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Unknown Rejection of Schema Creation, refer to libindy documentation";


    public UnknownSchemaRejectionException()
    {
        super(message, ErrorCode.UNKNOWN_SCHEMA_REJECTION.value());
    }
}