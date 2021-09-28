package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when attempting to open Pool for witch Genesis Transactions are not compatible with set Protocol version.
 */
public class PoolIncompatibleProtocolVersionException extends IndyException
{
	private static final long serialVersionUID = 6945180938262170499L;
	private final static String message = "Pool Genesis Transactions are not compatible with Protocol version.";

	/**
	 * Initializes a new PoolIncompatibleProtocolVersionException.
	 */
	public PoolIncompatibleProtocolVersionException()
	{
		super(message, ErrorCode.PoolLedgerNotCreatedError.value());
	}
}