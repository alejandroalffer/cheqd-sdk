package com.evernym.vdrtools.anoncreds;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when a invalid user revocation index is used.
 */
public class AnoncredsInvalidUserRevocId extends IndyException
{
	private static final long serialVersionUID = 4969718227042210813L;
	private final static String message = "The user revocation registry index specified is invalid.";

	/**
	 * Initializes a new AnoncredsInvalidUserRevocId.
	 */
	public AnoncredsInvalidUserRevocId()
	{
		super(message, ErrorCode.AnoncredsInvalidUserRevocId.value());
	}
}