package com.evernym.vdrtools.did;

import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.wallet.WalletItemNotFoundException;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class GetDidMetadataTest extends IndyIntegrationTestWithSingleWallet {

	private String did;

	@Before
	public void createDid() throws Exception {
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();
		did = result.getDid();
	}

	@Test
	public void testGetDidMetadataWorks() throws Exception {
		Did.setDidMetadata(wallet, did, METADATA).get();
		String receivedMetadata = Did.getDidMetadata(wallet, did).get();
		assertEquals(METADATA, receivedMetadata);
	}

	@Test
	public void testGetDidMetadataWorksForNoMetadata() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Did.getDidMetadata(wallet, did).get();
	}
}