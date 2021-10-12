package com.evernym.vdrtools.ledger;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when consensus was not reached during a ledger operation.
 */
public class ConsensusException extends IndyException
{
	private static final long serialVersionUID = -6503578332467229584L;
	private final static String message = "No consensus was reached during the ledger operation.";

	/**
	 * Initializes a new ConsensusException.
	 */
	public ConsensusException()
	{
		super(message, ErrorCode.LedgerNoConsensusError.value());
	}
}