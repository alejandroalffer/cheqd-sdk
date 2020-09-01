package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class WalletNotFoundException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Wallet not found";


    public WalletNotFoundException()
    {
        super(message, ErrorCode.WALLET_NOT_FOUND.value());
    }
}