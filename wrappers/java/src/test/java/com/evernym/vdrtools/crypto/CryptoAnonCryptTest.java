package com.evernym.vdrtools.crypto;

import com.evernym.vdrtools.IndyIntegrationTest;
import org.junit.Test;

import static org.junit.Assert.assertNotNull;

public class CryptoAnonCryptTest extends IndyIntegrationTest {

	@Test
	public void testPrepAnonymousMsgWorks() throws Exception {
		byte[] encryptedMsg = Crypto.anonCrypt(VERKEY_MY1, MESSAGE).get();
		assertNotNull(encryptedMsg);
	}
}