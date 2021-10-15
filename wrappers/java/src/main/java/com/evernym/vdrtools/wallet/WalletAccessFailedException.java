package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Attempt to open encrypted wallet with invalid credentials
 */
public class WalletAccessFailedException extends IndyException
{
	private static final long serialVersionUID = 3294831240096535507L;
	private final static String message = "The wallet security error.";

	/**
	 * Initializes a new WalletAccessFailedException.
	 */
	public WalletAccessFailedException() {
		super(message, ErrorCode.WalletAccessFailed.value());
	}
}