package com.evernym.vdrtools.ledger;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when attempt to parse invalid transaction response.
 */
public class LedgerInvalidTransactionException extends IndyException
{
	private static final long serialVersionUID = -6503578332467229584L;
	private final static String message = "No consensus was reached during the ledger operation.";

	/**
	 * Initializes a new LedgerInvalidTransactionException.
	 */
	public LedgerInvalidTransactionException()
	{
		super(message, ErrorCode.LedgerInvalidTransaction.value());
	}
}