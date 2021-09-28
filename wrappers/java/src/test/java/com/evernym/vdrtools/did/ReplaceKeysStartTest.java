package com.evernym.vdrtools.did;

import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.wallet.WalletItemNotFoundException;
import org.bitcoinj.core.Base58;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;

import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class ReplaceKeysStartTest extends IndyIntegrationTestWithSingleWallet {

	private String did;
	private String verkey;

	@Before
	public void before() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(this.wallet, "{}").get();
		did = result.getDid();
		verkey = result.getVerkey();
	}

	@Test
	public void testReplaceKeysStartWorksForEmptyJson() throws Exception {
		String verkey = Did.replaceKeysStart(wallet, did, "{}").get();
		assertEquals(32, Base58.decode(verkey).length);
	}

	@Test
	public void testReplaceKeysStartWorksForNotExistsDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Did.replaceKeysStart(this.wallet, DID, "{}").get();
	}

	@Test
	public void testReplaceKeysStartWorksForSeed() throws Exception {
		String verkey = Did.replaceKeysStart(this.wallet, this.did, MY1_IDENTITY_KEY_JSON).get();

		assertEquals(VERKEY_MY1, verkey);
		assertNotEquals(this.verkey, verkey);
	}
}
