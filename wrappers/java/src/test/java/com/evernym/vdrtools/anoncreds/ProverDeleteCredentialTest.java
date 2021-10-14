package com.evernym.vdrtools.anoncreds;


import com.evernym.vdrtools.wallet.WalletItemNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

public class ProverDeleteCredentialTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverDeleteCredentialWorks() throws Exception {
		Anoncreds.proverDeleteCredential(wallet, credentialIdX).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Anoncreds.proverGetCredential(wallet, credentialIdX).get();  // make sure it's gone

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Anoncreds.proverDeleteCredential(wallet, credentialIdX).get();  // exercise double deletion
	}
}
