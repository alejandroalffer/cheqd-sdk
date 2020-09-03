package com.evernym.sdk.vcx.connection;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class ConnectionAlreadyExistsException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Connection invitation has been already accepted. You have to use another invitation to set up a new connection.";


    public ConnectionAlreadyExistsException()
    {
        super(message, ErrorCode.CONNECTION_ALREADY_EXISTS.value());
    }
}
