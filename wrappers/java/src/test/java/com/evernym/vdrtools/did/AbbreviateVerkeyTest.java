package com.evernym.vdrtools.did;

import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;

public class AbbreviateVerkeyTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testAbbrVerkeyWorksForAbbrVerkey() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();

		String verkey = Did.AbbreviateVerkey(result.getDid(), result.getVerkey()).get();

		assertNotEquals(result.getVerkey(), verkey);
	}

	@Test
	public void testAbbrVerkeyWorksForNotAbbrVerkey() throws Exception {
		DidJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new DidJSONParameters.CreateAndStoreMyDidJSONParameter(DID_TRUSTEE, null, null, null);

		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, theirDidJson.toJson()).get();

		String verkey = Did.AbbreviateVerkey(result.getDid(), result.getVerkey()).get();

		assertEquals(result.getVerkey(), verkey);
	}
}
