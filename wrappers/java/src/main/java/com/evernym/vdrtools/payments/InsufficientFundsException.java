package com.evernym.vdrtools.payments;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

public class InsufficientFundsException extends IndyException {
    private static final long serialVersionUID = 6397499268992083528L;
    private static final String message = "Insufficient funds on inputs";

    /**
     * Initializes a new {@link InsufficientFundsException} with the specified message.
     */
    public InsufficientFundsException() {
        super(message, ErrorCode.InsufficientFundsError.value());
    }
}
