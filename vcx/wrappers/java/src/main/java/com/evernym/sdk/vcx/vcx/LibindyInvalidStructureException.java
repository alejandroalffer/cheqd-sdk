package com.evernym.sdk.vcx.vcx;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class LibindyInvalidStructureException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Object (json, config, key, credential and etc...) passed to libindy has invalid structure";


    public LibindyInvalidStructureException()
    {
        super(message, ErrorCode.LIBINDY_INVALID_STRUCTURE.value());
    }
}