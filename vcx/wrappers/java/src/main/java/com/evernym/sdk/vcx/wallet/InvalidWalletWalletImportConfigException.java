package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class InvalidWalletWalletImportConfigException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Invalid config JSON passed into wallet import";


    public InvalidWalletWalletImportConfigException()
    {
        super(message, ErrorCode.INVALID_WALLET_IMPORT_CONFIG.value());
    }
}