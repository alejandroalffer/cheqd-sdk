package com.evernym.sdk.vcx.token;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class MissingPaymentMethodException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Configuration is missing the Payment Method parameter";


    public MissingPaymentMethodException()
    {
        super(message, ErrorCode.MISSING_PAYMENT_METHOD.value());
    }
}