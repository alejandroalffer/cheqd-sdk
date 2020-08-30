package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class WalletCreateException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "IError Creating a wallet";


    public WalletCreateException()
    {
        super(message, ErrorCode.INVALID_WALLET_CREATION.value());
    }
}