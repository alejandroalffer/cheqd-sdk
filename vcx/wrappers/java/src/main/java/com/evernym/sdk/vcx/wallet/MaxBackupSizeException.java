package com.evernym.sdk.vcx.wallet;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

public class MaxBackupSizeException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Cloud Backup exceeds max size limit";


    public MaxBackupSizeException()
    {
        super(message, ErrorCode.MAX_BACKUP_SIZE.value());
    }
}