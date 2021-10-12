package com.evernym.vdrtools.payments;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

public class PaymentSourceDoesNotExistException extends IndyException {
    private static final long serialVersionUID = -5009466707967765943L;
    private static final String message = "No such source found";

    /**
     * Initializes a new {@link PaymentSourceDoesNotExistException} with the specified message.
     */
    public PaymentSourceDoesNotExistException() {
        super(message, ErrorCode.PaymentSourceDoesNotExistError.value());
    }
}
