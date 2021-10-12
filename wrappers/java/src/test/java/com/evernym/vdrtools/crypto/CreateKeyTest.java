package com.evernym.vdrtools.crypto;

import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;


public class CreateKeyTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testCreateKeyWorksForSeed() throws Exception {
		String senderVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();
		assertNotNull(senderVk);
	}

	@Test
	public void testCreateKeyWorksWithoutSeed() throws Exception {
		String senderVk = Crypto.createKey(wallet, "{}").get();
		assertNotNull(senderVk);
	}

	@Test
	public void testCreateKeyWorksForInvalidSeed() throws Exception {
		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter("invalidSeedLength", null).toJson();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Crypto.createKey(wallet, paramJson).get();
	}
}
