package com.evernym.vdrtools.crypto;

import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.wallet.WalletItemNotFoundException;
import org.junit.Test;

import java.util.Arrays;
import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import com.evernym.vdrtools.crypto.CryptoResults.AuthDecryptResult;


public class CryptoAuthDecryptTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testAuthDecryptWorks() throws Exception {
		String theirVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY2_SEED, null).toJson();
		String myVk = Crypto.createKey(wallet, paramJson).get();

		byte[] encryptedMsg = Crypto.authCrypt(wallet, theirVk, myVk, MESSAGE).get();
		AuthDecryptResult decryptResult = Crypto.authDecrypt(wallet, myVk, encryptedMsg).get();
		assertEquals(theirVk, decryptResult.getVerkey());
		assertTrue(Arrays.equals(MESSAGE, decryptResult.getDecryptedMessage()));
	}

	@Test
	public void testAuthDecryptWorksForUnknownTheirVk() throws Exception {
		String theirVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();

		byte[] encryptedMsg = Crypto.authCrypt(wallet, theirVk, VERKEY, MESSAGE).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Crypto.authDecrypt(wallet, VERKEY, encryptedMsg).get();
	}
}