package com.evernym.vdrtools.pairwise;

import com.evernym.vdrtools.wallet.WalletItemNotFoundException;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class GetPairwiseTest extends PairwiseIntegrationTest {

	@Test
	public void testGetPairwiseWorks() throws Exception {
		Pairwise.createPairwise(wallet, theirDid, myDid, metadata).get();

		String pairwiseInfoJson = Pairwise.getPairwise(wallet, theirDid).get();
		JSONObject pairwiseInfo = new JSONObject(pairwiseInfoJson);

		assertEquals(myDid, pairwiseInfo.getString("my_did"));
		assertEquals(metadata, pairwiseInfo.getString("metadata"));
	}

	@Test
	public void testGetPairwiseWorksForNotCreated() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Pairwise.getPairwise(wallet, theirDid).get();
	}
}
