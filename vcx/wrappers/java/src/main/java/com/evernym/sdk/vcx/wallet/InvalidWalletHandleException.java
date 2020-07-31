package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class InvalidWalletHandleException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Invalid Wallet or Search Handle";


    public InvalidWalletHandleException()
    {
        super(message, ErrorCode.INVALID_WALLET_HANDLE.value());
    }
}