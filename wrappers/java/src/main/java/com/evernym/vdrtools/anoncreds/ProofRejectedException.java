package com.evernym.vdrtools.anoncreds;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when a proof has been rejected.
 */
public class ProofRejectedException extends IndyException
{
	private static final long serialVersionUID = -5100028213117687183L;
	private final static String message = "The proof has been rejected.";

	/**
	 * Initializes a new ProofRejectionException.
	 */
	public ProofRejectedException()
	{
		super(message, ErrorCode.AnoncredsProofRejected.value());
	}
}