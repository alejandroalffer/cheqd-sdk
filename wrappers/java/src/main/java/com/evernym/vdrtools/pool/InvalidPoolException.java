package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when attempting to use a poll that has already been closed.
 */
public class InvalidPoolException extends IndyException
{
	private static final long serialVersionUID = 7124250084655044699L;
	private final static String message = "The pool is closed or invalid and cannot be used.";

	/**
	 * Initializes a new PoolClosedException.
	 */
	public InvalidPoolException()
	{
		super(message, ErrorCode.PoolLedgerInvalidPoolHandle.value());
	}
}