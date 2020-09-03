package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class RetriveExportedWalletException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Failed to retrieve exported wallet";


    public RetriveExportedWalletException()
    {
        super(message, ErrorCode.RETRIEVE_EXPORTED_WALLET.value());
    }
}