package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown occurred during wallet operation.
 */
public class WalletStorageException extends IndyException
{
	private static final long serialVersionUID = 1829076830401150667L;
	private final static String message = "Storage error occurred during wallet operation.";

	/**
	 * Initializes a new WalletStorageException.
	 */
	public WalletStorageException()
	{
		super(message, ErrorCode.WalletStorageError.value());
	}
}