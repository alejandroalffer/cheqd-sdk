package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when attempting to register a custom wallet type that has already been registered.
 */
public class DuplicateWalletTypeException extends IndyException
{
	private static final long serialVersionUID = -5414881660233778407L;
	private final static String message = "A wallet type with the specified name has already been registered.";

	/**
	 * Initializes a new DuplicateWalletTypeException.
	 */
	public DuplicateWalletTypeException()
	{
		super(message, ErrorCode.WalletTypeAlreadyRegisteredError.value());
	}
}