package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when provided wallet query is invalid.
 */
public class WalletInvalidQueryException extends IndyException
{
	private static final long serialVersionUID = 667964860056778208L;
	private final static String message = "Wallet query is invalid.";

	/**
	 * Initializes a new WalletInvalidQueryException.
	 */
	public WalletInvalidQueryException()
	{
		super(message, ErrorCode.WalletQueryError.value());
	}
}
