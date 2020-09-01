package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class DuplicateWalletRecordException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Record already exists in the wallet";


    public DuplicateWalletRecordException()
    {
        super(message, ErrorCode.DUPLICATE_WALLET_RECORD.value());
    }
}