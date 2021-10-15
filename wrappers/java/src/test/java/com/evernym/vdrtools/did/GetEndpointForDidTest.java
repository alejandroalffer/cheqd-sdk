package com.evernym.vdrtools.did;

import com.evernym.vdrtools.IndyIntegrationTestWithPoolAndSingleWallet;
import com.evernym.vdrtools.InvalidStateException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class GetEndpointForDidTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testGetEndpointForDidWorks() throws Exception {
		Did.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
		DidResults.EndpointForDidResult receivedEndpoint = Did.getEndpointForDid(wallet, pool, DID).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(VERKEY, receivedEndpoint.getTransportKey());
	}

	@Test
	public void testGetEndpointForDidWorksForUnknownDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStateException.class));

		Did.getEndpointForDid(wallet, pool, DID).get();
	}
}