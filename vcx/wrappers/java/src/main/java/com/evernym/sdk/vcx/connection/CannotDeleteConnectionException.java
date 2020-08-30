package com.evernym.sdk.vcx.connection;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class CannotDeleteConnectionException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Cannot Delete Connection. Check status of connection is appropriate to be deleted from agency";


    public CannotDeleteConnectionException()
    {
        super(message, ErrorCode.CANNOT_DELETE_CONNECTION.value());
    }
}
