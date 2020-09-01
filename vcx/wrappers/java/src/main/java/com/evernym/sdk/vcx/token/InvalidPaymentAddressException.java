package com.evernym.sdk.vcx.token;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 13/06/18.
 */

public class InvalidPaymentAddressException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Invalid payment address";


    public InvalidPaymentAddressException()
    {
        super(message, ErrorCode.INVALID_PAYMENT_ADDRESS.value());
    }
}