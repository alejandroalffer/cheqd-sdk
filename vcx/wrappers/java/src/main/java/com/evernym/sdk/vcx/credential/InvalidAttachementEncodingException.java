package com.evernym.sdk.vcx.credential;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class InvalidAttachementEncodingException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Message attachment is invalid";


    public InvalidAttachementEncodingException()
    {
        super(message, ErrorCode.INVALID_ATTACHMENT_ENCODING.value());
    }
}