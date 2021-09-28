package com.evernym.vdrtools.payments;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

public class TransactionNotAllowedException extends IndyException {
    private static final long serialVersionUID = 6397499268992083529L;
    private static final String message = "The transaction is not allowed to a requester";

    /**
     * Initializes a new {@link TransactionNotAllowedException} with the specified message.
     */
    public TransactionNotAllowedException() {
        super(message, ErrorCode.TransactionNotAllowedError.value());
    }
}
