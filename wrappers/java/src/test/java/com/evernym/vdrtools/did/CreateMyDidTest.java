package com.evernym.vdrtools.did;

import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.InvalidStructureException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

import org.bitcoinj.core.Base58;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class CreateMyDidTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testCreateMyDidWorksForEmptyJson() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();

		assertEquals(16, Base58.decode(result.getDid()).length);
		assertEquals(32, Base58.decode(result.getVerkey()).length);
	}

	@Test
	public void testCreateMyDidWorksForSeed() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(this.wallet, MY1_IDENTITY_JSON).get();

		assertEquals(DID_MY1, result.getDid());
		assertEquals(VERKEY_MY1, result.getVerkey());
	}

	@Test
	public void testCreateMyDidWorksForInvalidSeed() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String didJson = new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, "aaaaaaaaaaa", null, null).toJson();

		Did.createAndStoreMyDid(this.wallet, didJson).get();
	}

	@Test
	public void testCreateMyDidWorksForDuplicate() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(this.wallet, "{}").get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(DidAlreadyExistsException.class));

		String didJson = new DidJSONParameters.CreateAndStoreMyDidJSONParameter(result.getDid(), null, null, null).toJson();
		Did.createAndStoreMyDid(this.wallet, didJson).get();
	}
}
