package com.evernym.sdk.vcx.vcx;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class InvalidLibindyParamException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Parameter passed to libindy was invalid";


    public InvalidLibindyParamException()
    {
        super(message, ErrorCode.INVALID_LIBINDY_PARAM.value());
    }
}