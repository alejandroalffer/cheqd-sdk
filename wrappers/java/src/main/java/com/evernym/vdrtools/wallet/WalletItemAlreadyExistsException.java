package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when add record operation is used with record name that already exists.
 */
public class WalletItemAlreadyExistsException extends IndyException
{
	private static final long serialVersionUID = 667964860056778208L;
	private final static String message = "Item already exists.";

	/**
	 * Initializes a new WalletItemNotFoundException.
	 */
	public WalletItemAlreadyExistsException() {
		super(message, ErrorCode.WalletItemAlreadyExists.value());
	}
}
