package com.evernym.vdrtools.error;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.InvalidParameterException;
import com.evernym.vdrtools.InvalidStructureException;
import com.evernym.vdrtools.crypto.Crypto;
import com.evernym.vdrtools.crypto.CryptoJSONParameters;
import org.junit.Assert;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;

public class GetErrorTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testErrors() throws Exception {
		try {
			String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter("invalidSeedLength", null).toJson();
			Crypto.createKey(this.wallet, paramJson).get();
		} catch (ExecutionException e) {
			InvalidStructureException ex = (InvalidStructureException) e.getCause();
			Assert.assertEquals(ex.getSdkErrorCode(), ErrorCode.CommonInvalidStructure.value());
			assertFalse(ex.getMessage().isEmpty());
		}

		try {
			byte[] message = {};
			Crypto.cryptoSign(this.wallet, VERKEY, message).get();
		} catch (ExecutionException e) {
			InvalidParameterException ex = (InvalidParameterException) e.getCause();
			assertEquals(ex.getSdkErrorCode(), ErrorCode.CommonInvalidParam5.value());
			assertFalse(ex.getMessage().isEmpty());
		}
	}
}
