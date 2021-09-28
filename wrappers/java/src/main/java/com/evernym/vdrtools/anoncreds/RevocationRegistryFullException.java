package com.evernym.vdrtools.anoncreds;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when a revocation registry is full.
 */
public class RevocationRegistryFullException extends IndyException
{
	private static final long serialVersionUID = 8294079007838985455L;
	private final static String message = "The specified revocation registry is full.  Another revocation registry must be created.";

	/**
	 * Initializes a new RevocationRegistryFullException.
	 */
	public RevocationRegistryFullException()
	{
		super(message, ErrorCode.AnoncredsRevocationRegistryFullError.value());
	}
}