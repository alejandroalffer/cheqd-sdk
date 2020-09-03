package com.evernym.sdk.vcx.vcx;

import com.evernym.sdk.vcx.ErrorCode;
import com.evernym.sdk.vcx.VcxException;

/**
 * Created by abdussami on 05/06/18.
 */

public class DidAlreadyExistsInWalletException extends VcxException
{
    private static final long serialVersionUID = 3294831240096535507L;
    private final static String message = "Attempted to add a DID to wallet when that DID already exists in wallet";


    public DidAlreadyExistsInWalletException()
    {
        super(message, ErrorCode.DID_ALREADY_EXISTS_IN_WALLET.value());
    }
}