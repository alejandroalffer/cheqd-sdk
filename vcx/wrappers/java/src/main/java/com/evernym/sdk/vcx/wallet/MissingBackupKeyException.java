package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class MissingBackupKeyException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Missing exported backup key in config";


    public MissingBackupKeyException()
    {
        super(message, ErrorCode.MISSING_BACKUP_KEY.value());
    }
}