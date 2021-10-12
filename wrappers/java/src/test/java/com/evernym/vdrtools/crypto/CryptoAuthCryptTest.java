package com.evernym.vdrtools.crypto;

import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.InvalidStructureException;
import com.evernym.vdrtools.wallet.WalletItemNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.*;


public class CryptoAuthCryptTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testAuthCryptWorksForCreatedKey() throws Exception {
		String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

		byte[] encryptedMsg = Crypto.authCrypt(wallet, myVk, VERKEY_MY2, MESSAGE).get();
		assertNotNull(encryptedMsg);
	}

	@Test
	public void testAuthCryptWorksForUnknownSenderVerkey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Crypto.authCrypt(wallet, VERKEY, VERKEY_MY2, MESSAGE).get();
	}

	@Test
	public void testAuthCryptWorksForInvalidTheirVk() throws Exception {
		String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Crypto.authCrypt(wallet, myVk, INVALID_VERKEY, MESSAGE).get();
	}
}