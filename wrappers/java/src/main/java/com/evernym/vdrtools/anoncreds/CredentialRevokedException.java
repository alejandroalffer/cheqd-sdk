package com.evernym.vdrtools.anoncreds;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when attempting to use a credential that has been revoked.
 */
public class CredentialRevokedException extends IndyException
{
	private static final long serialVersionUID = 8269746965241515882L;
	private final static String message = "The credential has been revoked.";

	/**
	 * Initializes a new CredentialRevokedException.
	 */
	public CredentialRevokedException() {
		super(message, ErrorCode.AnoncredsCredentialRevoked.value());
	}
}