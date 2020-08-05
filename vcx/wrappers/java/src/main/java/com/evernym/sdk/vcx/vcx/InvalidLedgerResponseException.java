package com.evernym.sdk.vcx.vcx;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class InvalidLedgerResponseException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Invalid response from ledger for paid transaction";


    public InvalidLedgerResponseException()
    {
        super(message, ErrorCode.INVALID_LEDGER_RESPONSE.value());

    }
}