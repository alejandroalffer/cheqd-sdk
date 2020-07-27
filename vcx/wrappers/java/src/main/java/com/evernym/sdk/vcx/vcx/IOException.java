package com.evernym.sdk.vcx.vcx;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class IOException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "IO Error, possibly creating a backup wallet";


    public IOException()
    {
        super(message, ErrorCode.IOERROR.value());

    }
}