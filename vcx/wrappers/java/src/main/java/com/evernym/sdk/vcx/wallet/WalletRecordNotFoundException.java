package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class WalletRecordNotFoundException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Wallet record not found";


    public WalletRecordNotFoundException()
    {
        super(message, ErrorCode.WALLET_RECORD_NOT_FOUND.value());
    }
}