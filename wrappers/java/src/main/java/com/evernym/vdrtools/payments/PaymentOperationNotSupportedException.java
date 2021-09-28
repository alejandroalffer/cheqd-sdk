package com.evernym.vdrtools.payments;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

public class PaymentOperationNotSupportedException extends IndyException {
	private static final long serialVersionUID = - 5009466707967765943L;
	private static final String message = "Operation is not supported for payment method";

	/**
	 * Initializes a new {@link PaymentOperationNotSupportedException} with the specified message.
	 */
	public PaymentOperationNotSupportedException() {
		super(message, ErrorCode.PaymentOperationNotSupportedError.value());
	}
}
