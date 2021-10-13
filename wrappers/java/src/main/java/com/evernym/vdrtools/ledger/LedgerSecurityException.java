package com.evernym.vdrtools.ledger;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when a transaction cannot be sent to to insufficient privileges.
 */
public class LedgerSecurityException extends IndyException
{
	private static final long serialVersionUID = 1695822815015877550L;
	private final static String message = "The transaction cannot be sent as the privileges for the current pool connection don't allow it.";

	/**
	 * Initializes a new LedgerSecurityException.
	 */
	public LedgerSecurityException()
	{
		super(message, ErrorCode.LedgerSecurityError.value());
	}
}