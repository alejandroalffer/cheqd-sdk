package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class CreateWalletBackupException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Could not create WalletBackup";


    public CreateWalletBackupException()
    {
        super(message, ErrorCode.CREATE_WALLET_BACKUP.value());
    }
}