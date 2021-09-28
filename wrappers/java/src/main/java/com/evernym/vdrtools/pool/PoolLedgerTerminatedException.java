package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when a pool ledger has been terminated.
 */
public class PoolLedgerTerminatedException extends IndyException
{
	private static final long serialVersionUID = 768482152424714514L;
	private final static String message = "The pool ledger was terminated.";

	/**
	 * Initializes a new PoolLedgerTerminatedException.
	 */
	public PoolLedgerTerminatedException()
	{
		super(message, ErrorCode.PoolLedgerTerminated.value());
	}
}