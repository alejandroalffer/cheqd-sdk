package com.evernym.vdrtools.payments;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

public class ExtraFundsException extends IndyException {
    private static final long serialVersionUID = 6397499268992083529L;
    private static final String message = "Extra funds on inputs";

    /**
     * Initializes a new {@link ExtraFundsException} with the specified message.
     */
    public ExtraFundsException() {
        super(message, ErrorCode.ExtraFundsError.value());
    }
}
